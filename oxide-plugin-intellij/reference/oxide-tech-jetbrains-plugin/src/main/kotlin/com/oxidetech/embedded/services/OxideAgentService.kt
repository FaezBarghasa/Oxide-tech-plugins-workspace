package com.oxidetech.embedded.services

import com.intellij.credentialStore.CredentialAttributes
import com.intellij.credentialStore.generateServiceName
import com.intellij.ide.passwordSafe.PasswordSafe
import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project
import com.intellij.openapi.options.Configurable
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.notification.Notification
import com.intellij.notification.NotificationAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.options.ShowSettingsUtil
import com.intellij.openapi.diagnostic.Logger
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure
import javax.swing.*
import java.awt.GridLayout
import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL

// Logger declaration
private val LOG = Logger.getInstance("OxideAgentService")

// JNA physical dimension structure mapping
open class PhysicalDimension : Structure() {
    @JvmField var width: Double = 0.0
    @JvmField var length: Double = 0.0
    @JvmField var wall_thickness: Double = 0.0
    @JvmField var clearance: Double = 0.0

    override fun getFieldOrder(): List<String> {
        return listOf("width", "length", "wall_thickness", "clearance")
    }
}

// JNA Oxide Core DLL interface
interface OxideCoreLib : Library {
    companion object {
        val INSTANCE: OxideCoreLib = try {
            Native.load("oxide_core", OxideCoreLib::class.java)
        } catch (e: UnsatisfiedLinkError) {
            LOG.error("Failed to load native oxide_core library. Ensure liboxide_core is on PATH.", e)
            throw e
        }
    }

    fun system_controller_new(dimension: PhysicalDimension): Pointer
    fun system_controller_run_gjk_clearance_solver(controller: Pointer, designator: String, x: Double, y: Double, height: Double): Boolean
    fun system_controller_destroy(controller: Pointer)
}

// Credentials attribute helper
private val credentialAttributes = CredentialAttributes(
    generateServiceName("OxideTech", "GeminiApiKey")
)

@Service(Service.Level.PROJECT)
class OxideAgentService(private val project: Project) {

    companion object {
        fun getInstance(project: Project): OxideAgentService = project.getService(OxideAgentService::class.java)
    }

    /**
     * Spawns a background IntelliJ task to audit workspace files for no_std safety and clearance.
     */
    fun runBackgroundAudit(targetFile: VirtualFile) {
        val apiKey = PasswordSafe.instance.getPassword(credentialAttributes)
        
        if (apiKey.isNullOrEmpty()) {
            val notification = NotificationGroupManager.getInstance()
                .getNotificationGroup("Oxide Notifications")
                .createNotification(
                    "Gemini API Key Missing",
                    "Please configure your Gemini API Key in Settings to run Oxide-Tech audits.",
                    NotificationType.WARNING
                )
            
            notification.addAction(object : NotificationAction("Configure Key") {
                override fun actionPerformed(e: AnActionEvent, notification: Notification) {
                    ShowSettingsUtil.getInstance().showSettingsDialog(project, OxideConfigurable::class.java)
                    notification.expire()
                }
            })
            
            notification.notify(project)
            return
        }

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Oxide Workspace Audit", true) {
            override fun run(indicator: ProgressIndicator) {
                indicator.isIndeterminate = false
                indicator.fraction = 0.1
                
                // 1. AST Context Verification & check for #![no_std] safety
                indicator.text = "Checking AST safety boundaries..."
                val isSafe = auditFileForNoStd(targetFile)
                indicator.fraction = 0.5

                if (!isSafe) {
                    LOG.warn("Workspace safety audit failed for ${targetFile.name}.")
                    return
                }

                // 2. Load DLL using JNA and run clearance sweep
                indicator.text = "Running native physical collision sweep..."
                try {
                    val dim = PhysicalDimension().apply {
                        width = 100.0
                        length = 80.0
                        wall_thickness = 2.0
                        clearance = 1.5
                    }
                    
                    val controller = OxideCoreLib.INSTANCE.system_controller_new(dim)
                    // Sample collision checks for footprints
                    val collisionDetected = OxideCoreLib.INSTANCE.system_controller_run_gjk_clearance_solver(
                        controller, "U1", 10.0, 15.0, 4.0
                    )
                    
                    OxideCoreLib.INSTANCE.system_controller_destroy(controller)
                    indicator.fraction = 1.0

                    if (collisionDetected) {
                        LOG.warn("Physical clearance violations found.")
                    } else {
                        LOG.info("Native clearance sweep passed.")
                    }
                } catch (e: Exception) {
                    LOG.error("Failed to run native JNA clearance solver: ${e.message}", e)
                }
            }
        })
    }

    /**
     * Checks if standard library symbols or heap allocation libraries are referenced.
     */
    fun auditFileForNoStd(file: VirtualFile): Boolean {
        try {
            val content = String(file.contentsToByteArray())
            val hasNoStd = content.contains("#![no_std]")
            val usesAlloc = content.contains("use alloc::") || content.contains("extern crate alloc;")
            val usesStd = content.contains("use std::")

            if (!hasNoStd) {
                LOG.error("File ${file.name} lacks required '#![no_std]' attribute.")
                return false
            }
            if (usesAlloc || usesStd) {
                LOG.error("File ${file.name} imports heap-allocation or std crate.")
                return false
            }
            return true
        } catch (e: Exception) {
            LOG.error("Failed to read file contents for safety audit: ${e.message}")
            return false
        }
    }

    /**
     * Polls local agent orchestrator status to check if active in KiCad or Altium.
     */
    fun checkOrchestratorStatus(): String {
        var connection: HttpURLConnection? = null
        return try {
            val url = URL("http://127.0.0.1:8086/status")
            connection = url.openConnection() as HttpURLConnection
            connection.requestMethod = "GET"
            connection.connectTimeout = 3000
            connection.readTimeout = 3000

            val responseCode = connection.responseCode
            if (responseCode == HttpURLConnection.HTTP_OK) {
                val reader = BufferedReader(InputStreamReader(connection.inputStream))
                val response = reader.readText()
                reader.close()
                response
            } else {
                "Offline: HTTP Code $responseCode"
            }
        } catch (e: Exception) {
            "Offline: ${e.message}"
        } finally {
            connection?.disconnect()
        }
    }
}

/**
 * Settings UI mapping Configurable interface for IntelliJ Preference panel.
 */
class OxideConfigurable : Configurable {
    private var myPanel: JPanel? = null
    private val keyField = JPasswordField(30)

    override fun getDisplayName(): String = "Oxide-Tech Settings"

    override fun createComponent(): JComponent {
        val panel = JPanel(GridLayout(2, 2, 10, 10))
        panel.add(JLabel("Gemini API Key:"))
        panel.add(keyField)

        val savedKey = PasswordSafe.instance.getPassword(credentialAttributes)
        if (savedKey != null) {
            keyField.text = savedKey
        }

        myPanel = panel
        return panel
    }

    override fun isModified(): Boolean {
        val savedKey = PasswordSafe.instance.getPassword(credentialAttributes) ?: ""
        return String(keyField.password) != savedKey
    }

    override fun apply() {
        val key = String(keyField.password).trim()
        PasswordSafe.instance.setPassword(credentialAttributes, if (key.isEmpty()) null else key)
    }

    override fun reset() {
        val savedKey = PasswordSafe.instance.getPassword(credentialAttributes) ?: ""
        keyField.text = savedKey
    }

    override fun disposeUIResources() {
        myPanel = null
    }
}
