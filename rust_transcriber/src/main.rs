use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::Event;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
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

        if !running.load(Ordering::SeqCst) {
            *control_flow = ControlFlow::Exit;
            return;
        }

        if let Event::NewEvents(_) = event {
            while let Ok(hotkey_event) = global_hotkey_channel.try_recv() {
                if hotkey_event.id == hotkey.id() && hotkey_event.state == HotKeyState::Pressed {
                    println!("Global Hotkey: You pressed F7!");
                }
            }
        }
    });
}
