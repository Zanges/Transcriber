use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::Event;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })?;

    let event_loop = EventLoop::new();
    let manager = GlobalHotKeyManager::new()?;
    let hotkey = HotKey::new(None, global_hotkey::hotkey::Code::F7);
    manager.register(hotkey)?;

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    println!("Press F7 to trigger the global hotkey. Press Ctrl+C to exit.");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Event::NewEvents(_) = event {
            while let Ok(hotkey_event) = global_hotkey_channel.try_recv() {
                if hotkey_event.id == hotkey.id() && hotkey_event.state == HotKeyState::Pressed {
                    println!("Global Hotkey: You pressed F7!");
                }
            }
        }
    });
}
