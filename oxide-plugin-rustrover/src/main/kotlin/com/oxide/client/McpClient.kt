package com.oxide.client

import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody

class McpClient(private val baseUrl: String) {
    private val client = OkHttpClient()

    fun callTool(toolName: String, args: Map<String, String>): String {
        val argsJson = args.entries.joinToString(",") { (k, v) -> "\"$k\":\"$v\"" }
        val jsonPayload = "{\"name\":\"$toolName\",\"arguments\":{$argsJson}}"

        val request = Request.Builder()
            .url("$baseUrl/api/mcp/call")
            .post(jsonPayload.toRequestBody("application/json".toMediaType()))
            .build()

        client.newCall(request).execute().use { response ->
            if (!response.isSuccessful) {
                throw Exception("Server returned HTTP ${response.code}")
            }
            return response.body?.string() ?: ""
        }
    }

    fun listTools(): List<String> {
        val request = Request.Builder()
            .url("$baseUrl/api/mcp/list")
            .get()
            .build()
        try {
            client.newCall(request).execute().use { response ->
                if (!response.isSuccessful) {
                    return getFallbackTools()
                }
                val body = response.body?.string() ?: ""
                // Simple parsing to extract tool names from JSON list if available
                if (body.contains("[")) {
                    val cleaned = body.replace("[", "").replace("]", "").replace("\"", "")
                    return cleaned.split(",").map { it.trim() }.filter { it.isNotEmpty() }
                }
                return getFallbackTools()
            }
        } catch (e: Exception) {
            return getFallbackTools()
        }
    }

    private fun getFallbackTools(): List<String> {
        return listOf(
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
}
