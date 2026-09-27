package com.oxide.client

import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.Response

class OxideTechClient(private val baseUrl: String) {
    private val client = OkHttpClient.Builder()
        .addInterceptor { chain ->
            val request = chain.request().newBuilder()
                .addHeader("X-API-Key", getApiKey())
                .addHeader("X-Tenant-ID", getTenantId())
                .build()
            chain.proceed(request)
        }
        .build()
    
    fun streamInference(
        prompt: String,
        projectName: String,
        model: String = "ornith-35b"
    ): Flow<String> = flow {
        val json = """
            {
                "messages": [{"role": "user", "content": "$prompt"}],
                "model": "$model",
                "stream": true
            }
        """.trimIndent()
        
        val request = Request.Builder()
            .url("$baseUrl/v1/inference/stream")
            .post(json.toRequestBody("application/json".toMediaType()))
            .build()
        
        val response = client.newCall(request).execute()
        response.body?.byteStream()?.bufferedReader()?.use { reader ->
            while (true) {
                val line = reader.readLine() ?: break
                if (line.startsWith("data: ")) {
                    val data = line.removePrefix("data: ").trim()
                    if (data == "[DONE]") break
                    emit(data)
                }
            }
        }
    }

    private fun getApiKey(): String {
        val envKey = System.getenv("GEMINI_API_KEY")
        if (!envKey.isNullOrEmpty()) return envKey
        
        val userHome = System.getProperty("user.home")
        val configPath = java.io.File(userHome, ".config/oxide_tech/credentials.json")
        if (configPath.exists()) {
            try {
                val content = configPath.readText()
                val regex = "\"gemini_api_key\"\\s*:\\s*\"([^\"]+)\"".toRegex()
                val match = regex.find(content)
                if (match != null) {
                    return match.groupValues[1]
                }
            } catch (e: Exception) {
                // Ignore
            }
        }
        return ""
    }

    private fun getTenantId(): String {
        val envTenant = System.getenv("OXIDE_TENANT_ID")
        if (!envTenant.isNullOrEmpty()) return envTenant
        
        val userHome = System.getProperty("user.home")
        val configPath = java.io.File(userHome, ".config/oxide_tech/credentials.json")
        if (configPath.exists()) {
            try {
                val content = configPath.readText()
                val regex = "\"tenant_id\"\\s*:\\s*\"([^\"]+)\"".toRegex()
                val match = regex.find(content)
                if (match != null) {
                    return match.groupValues[1]
                }
            } catch (e: Exception) {
                // Ignore
            }
        }
        return "00000000-0000-0000-0000-000000000000"
    }
}
