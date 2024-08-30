mod hotkey_handler;
mod config_handler;

use winit::event_loop::EventLoop;
use hotkey_handler::HotkeyHandler;
use config_handler::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })?;

    let config = Config::load()?;
    println!("Loaded configuration: {:?}", config);

    let event_loop = EventLoop::new();
    let hotkey_handler = HotkeyHandler::new(&config.hotkey)?;

    hotkey_handler.handle_events(event_loop);

    Ok(())
}
