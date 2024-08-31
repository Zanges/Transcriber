mod hotkey_handler;
mod config_handler;
mod record_audio;
mod openai_transcribe;

use winit::event_loop::EventLoop;
use hotkey_handler::HotkeyHandler;
use config_handler::Config;
use record_audio::AudioRecorder;
use openai_transcribe::OpenAITranscriber;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })?;

    let config = Config::load()?;
    println!("Loaded configuration:");
    println!("  Hotkey: {}", config.hotkey);
    println!("  Language: {}", config.language);

    let audio_recorder = Arc::new(Mutex::new(AudioRecorder::new()));
    let openai_transcriber = OpenAITranscriber::new(config.openai_api_key.clone());

    let event_loop = EventLoop::new();
    let hotkey_handler = HotkeyHandler::new(&config.hotkey, audio_recorder.clone(), Arc::new(openai_transcriber))?;

    hotkey_handler.handle_events(event_loop);

    Ok(())
}
