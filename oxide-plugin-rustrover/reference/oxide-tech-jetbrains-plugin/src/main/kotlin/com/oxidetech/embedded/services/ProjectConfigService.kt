package com.oxidetech.embedded.services

import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project

@Service(Service.Level.PROJECT)
class ProjectConfigService(private val project: Project) {
    companion object {
        fun getInstance(project: Project): ProjectConfigService = project.getService(ProjectConfigService::class.java)
    }

    fun getConfig(): ProjectConfig {
        return ProjectConfig(mcuType = "STM32H743VITx", maxPowerMW = 500)
    }
}
