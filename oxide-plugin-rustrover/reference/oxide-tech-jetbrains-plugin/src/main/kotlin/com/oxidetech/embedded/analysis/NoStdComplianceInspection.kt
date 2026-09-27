package com.oxidetech.embedded.analysis

import com.intellij.codeInspection.LocalInspectionTool
import com.intellij.codeInspection.ProblemHighlightType
import com.intellij.codeInspection.ProblemsHolder
import com.intellij.openapi.application.ReadAction
import com.intellij.psi.PsiElementVisitor
import com.intellij.psi.PsiFile
import com.intellij.util.concurrency.AppExecutorUtil
import com.oxidetech.embedded.services.BackendAPIService
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch

// Mock classes
abstract class RustElementVisitor : PsiElementVisitor() {
    open fun visitRustFile(file: PsiFile) {}
}

class NoStdComplianceInspection : LocalInspectionTool() {
    override fun buildVisitor(holder: ProblemsHolder, isOnTheFly: Boolean): PsiElementVisitor {
        return object : RustElementVisitor() {
            override fun visitRustFile(file: PsiFile) {
                // Launch coroutine on read action dispatcher
                ReadAction.nonBlocking<Unit> {
                    CoroutineScope(Dispatchers.Main).launch {
                        try {
                            val backendAPI = BackendAPIService.getInstance(file.project)
                            val ast = backendAPI.parseAST(file.name, file.text)
                            
                            // Mock tree checking for no_std
                            val hasNoStd = file.text.contains("#![no_std]")
                            if (!hasNoStd) {
                                holder.registerProblem(
                                    file,
                                    "File missing #![no_std] definition.",
                                    ProblemHighlightType.WARNING
                                )
                            }
                            
                            val forbidden = listOf("Vec", "String", "HashMap", "Box")
                            forbidden.forEach { keyword ->
                                if (file.text.contains(keyword)) {
                                    holder.registerProblem(
                                        file,
                                        "Allocation (\${keyword}) in no_std context. Use core::mem or defmt instead.",
                                        ProblemHighlightType.ERROR
                                    )
                                }
                            }
                            
                        } catch (e: Exception) {
                            // Ignored
                        }
                    }
                }.submit(AppExecutorUtil.getAppExecutorService())
            }
        }
    }
}
