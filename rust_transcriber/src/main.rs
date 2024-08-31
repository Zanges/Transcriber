mod hotkey_handler;
mod config_handler;
mod record_audio;
mod openai_transcribe;
mod output_handler;
mod main_gui;

use config_handler::Config;
use main_gui::run_gui;
use record_audio::AudioRecorder;
use openai_transcribe::OpenAITranscriber;
use hotkey_handler::HotkeyHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    
    let audio_recorder = AudioRecorder::new(&config)?;
    let openai_transcriber = OpenAITranscriber::new(&config.openai_api_key)?;
    let hotkey_handler = HotkeyHandler::new(&config.hotkey)?;

    // Run the GUI
    run_gui(config, audio_recorder, openai_transcriber, hotkey_handler)
}
mod config_gui;
