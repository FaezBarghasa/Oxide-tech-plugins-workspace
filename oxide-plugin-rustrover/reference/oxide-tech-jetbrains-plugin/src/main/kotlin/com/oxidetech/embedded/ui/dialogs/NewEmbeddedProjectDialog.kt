package com.oxidetech.embedded.ui.dialogs

import com.intellij.openapi.ui.ComboBox
import com.intellij.openapi.ui.DialogWrapper
import com.oxidetech.embedded.services.BackendAPIService
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import javax.swing.DefaultComboBoxModel
import javax.swing.JComponent
import javax.swing.JPanel
import java.awt.BorderLayout

class NewEmbeddedProjectDialog : DialogWrapper(true) {
    private val mcuComboBox = ComboBox<String>()
    private val templateComboBox = ComboBox<String>()
    
    init {
        title = "New Embedded Rust Project"
        
        // Populate MCU list from datasheet database
        val mcuList = listOf(
            "STM32H743VITx (STM32H7, Cortex-M7, 480MHz, 1MB Flash)",
            "ESP32-C6-WROOM (RISC-V, 160MHz, WiFi+BLE)",
            "Raspberry Pi Pico (ARM Cortex-M0+, 133MHz)"
        )
        mcuComboBox.model = DefaultComboBoxModel(mcuList.toTypedArray())
        
        templateComboBox.model = DefaultComboBoxModel(arrayOf(
            "Blank (Embassy)",
            "Blank (RTIC v2)",
            "Blinky LED (Embassy)",
            "Timer Interrupt (RTIC)"
        ))
        init()
    }
    
    override fun createCenterPanel(): JComponent {
        val panel = JPanel(BorderLayout())
        panel.add(mcuComboBox, BorderLayout.NORTH)
        panel.add(templateComboBox, BorderLayout.SOUTH)
        return panel
    }
    
    override fun doOKAction() {
        val selectedMCU = mcuComboBox.selectedItem as? String ?: return
        val selectedTemplate = templateComboBox.selectedItem as? String ?: return
        
        // In real environment, fetch project creation here
        val mcuShort = selectedMCU.split(" ")[0]
        val templateShort = selectedTemplate.split(" ")[0]
        
        println("Generating Embedded Project for \$mcuShort using \$templateShort")
        
        super.doOKAction()
    }
}
