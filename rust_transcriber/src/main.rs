mod hotkey_handler;
mod config_handler;
mod record_audio;
mod openai_transcribe;
mod output_handler;
mod main_gui;

use config_handler::Config;
use main_gui::run_gui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    // Run the GUI
    run_gui(config)
}
