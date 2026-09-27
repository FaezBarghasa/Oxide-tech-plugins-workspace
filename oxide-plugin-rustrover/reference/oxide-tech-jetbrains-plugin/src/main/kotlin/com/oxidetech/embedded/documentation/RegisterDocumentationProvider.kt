package com.oxidetech.embedded.documentation

import com.intellij.lang.documentation.DocumentationProvider
import com.intellij.psi.PsiElement
import com.oxidetech.embedded.services.DatasheetService
import com.oxidetech.embedded.services.ProjectConfigService

class RegisterDocumentationProvider : DocumentationProvider {
    override fun generateDoc(element: PsiElement?, originalElement: PsiElement?): String? {
        val text = element?.text ?: return null
        
        // Extract MCU type from project config
        val project = element.project
        val configService = ProjectConfigService.getInstance(project)
        val mcuType = configService.getConfig().mcuType ?: return null
        
        // Check if this looks like a register name (uppercase_underscore pattern)
        if (!Regex("[A-Z0-9_]+").matches(text)) return null
        
        // Fetch datasheet from backend with caching
        val datasheet = DatasheetService.getInstance(project)
            .getRegisterInfo(mcuType, text)
            ?.let { registerInfo ->
                """
                <h3>\${registerInfo.name} @ \${registerInfo.address}</h3>
                <p>\${registerInfo.description}</p>
                <h4>Fields:</h4>
                <ul>
                    \${registerInfo.fields.joinToString("\n") { 
                        field -> "<li>\${field.name}[\${field.bitRange}]: \${field.description}</li>"
                    }}
                </ul>
                """.trimIndent()
            }
        
        return datasheet
    }
    
    override fun getQuickNavigateInfo(element: PsiElement?, originalElement: PsiElement?): String? {
        return generateDoc(element, originalElement)
    }
}
