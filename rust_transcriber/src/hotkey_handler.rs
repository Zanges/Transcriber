use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use winit::event_loop::EventLoop;
use crate::record_audio::AudioRecorder;
use std::sync::{Arc, Mutex};

pub struct HotkeyHandler {
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
    global_hotkey_channel: crossbeam_channel::Receiver<GlobalHotKeyEvent>,
    audio_recorder: Arc<Mutex<AudioRecorder>>,
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
            audio_recorder: Arc::new(Mutex::new(AudioRecorder::new())),
        })
    }

    pub fn handle_events(self, event_loop: EventLoop<()>) {
        println!("Press and hold {:?} to record audio. Release to stop recording. Press Ctrl+C to exit.", self.hotkey);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;

            if let winit::event::Event::NewEvents(_) = event {
                if let Ok(hotkey_event) = self.global_hotkey_channel.try_recv() {
                    if hotkey_event.id == self.hotkey.id() {
                        match hotkey_event.state {
                            HotKeyState::Pressed => {
                                println!("Starting audio recording...");
                                if let Ok(mut recorder) = self.audio_recorder.lock() {
                                    if let Err(e) = recorder.start_recording() {
                                        eprintln!("Failed to start recording: {}", e);
                                    }
                                }
                            }
                            HotKeyState::Released => {
                                println!("Stopping audio recording...");
                                if let Ok(mut recorder) = self.audio_recorder.lock() {
                                    recorder.stop_recording();
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
use openai::{
    Client,
    types::{
        CreateTranscriptionRequest,
        CreateTranscriptionRequestArgs,
        AudioInput,
    },
};
use std::path::Path;

pub struct OpenAIHandler {
    client: Client,
}

impl OpenAIHandler {
    pub fn new(api_key: String) -> Self {
        OpenAIHandler {
            client: Client::new(api_key),
        }
    }

    pub async fn transcribe_audio(&self, audio_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let audio_input = AudioInput::File(audio_path.to_path_buf());
        let request = CreateTranscriptionRequestArgs::default()
            .file(audio_input)
            .model("whisper-1")
            .build()?;

        let response = self.client.audio().transcribe(request).await?;
        Ok(response.text)
    }
}
