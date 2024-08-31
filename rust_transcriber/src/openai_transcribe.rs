use reqwest::Client;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

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
        let mut file = File::open(audio_file_path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        let form = reqwest::multipart::Form::new()
            .text("model", "whisper-1")
            .part("file", reqwest::multipart::Part::bytes(buffer).file_name("audio.wav"));

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
