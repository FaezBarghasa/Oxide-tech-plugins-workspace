package com.oxidetech.embedded.codeCompletion

import com.intellij.codeInsight.completion.*
import com.intellij.codeInsight.lookup.LookupElementBuilder
import com.intellij.patterns.PlatformPatterns
import com.intellij.util.ProcessingContext
import com.oxidetech.embedded.services.DatasheetService
import com.oxidetech.embedded.services.ProjectConfigService
import javax.swing.Icon

// Mock Icon
object Icons {
    val REGISTER: Icon? = null
}

class RegisterMnemonicCompletion : CompletionContributor() {
    init {
        extend(CompletionType.BASIC, PlatformPatterns.psiElement(), 
            object : CompletionProvider<CompletionParameters>() {
                override fun addCompletions(
                    parameters: CompletionParameters,
                    context: ProcessingContext,
                    result: CompletionResultSet
                ) {
                    fillCompletionVariants(parameters, result)
                }
            }
        )
    }

    override fun fillCompletionVariants(parameters: CompletionParameters, result: CompletionResultSet) {
        val position = parameters.position
        val element = position.parent
        
        // Simplified context validation
        val isRegisterContext = true // PsiUtil checks would go here
        
        if (!isRegisterContext) return
        
        val project = parameters.editor.project ?: return
        val configService = ProjectConfigService.getInstance(project)
        val mcuType = configService.getConfig().mcuType ?: return // e.g., "STM32H743VITx"
        
        // Fetch register list from embedded datasheet database
        val registers = DatasheetService.getInstance(project)
            .getRegistersForMCU(mcuType)
            .filter { it.name.startsWith(parameters.position.text, ignoreCase = true) }
        
        registers.forEach { register ->
            val builder = LookupElementBuilder
                .create(register.name)
                .withIcon(Icons.REGISTER)
                .withTypeText("u32", true)
                .withTailText(" @ \${register.address}", true)
                .withInsertHandler { context, _ ->
                    context.document.insertString(
                        context.selectionEndOffset,
                        " /* \${register.description} */"
                    )
                }
            result.addElement(builder)
        }
    }
}
