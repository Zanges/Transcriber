use global_hotkey::{
    hotkey::HotKey,
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent, DeviceEvent, ElementState, VirtualKeyCode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    })?;

    let event_loop = EventLoop::new();
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(None, global_hotkey::hotkey::Code::F7);
    manager.register(hotkey).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    println!("Press F7 to trigger the global hotkey. Press Ctrl+C to exit.");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if !running.load(Ordering::SeqCst) {
            *control_flow = ControlFlow::Exit;
            return;
        }

        match event {
            Event::NewEvents(_) => {
                while let Ok(hotkey_event) = global_hotkey_channel.try_recv() {
                    if hotkey_event.id == hotkey.id() && hotkey_event.state == HotKeyState::Pressed {
                        println!("Global Hotkey: You pressed F7!");
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
                                println!("Direct Key Press: You pressed F7 directly (window event)!");
                                return;
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
                                println!("Device Key Press: You pressed F7 (device event)!");
                                return;
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
