package com.oxidetech.embedded.services

import com.intellij.openapi.components.Service
import com.intellij.openapi.project.Project

@Service(Service.Level.PROJECT)
class DatasheetService(private val project: Project) {
    companion object {
        fun getInstance(project: Project): DatasheetService = project.getService(DatasheetService::class.java)
    }

    fun getRegistersForMCU(mcuType: String): List<RegisterInfo> {
        return listOf(
            RegisterInfo("GPIOA_MODER", "0x40020000", "GPIO port mode register", listOf(BitField("MODE0", "1:0", "Port x configuration bits (y = 0..15)")), "RW"),
            RegisterInfo("RCC_AHB1ENR", "0x40023830", "RCC AHB1 peripheral clock register", listOf(), "RW"),
            RegisterInfo("USART1_CR1", "0x40011000", "Control register 1", listOf(), "RW")
        )
    }

    fun getRegisterInfo(mcuType: String, text: String): RegisterInfo? {
        return getRegistersForMCU(mcuType).find { it.name == text }
    }
}
