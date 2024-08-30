use global_hotkey::{
    hotkey::{HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{EventLoop, ControlFlow};
use msgbox::IconType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::CONTROL), global_hotkey::hotkey::Code::Digit1);
    manager.register(hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    println!("Press Ctrl+1 to trigger the global hotkey. Press Ctrl+C to exit.");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = global_hotkey_channel.try_recv() {
            if event.id == hotkey.id() {
                msgbox::create("Global Hotkey", "You pressed Ctrl+1!", IconType::Info)
                    .expect("Failed to create message box");
            }
        }

        if let winit::event::Event::LoopDestroyed = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}
