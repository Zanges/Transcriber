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
    
    let audio_recorder = Arc::new(Mutex::new(AudioRecorder::new(&config)));
    let openai_transcriber = Arc::new(OpenAITranscriber::new(config.openai_api_key.clone()));
    let output_handler = Arc::new(OutputHandler::new(config.word_delay, config.key_event_delay));
    let hotkey_handler = HotkeyHandler::new(&config.hotkey, audio_recorder.clone(), openai_transcriber.clone(), output_handler.clone())?;

    // Run the GUI
    Ok(run_gui(config)?)
}
mod config_gui;
