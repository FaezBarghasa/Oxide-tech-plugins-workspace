package com.oxide.mcp

import com.intellij.openapi.ui.DialogWrapper
import com.oxide.client.McpClient
import javax.swing.*
import java.awt.Component

class McpToolPanel : DialogWrapper(true) {
    private val mcpClient = McpClient("https://llm.oxide-tech.com")
    
    init {
        title = "Oxide-Tech MCP Tools"
        init()
    }
    
    override fun createCenterPanel(): JComponent {
        val panel = JPanel()
        panel.layout = BoxLayout(panel, BoxLayout.Y_AXIS)
        
        // Dynamically fetch all tools from client
        val tools = try {
            mcpClient.listTools()
        } catch (e: Exception) {
            listOf(
                "probe_rs_flash",
                "run_rust_code",
                "rust_skills",
                "bacon",
                "cargo_nextest",
                "cargo_geiger",
                "cargo_audit",
                "tokless",
                "rtk",
                "ponytail",
                "renode_mcp",
                "web_search"
            )
        }

        for (tool in tools) {
            val btnName = tool.replace("tools/", "").replace("_", " ").replace("-", " ").split(" ").joinToString(" ") { it.capitalize() }
            val btn = JButton(btnName)
            btn.alignmentX = Component.CENTER_ALIGNMENT
            btn.addActionListener {
                SwingUtilities.invokeLater {
                    // Safety Guard: human confirmation for write/modify/execute operations
                    val confirm = JOptionPane.showConfirmDialog(
                        panel,
                        "Are you sure you want to run the tool '$tool'?",
                        "Confirm Tool Execution",
                        JOptionPane.YES_NO_OPTION,
                        JOptionPane.QUESTION_MESSAGE
                    )
                    
                    if (confirm == JOptionPane.YES_OPTION) {
                        runCatching {
                            // Determine args based on tool
                            val args = when (tool) {
                                "probe_rs_flash" -> mapOf("elf_path" to getCurrentElfPath())
                                "run_rust_code" -> mapOf("code" to "fn main() { println!(\"Hello\"); }")
                                "rust_skills" -> mapOf("action" to "list")
                                "bacon" -> mapOf("command" to "check")
                                "cargo_nextest" -> mapOf("args" to "")
                                "cargo_geiger" -> mapOf("path" to ".")
                                "cargo_audit" -> mapOf("ignore" to "")
                                "tokless" -> mapOf("action" to "info")
                                "rtk" -> mapOf("cmd" to "status")
                                "ponytail" -> mapOf("intensity" to "full")
                                "renode_mcp" -> mapOf("board" to "stm32f4")
                                "web_search" -> {
                                    val query = JOptionPane.showInputDialog(panel, "Enter search query:", "Web Search", JOptionPane.QUESTION_MESSAGE) ?: ""
                                    mapOf("query" to query)
                                }
                                else -> emptyMap()
                            }
                            mcpClient.callTool(tool, args)
                        }.onSuccess {
                            JOptionPane.showMessageDialog(panel, "Tool '$tool' executed successfully!")
                        }.onFailure {
                            JOptionPane.showMessageDialog(panel, "Execution failed: ${it.message}")
                        }
                    }
                }
            }
            panel.add(btn)
            panel.add(Box.createVerticalStrut(10))
        }
        
        return panel
    }

    private fun getCurrentElfPath(): String {
        return "target/thumbv7em-none-eabihf/release/firmware.elf"
    }
}
