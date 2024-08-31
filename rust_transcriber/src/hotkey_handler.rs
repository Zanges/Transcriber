use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use winit::event_loop::EventLoop;
use crate::record_audio::AudioRecorder;
use crate::openai_transcribe::OpenAITranscriber;
use std::sync::{Arc, Mutex};

pub struct HotkeyHandler {
    #[allow(dead_code)]
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
    global_hotkey_channel: crossbeam_channel::Receiver<GlobalHotKeyEvent>,
    audio_recorder: Arc<Mutex<AudioRecorder>>,
    openai_transcriber: Arc<OpenAITranscriber>,
}

impl HotkeyHandler {
    pub fn new(hotkey_str: &str, audio_recorder: Arc<Mutex<AudioRecorder>>, openai_transcriber: Arc<OpenAITranscriber>) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = GlobalHotKeyManager::new()?;
        let hotkey = HotKey::new(None, hotkey_str.parse()?);
        manager.register(hotkey)?;

        Ok(Self {
            manager,
            hotkey,
            global_hotkey_channel: GlobalHotKeyEvent::receiver().clone(),
            audio_recorder,
            openai_transcriber,
        })
    }

    pub fn handle_events(self, event_loop: EventLoop<()>) {
        println!("Press and hold {:?} to record audio. Release to stop recording. Press Ctrl+C to exit.", self.hotkey);

        let audio_recorder = self.audio_recorder.clone();
        let openai_transcriber = self.openai_transcriber.clone();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;

            if let winit::event::Event::NewEvents(_) = event {
                if let Ok(hotkey_event) = self.global_hotkey_channel.try_recv() {
                    if hotkey_event.id == self.hotkey.id() {
                        match hotkey_event.state {
                            HotKeyState::Pressed => {
                                println!("Starting audio recording...");
                                if let Ok(mut recorder) = audio_recorder.lock() {
                                    if let Err(e) = recorder.start_recording() {
                                        eprintln!("Failed to start recording: {}", e);
                                    }
                                }
                            }
                            HotKeyState::Released => {
                                println!("Stopping audio recording...");
                                if let Ok(mut recorder) = audio_recorder.lock() {
                                    if let Some(audio_file_path) = recorder.stop_recording() {
                                        println!("Recording saved to: {:?}", audio_file_path);
                                        
                                        // Transcribe the recorded audio
                                        let transcriber = openai_transcriber.clone();
                                        let audio_file_path_str = audio_file_path.to_str().unwrap().to_string();
                                        tokio::spawn(async move {
                                            match transcriber.transcribe(&audio_file_path_str).await {
                                                Ok(transcription) => {
                                                    println!("Transcription: {}", transcription);
                                                    if let Err(e) = std::fs::remove_file(&audio_file_path_str) {
                                                        eprintln!("Failed to delete audio file: {}", e);
                                                    } else {
                                                        println!("Deleted audio file: {}", audio_file_path_str);
                                                    }
                                                },
                                                Err(e) => eprintln!("Failed to transcribe: {}", e),
                                            }
                                        });
                                    } else {
                                        println!("No recording was in progress.");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
