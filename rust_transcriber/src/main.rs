use global_hotkey::{
    hotkey::HotKey,
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent, DeviceEvent, ElementState, VirtualKeyCode};
use msgbox::IconType;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Ctrl+C received, exiting...");
        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    })?;

    let event_loop = EventLoop::new();
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(None, global_hotkey::hotkey::Code::F7);
    manager.register(hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    println!("Press F7 to trigger the global hotkey. Press Ctrl+C to exit.");
    println!("Debugging: Global hotkey registered: {:?}", hotkey);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if !running.load(Ordering::SeqCst) {
            println!("Running flag is false. Exiting...");
            *control_flow = ControlFlow::Exit;
            return;
        }

        match event {
            Event::NewEvents(_) => {
                while let Ok(hotkey_event) = global_hotkey_channel.try_recv() {
                    println!("Received hotkey event: {:?}", hotkey_event);
                    if hotkey_event.id == hotkey.id() {
                        println!("Hotkey F7 pressed!");
                        msgbox::create("Global Hotkey", "You pressed F7!", IconType::Info)
                            .expect("Failed to create message box");
                    }
                }
            }
            Event::WindowEvent { 
                event: WindowEvent::KeyboardInput { 
                    input,
                    ..
                },
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        match keycode {
                            VirtualKeyCode::F7 => {
                                println!("F7 key pressed directly (window event)!");
                                msgbox::create("Direct Key Press", "You pressed F7 directly (window event)!", IconType::Info)
                                    .expect("Failed to create message box");
                            }
                            _ => {},
                        }
                    }
                }
            }
            Event::DeviceEvent { 
                event: DeviceEvent::Key(input),
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        match keycode {
                            VirtualKeyCode::F7 => {
                                println!("F7 key pressed (device event)!");
                                msgbox::create("Device Key Press", "You pressed F7 (device event)!", IconType::Info)
                                    .expect("Failed to create message box");
                            }
                            _ => {},
                        }
                    }
                }
            }
            _ => {}
        }
    });
}
