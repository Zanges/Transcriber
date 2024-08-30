use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use winit::event_loop::EventLoop;

pub struct HotkeyHandler {
    #[allow(dead_code)]
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
    global_hotkey_channel: crossbeam_channel::Receiver<GlobalHotKeyEvent>,
}

impl HotkeyHandler {
    pub fn new(hotkey_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = GlobalHotKeyManager::new()?;
        let hotkey = HotKey::new(None, hotkey_str.parse()?);
        manager.register(hotkey)?;

        Ok(Self {
            manager,
            hotkey,
            global_hotkey_channel: GlobalHotKeyEvent::receiver().clone(),
        })
    }

    pub fn handle_events(self, event_loop: EventLoop<()>) {
        println!("Press {:?} to trigger the global hotkey. Press Ctrl+C to exit.", self.hotkey);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;

            if let winit::event::Event::NewEvents(_) = event {
                if let Ok(hotkey_event) = self.global_hotkey_channel.try_recv() {
                    if hotkey_event.id == self.hotkey.id() && hotkey_event.state == HotKeyState::Pressed {
                        println!("Global Hotkey: You pressed {:?}!", self.hotkey);
                    }
                }
            }
        });
    }
}
