use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use winit::event_loop::EventLoop;

pub struct HotkeyHandler {
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
    global_hotkey_channel: crossbeam_channel::Receiver<GlobalHotKeyEvent>,
}

impl HotkeyHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = GlobalHotKeyManager::new()?;
        let hotkey = HotKey::new(None, global_hotkey::hotkey::Code::F7);
        manager.register(hotkey)?;

        let global_hotkey_channel = GlobalHotKeyEvent::receiver();

        Ok(Self {
            manager,
            hotkey,
            global_hotkey_channel: global_hotkey_channel.clone(),
        })
    }

    pub fn handle_events(&self, event_loop: &EventLoop<()>) {
        println!("Press F7 to trigger the global hotkey. Press Ctrl+C to exit.");

        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;

            if let winit::event::Event::NewEvents(_) = event {
                while let Ok(hotkey_event) = self.global_hotkey_channel.try_recv() {
                    if hotkey_event.id == self.hotkey.id() && hotkey_event.state == HotKeyState::Pressed {
                        println!("Global Hotkey: You pressed F7!");
                    }
                }
            }
        });
    }
}
