use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use winit::event_loop::EventLoop;
use crate::record_audio::AudioRecorder;
use crate::openai_handler::OpenAIHandler;
use std::sync::{Arc, Mutex};
use std::path::Path;
use tokio::runtime::Runtime;

pub struct HotkeyHandler {
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
    global_hotkey_channel: crossbeam_channel::Receiver<GlobalHotKeyEvent>,
    audio_recorder: Arc<Mutex<AudioRecorder>>,
    openai_handler: Arc<OpenAIHandler>,
    runtime: Runtime,
}

impl HotkeyHandler {
    pub fn new(hotkey_str: &str, openai_handler: Arc<OpenAIHandler>) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = GlobalHotKeyManager::new()?;
        let hotkey = HotKey::new(None, hotkey_str.parse()?);
        manager.register(hotkey)?;

        Ok(Self {
            manager,
            hotkey,
            global_hotkey_channel: GlobalHotKeyEvent::receiver().clone(),
            audio_recorder: Arc::new(Mutex::new(AudioRecorder::new())),
            openai_handler,
            runtime: Runtime::new()?,
        })
    }

    pub fn handle_events(self, event_loop: EventLoop<()>) {
        println!("Press and hold {:?} to record audio. Release to stop recording and transcribe. Press Ctrl+C to exit.", self.hotkey);

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
                                    let audio_path = Path::new("temporary").join("recorded_audio.wav");
                                    let openai_handler = self.openai_handler.clone();
                                    self.runtime.spawn(async move {
                                        match openai_handler.transcribe_audio(&audio_path).await {
                                            Ok(transcription) => println!("Transcription: {}", transcription),
                                            Err(e) => eprintln!("Failed to transcribe audio: {}", e),
                                        }
                                    });
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
