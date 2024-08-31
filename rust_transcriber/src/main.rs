mod hotkey_handler;
mod config_handler;
mod record_audio;
mod openai_transcribe;
mod output_handler;
mod main_gui;

use winit::event_loop::EventLoop;
use std::sync::{Arc, Mutex};
use config_handler::Config;
use output_handler::OutputHandler;
use main_gui::run_gui;
use record_audio::AudioRecorder;
use openai_transcribe::OpenAITranscriber;
use hotkey_handler::HotkeyHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    
    let audio_recorder = Arc::new(Mutex::new(AudioRecorder::new(&config)));
    let openai_transcriber = Arc::new(OpenAITranscriber::new(config.openai_api_key.clone()));
    let output_handler = Arc::new(OutputHandler::new(config.word_delay, config.key_event_delay));
    let hotkey_handler = HotkeyHandler::new(&config.hotkey, Some(audio_recorder.clone()), Some(openai_transcriber.clone()), Some(output_handler.clone()))?;

    let event_loop = EventLoop::new();
    hotkey_handler.handle_events(event_loop);

    // Run the GUI
    Ok(run_gui(config)?)
}
mod config_gui;
