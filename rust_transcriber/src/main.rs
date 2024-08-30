mod hotkey_handler;
mod config_handler;
mod record_audio;
mod openai_handler;

use winit::event_loop::EventLoop;
use hotkey_handler::HotkeyHandler;
use config_handler::Config;
use openai_handler::OpenAIHandler;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })?;

    let config = Config::load()?;
    println!("Loaded configuration:");
    println!("  Hotkey: {}", config.hotkey);
    println!("  Language: {}", config.language);
    println!("  OpenAI API Key: {}", if config.openai_api_key.is_empty() { "Not set" } else { "********" });

    let openai_handler = Arc::new(OpenAIHandler::new(config.openai_api_key));
    let event_loop = EventLoop::new();
    let hotkey_handler = HotkeyHandler::new(&config.hotkey, openai_handler)?;

    hotkey_handler.handle_events(event_loop);

    Ok(())
}
