mod hotkey_handler;

use winit::event_loop::EventLoop;
use hotkey_handler::HotkeyHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })?;

    let event_loop = EventLoop::new();
    let hotkey_handler = HotkeyHandler::new()?;

    hotkey_handler.handle_events(&event_loop);

    Ok(())
}
