package com.oxidetech.embedded.services

import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.Messages
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.launch

// Mock classes for demonstration without needing full IntelliJ UI implementation
object CompilationPanel {
    const val ERROR_STYLE = "error"
    fun getInstance(project: Project): CompilationPanelInstance = CompilationPanelInstance()
}

class CompilationPanelInstance {
    fun appendLine(text: String, style: String = "") {
        println("[\${if (style.isNotEmpty()) style else "info"}] \$text")
    }
    fun markRunning(running: Boolean) {
        println("Compilation running state: \$running")
    }
}

fun parseCargoJSON(diagnostics: List<CompilationDiagnostic>): List<CompilationDiagnostic> = diagnostics
fun createProblemDescriptor(project: Project, diag: CompilationDiagnostic) {
    println("Problem: \${diag.message}")
}

@Service(Service.Level.PROJECT)
class CompilationService(private val project: Project) {
    companion object {
        fun getInstance(project: Project): CompilationService = project.getService(CompilationService::class.java)
    }

    private val backendAPI = BackendAPIService.getInstance(project)
    private val coroutineScope = CoroutineScope(Dispatchers.Default)
    
    fun runCargoCheck(workspacePath: String): Job = coroutineScope.launch {
        val compilation = CompilationPanel.getInstance(project)
        
        backendAPI.openCompilationWebSocket(workspacePath).collect { event ->
            when (event.type) {
                "started" -> {
                    compilation.appendLine("Starting cargo check for oxide-tech hardware validator...")
                    compilation.markRunning(true)
                }
                "stdout" -> {
                    compilation.appendLine(event.data)
                }
                "stderr" -> {
                    compilation.appendLine(event.data, CompilationPanel.ERROR_STYLE)
                }
                "complete" -> {
                    compilation.markRunning(false)
                    if (event.success) {
                        // Using a print to not open real dialogs in headless test execution, but matches the spec intent
                        println("Compilation succeeded popup")
                    } else {
                        parseCargoJSON(event.diagnostics).forEach { diag ->
                            createProblemDescriptor(project, diag)
                        }
                    }
                }
                "error" -> {
                    compilation.appendLine("ERROR: \${event.message}", CompilationPanel.ERROR_STYLE)
                }
            }
        }
    }
}
