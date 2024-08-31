use reqwest::Client;
use std::error::Error;
use std::path::Path;
use std::fs;

pub struct OpenAITranscriber {
    api_key: String,
    client: Client,
}

impl OpenAITranscriber {
    pub fn new(api_key: String) -> Self {
        OpenAITranscriber {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn transcribe(&self, audio_file_path: &str) -> Result<String, Box<dyn Error>> {
        let file_path = Path::new(audio_file_path);
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        let file_content = std::fs::read(audio_file_path)?;
        let file_part = reqwest::multipart::Part::bytes(file_content)
            .file_name(file_name.to_string())
            .mime_str("audio/wav")?;

        let form = reqwest::multipart::Form::new()
            .text("model", "whisper-1")
            .part("file", file_part);

        let response = self.client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            Ok(result["text"].as_str().unwrap_or("").to_string())
        } else {
            Err(format!("API request failed: {:?}", response.text().await?).into())
        }
    }
}
