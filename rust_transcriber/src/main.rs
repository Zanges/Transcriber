use global_hotkey::{
    hotkey::{HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent, DeviceEvent, ElementState, VirtualKeyCode};
use msgbox::IconType;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(None, global_hotkey::hotkey::Code::F7);
    manager.register(hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    println!("Press F7 to trigger the global hotkey. Press Escape to exit.");
    println!("Debugging: Global hotkey registered: {:?}", hotkey);

    let mut last_event_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let now = Instant::now();
        println!("Received event after {:?}: {:?}", now.duration_since(last_event_time), event);
        last_event_time = now;

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
            Event::WindowEvent { 
                event: WindowEvent::KeyboardInput { 
                    input,
                    ..
                },
                ..
            } => {
                println!("Window keyboard input: {:?}", input);
                if let Some(keycode) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        match keycode {
                            VirtualKeyCode::F7 => {
                                println!("F7 key pressed directly (window event)!");
                                msgbox::create("Direct Key Press", "You pressed F7 directly (window event)!", IconType::Info)
                                    .expect("Failed to create message box");
                            }
                            VirtualKeyCode::Escape => {
                                println!("Escape key pressed. Exiting...");
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => println!("Other key pressed: {:?}", keycode),
                        }
                    }
                }
            }
            Event::DeviceEvent { 
                event: DeviceEvent::Key(input),
                ..
            } => {
                println!("Device key event: {:?}", input);
                if let Some(keycode) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        match keycode {
                            VirtualKeyCode::F7 => {
                                println!("F7 key pressed (device event)!");
                                msgbox::create("Device Key Press", "You pressed F7 (device event)!", IconType::Info)
                                    .expect("Failed to create message box");
                            }
                            VirtualKeyCode::Escape => {
                                println!("Escape key pressed (device event). Exiting...");
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => println!("Other key pressed (device event): {:?}", keycode),
                        }
                    }
                }
            }
            Event::LoopDestroyed => {
                println!("Event loop is being destroyed.");
            }
            _ => {}
        }
    });
}
