use global_hotkey::{
    hotkey::{HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::Event;
use msgbox::IconType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(None, global_hotkey::hotkey::Code::F7);
    manager.register(hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    println!("Press F7 to trigger the global hotkey. Press Ctrl+C to exit.");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        println!("Received event: {:?}", event);

        match event {
            Event::NewEvents(_) => {
                println!("Checking for hotkey events...");
                while let Ok(hotkey_event) = global_hotkey_channel.try_recv() {
                    println!("Received hotkey event: {:?}", hotkey_event);
                    if hotkey_event.id == hotkey.id() {
                        println!("Hotkey F7 pressed!");
                        msgbox::create("Global Hotkey", "You pressed F7!", IconType::Info)
                            .expect("Failed to create message box");
                    }
                }
            }
            Event::LoopDestroyed => {
                println!("Event loop is being destroyed.");
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
