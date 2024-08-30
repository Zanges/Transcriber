use global_hotkey::{
    hotkey::{HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{EventLoop, ControlFlow};
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

        match event {
            winit::event::Event::NewEvents(_) => {
                if let Ok(event) = global_hotkey_channel.try_recv() {
                    println!("Received hotkey event: {:?}", event);
                    if event.id == hotkey.id() {
                        println!("Hotkey F7 pressed!");
                        msgbox::create("Global Hotkey", "You pressed F7!", IconType::Info)
                            .expect("Failed to create message box");
                    }
                }
            }
            winit::event::Event::LoopDestroyed => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
