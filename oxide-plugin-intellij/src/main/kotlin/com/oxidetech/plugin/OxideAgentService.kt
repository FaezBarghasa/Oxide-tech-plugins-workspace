package com.oxidetech.plugin

import com.intellij.openapi.Disposable
import com.intellij.openapi.components.Service
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.fileEditor.FileEditorManager
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.Task
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import java.io.InputStreamReader

interface OxideCoreLib : Library {
    fun system_controller_new(width: Double, length: Double, wallThickness: Double, clearance: Double): Pointer?
    fun system_controller_dispatch_mcp_action(controller: Pointer, payloadJson: String): Int
    fun system_controller_destroy(controller: Pointer)
}

@Service(Service.Level.PROJECT)
class OxideAgentService(private val project: Project) : Disposable {

    private val logger = Logger.getInstance(OxideAgentService::class.java)
    private var nativeLibrary: OxideCoreLib? = null
    private var controllerPointer: Pointer? = null

    init {
        try {
            // "oxide_core" resolves to liboxide_core.so, oxide_core.dll, or liboxide_core.dylib depending on OS
            nativeLibrary = Native.load("oxide_core", OxideCoreLib::class.java)
            // Default dimension parameters for initialization
            controllerPointer = nativeLibrary?.system_controller_new(100.0, 100.0, 2.0, 1.5)
            logger.info("Successfully loaded UniFFI bindings and initialized SystemController.")
        } catch (e: Exception) {
            logger.error("Failed to load native Oxide Core library or initialize SystemController via JNA", e)
        }
    }

    fun auditWorkspaceFile() {
        val editorManager = FileEditorManager.getInstance(project)
        val activeFile = editorManager.selectedFiles.firstOrNull()
        
        if (activeFile == null) {
            logger.warn("No active file selected for compilation audit.")
            return
        }

        object : Task.Backgroundable(project, "Oxide-Tech: Compiling and Auditing File Context", true) {
            override fun run(indicator: ProgressIndicator) {
                indicator.isIndeterminate = true
                indicator.text = "Reading file content..."
                
                try {
                    val fileContent = readFileContent(activeFile)
                    
                    indicator.text = "Verifying safety parameters..."
                    var isNoStd = false
                    var hasAlloc = false
                    
                    val lines = fileContent.split("\n")
                    for (line in lines) {
                        val trimmed = line.trim()
                        if (trimmed.startsWith("#![no_std]")) {
                            isNoStd = true
                        }
                        if (trimmed.contains("extern crate alloc") || trimmed.contains("use alloc::")) {
                            hasAlloc = true
                        }
                    }

                    if (!isNoStd) {
                        logger.warn("Safety parameter verification failed: #![no_std] not found in \${activeFile.name}.")
                    }
                    if (hasAlloc) {
                        logger.warn("Heap allocation libraries detected in \${activeFile.name}. 'alloc' imports are flagged for strict memory limits.")
                    }

                    indicator.text = "Dispatching to native solver..."
                    val payload = """
                        {
                            "action": "verify_ast_context",
                            "file": "\${activeFile.path}",
                            "is_no_std": $isNoStd,
                            "has_alloc": $hasAlloc
                        }
                    """.trimIndent()

                    val currentPointer = controllerPointer
                    val lib = nativeLibrary
                    if (currentPointer != null && lib != null) {
                        val result = lib.system_controller_dispatch_mcp_action(currentPointer, payload)
                        logger.info("Native dispatch completed with status code: $result")
                    } else {
                        logger.error("Cannot dispatch MCP action. Native library or controller is not initialized.")
                    }

                } catch (e: Exception) {
                    logger.error("Error during background compilation audit task", e)
                }
            }
        }.queue()
    }

    private fun readFileContent(file: VirtualFile): String {
        return InputStreamReader(file.inputStream, "UTF-8").use { it.readText() }
    }

    override fun dispose() {
        try {
            val currentPointer = controllerPointer
            val lib = nativeLibrary
            if (currentPointer != null && lib != null) {
                lib.system_controller_destroy(currentPointer)
                logger.info("Successfully de-allocated native SystemController memory.")
            }
            controllerPointer = null
            nativeLibrary = null
        } catch (e: Exception) {
            logger.error("Error disposing native SystemController memory", e)
        }
    }
}
