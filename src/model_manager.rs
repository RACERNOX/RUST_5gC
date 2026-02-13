use async_trait::async_trait;
use std::env;

/// The core trait that all LLM modules must implement.
/// This allows the rest of the application to treat Gemini and Local models identically.
#[async_trait]
pub trait LlmModule: Send + Sync {
    async fn chat(&self, prompt: &str) -> Result<String, String>;
}

/// Client for Google's Gemini API
pub struct GeminiClient {
    pub api_key: String,
    pub client: reqwest::Client,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmModule for GeminiClient {
    async fn chat(&self, prompt: &str) -> Result<String, String> {
        let model_name = std::env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-1.5-flash-latest".to_string());
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model_name, self.api_key
        );

        let body = serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        });

        let res = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(format!("API Error: {}", error_text));
        }

        let response_json: serde_json::Value = res.json().await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Extract text from Gemini response structure
        // Note: Real implementation needs robust error handling for missing fields
        let text = response_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("No response text found")
            .to_string();

        Ok(text)
    }
}

/// Client for Local LLM Server (e.g., Ollama)
pub struct LocalClient {
    pub url: String,
    pub client: reqwest::Client,
}

impl LocalClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmModule for LocalClient {
    async fn chat(&self, prompt: &str) -> Result<String, String> {
        let model_name = std::env::var("LOCAL_MODEL").unwrap_or_else(|_| "llama3".to_string());
        // Assuming standard Ollama /api/generate format
        let body = serde_json::json!({
            "model": model_name,
            "prompt": prompt,
            "stream": false
        });

        let res = self.client.post(&self.url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

         if !res.status().is_success() {
             return Err(format!("Local Server Error: {}", res.status()));
         }

        let response_json: serde_json::Value = res.json().await
             .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let text = response_json["response"]
            .as_str()
            .unwrap_or("No response text found")
            .to_string();

        Ok(text)
    }
}

/// Factory function to create the correct module based on environment variables
pub fn try_create_model() -> Result<Box<dyn LlmModule>, String> {
    let model_type = env::var("CURRENT_MODEL").unwrap_or_else(|_| "gemini".to_string());
    
    match model_type.as_str() {
        "local" => {
            let url = env::var("LOCAL_LLM_URL")
                .map_err(|_| "Configuration Error: LOCAL_LLM_URL must be set in .env")?;
            println!("🚀 Using LOCAL model at {}", url);
            Ok(Box::new(LocalClient::new(url)))
        }
        _ => {
            let api_key = env::var("GEMINI_API_KEY")
                .map_err(|_| "Configuration Error: GEMINI_API_KEY must be set in .env")?;
            println!("✨ Using GEMINI model");
            Ok(Box::new(GeminiClient::new(api_key)))
        }
    }
}
