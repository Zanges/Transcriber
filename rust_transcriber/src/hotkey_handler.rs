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

        let (sender, receiver) = crossbeam_channel::unbounded();
        GlobalHotKeyEvent::set_receiver(sender);

        Ok(Self {
            manager,
            hotkey,
            global_hotkey_channel: receiver,
        })
    }

    pub fn handle_events(self, event_loop: EventLoop<()>) {
        println!("Press F7 to trigger the global hotkey. Press Ctrl+C to exit.");

        let hotkey = self.hotkey;
        let channel = self.global_hotkey_channel;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;

            if let winit::event::Event::NewEvents(_) = event {
                while let Ok(hotkey_event) = channel.try_recv() {
                    if hotkey_event.id == hotkey.id() && hotkey_event.state == HotKeyState::Pressed {
                        println!("Global Hotkey: You pressed F7!");
                    }
                }
            }
        });
    }
}
