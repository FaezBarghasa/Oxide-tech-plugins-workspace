package com.oxidetech.embedded.services

import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project
import io.ktor.client.*
import io.ktor.client.engine.okhttp.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.plugins.websocket.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

interface IOxideTechBackendAPI {
    suspend fun compileProject(workspacePath: String): Flow<CompilationEvent>
    suspend fun runCargoCheck(workspacePath: String): Flow<CompilationEvent>
    suspend fun parseAST(filePath: String, sourceCode: String): Any // Placeholder for ASTNode
    suspend fun getSymbolInfo(filePath: String, line: Int, column: Int): Any // Placeholder for SymbolInfo
    suspend fun getRegisterInfo(mcuType: String, registerName: String): RegisterInfo?
    suspend fun getDatasheetSnippet(mcuType: String, query: String): String?
    suspend fun validatePinAssignment(pin: String, usage: PinUsage): ValidationResult
    suspend fun validateHardwareConfig(config: ProjectConfig): ConfigValidationReport
    suspend fun generateHALStruct(mcuType: String, pins: List<String>): String
    suspend fun suggestAsyncPattern(code: String): CodeSuggestion?
    suspend fun queryComponentDependencies(componentRef: String): List<String>
    suspend fun queryNetConnections(netName: String): List<Connection>
}

@Serializable
data class CompilationDiagnostic(val message: String, val level: String)

@Serializable
data class CompilationEvent(
    val type: String, // "started", "stdout", "stderr", "complete", "error"
    val data: String = "",
    val success: Boolean = false,
    val diagnostics: List<CompilationDiagnostic> = emptyList(),
    val message: String = ""
)

@Serializable
data class BitField(val name: String, val bitRange: String, val description: String)

@Serializable
data class RegisterInfo(
    val name: String,
    val address: String,
    val description: String,
    val fields: List<BitField>,
    val accessType: String
)

@Serializable data class ASTParseRequest(val filePath: String, val sourceCode: String)
@Serializable data class PinUsage(val function: String)
@Serializable data class ValidationResult(val isValid: Boolean, val errors: List<String>)
@Serializable data class ProjectConfig(val mcuType: String?, val maxPowerMW: Int = 100)
@Serializable data class ConfigValidationReport(val successful: Boolean, val violations: List<String>)
@Serializable data class CodeSuggestion(val suggestion: String, val confidence: Double)
@Serializable data class Connection(val from: String, val to: String)

@Service(Service.Level.PROJECT)
class BackendAPIService(private val project: Project) : IOxideTechBackendAPI {
    companion object {
        fun getInstance(project: Project): BackendAPIService = project.getService(BackendAPIService::class.java)
    }

    private val httpClient = HttpClient(OkHttp) {
        install(ContentNegotiation) {
            json(Json { ignoreUnknownKeys = true })
        }
        install(WebSockets) {
            pingInterval = 20_000
        }
    }

    private val baseUrl = "http://127.0.0.1:8080/api"

    // Simulate flow logic for external compilation execution
    override suspend fun compileProject(workspacePath: String): Flow<CompilationEvent> = flow {
        emit(CompilationEvent(type = "started", data = "Compiling at $workspacePath"))
        try {
            // Ideally connects to a WebSocket here. Simulating.
            emit(CompilationEvent(type = "stdout", data = "cargo build --release"))
            emit(CompilationEvent(type = "complete", success = true))
        } catch (e: Exception) {
            emit(CompilationEvent(type = "error", message = e.message ?: "Compile failed"))
        }
    }

    override suspend fun runCargoCheck(workspacePath: String): Flow<CompilationEvent> = flow {
        emit(CompilationEvent(type = "started", data = "Checking at $workspacePath"))
        try {
            emit(CompilationEvent(type = "stdout", data = "cargo check"))
            emit(CompilationEvent(type = "complete", success = true))
        } catch (e: Exception) {
            emit(CompilationEvent(type = "error", message = e.message ?: "Check failed"))
        }
    }

    suspend fun openCompilationWebSocket(workspacePath: String): Flow<CompilationEvent> {
        return runCargoCheck(workspacePath)
    }

    override suspend fun parseAST(filePath: String, sourceCode: String): Any = withContext(Dispatchers.IO) {
        // Fallback or network call
        val request = ASTParseRequest(filePath, sourceCode)
        try {
            // httpClient.post("$baseUrl/tree-sitter/parse") { contentType(ContentType.Application.Json); setBody(request) }
            Any() 
        } catch (e: Exception) {
            Any()
        }
    }

    override suspend fun getSymbolInfo(filePath: String, line: Int, column: Int): Any = Any()

    override suspend fun getRegisterInfo(mcuType: String, registerName: String): RegisterInfo? = withContext(Dispatchers.IO) {
        try {
            // Model fetch call
            RegisterInfo(
                name = registerName,
                address = "0x40020000",
                description = "Simulated Register Info for $registerName on $mcuType",
                fields = listOf(BitField("EN", "0:0", "Enable bit")),
                accessType = "RW"
            )
        } catch (e: Exception) {
            null
        }
    }

    override suspend fun getDatasheetSnippet(mcuType: String, query: String): String? = "Simulated snippet."
    
    override suspend fun validatePinAssignment(pin: String, usage: PinUsage): ValidationResult = ValidationResult(true, emptyList())
    
    override suspend fun validateHardwareConfig(config: ProjectConfig): ConfigValidationReport = ConfigValidationReport(true, emptyList())
    
    override suspend fun generateHALStruct(mcuType: String, pins: List<String>): String = "pub struct HalStub {}"
    
    override suspend fun suggestAsyncPattern(code: String): CodeSuggestion? = CodeSuggestion("async fn optimized() {}", 0.95)
    
    override suspend fun queryComponentDependencies(componentRef: String): List<String> = emptyList()
    
    override suspend fun queryNetConnections(netName: String): List<Connection> = emptyList()
}
