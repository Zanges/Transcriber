mod hotkey_handler;
mod config_handler;
mod record_audio;
mod openai_transcribe;
mod output_handler;
mod main_gui;

use config_handler::Config;
use main_gui::run_gui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })?;

    let config = Config::load()?;

    // Run the GUI
    if let Err(e) = run_gui(config) {
        eprintln!("Error running GUI: {}", e);
    }

    Ok(())
}
