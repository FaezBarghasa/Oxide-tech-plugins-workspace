package com.oxidetech.embedded.analysis

import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project
import com.oxidetech.embedded.services.PinUsage
import com.oxidetech.embedded.services.ProjectConfig
import com.oxidetech.embedded.services.ProjectConfigService

enum class ErrorSeverity { WARNING, ERROR }

data class ConfigurationError(
    val message: String,
    val severity: ErrorSeverity,
    val sourceElement: Any? // Optional PsiElement binding
)

data class PinAssignment(
    val pinName: String,
    val function: String,
    val timerUnit: String?,
    val sourceElement: Any?
)

@Service(Service.Level.PROJECT)
class HardwareConstraintValidator(private val project: Project) {
    
    companion object {
        fun getInstance(project: Project): HardwareConstraintValidator = project.getService(HardwareConstraintValidator::class.java)
    }

    fun validateConfiguration(): List<ConfigurationError> {
        val configService = ProjectConfigService.getInstance(project)
        val config = configService.getConfig()
        val errors = mutableListOf<ConfigurationError>()
        
        val pinAssignments = extractPinAssignments(project)
        
        // 1. Check for pin conflicts
        pinAssignments.groupBy { it.pinName }.forEach { (pin, assignments) ->
            if (assignments.size > 1) {
                errors.add(ConfigurationError(
                    message = "Pin \$pin assigned to multiple functions: \${assignments.map { it.function }}",
                    severity = ErrorSeverity.ERROR,
                    sourceElement = assignments.first().sourceElement
                ))
            }
        }
        
        // 2. Check timer resource conflicts
        val timerAssignments = pinAssignments.filter { it.function.contains("Timer", ignoreCase = true) }
        timerAssignments.groupBy { it.timerUnit }.forEach { (timer, assignments) ->
            if (timer != null && assignments.size > 1) {
                errors.add(ConfigurationError(
                    message = "Timer \$timer used multiple times: \${assignments.map { it.pinName }}",
                    severity = ErrorSeverity.WARNING,
                    sourceElement = assignments.first().sourceElement
                ))
            }
        }
        
        // 3. Check power budget
        val powerEstimate = calculatePowerEstimate(pinAssignments, config)
        if (powerEstimate > config.maxPowerMW) {
            errors.add(ConfigurationError(
                message = "Power budget exceeded: \${powerEstimate}mW > \${config.maxPowerMW}mW",
                severity = ErrorSeverity.ERROR,
                sourceElement = null
            ))
        }
        
        return errors
    }

    // Mock extraction - in reality parses AST using Tree-Sitter
    private fun extractPinAssignments(project: Project): List<PinAssignment> {
        return listOf(
            PinAssignment("PA0", "GPIO_Input", null, null),
            PinAssignment("PA1", "Timer_PWM", "TIM2", null),
            PinAssignment("PA1", "SPI_MISO", null, null) // Conflict Example
        )
    }

    private fun calculatePowerEstimate(assignments: List<PinAssignment>, config: ProjectConfig): Int {
        // Mock estimate calc
        var total = 10 // Baseline
        assignments.forEach { 
            total += if (it.function.contains("PWM")) 5 else 2 
        }
        return total
    }
}
