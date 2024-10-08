use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use crate::record_audio::AudioRecorder;
use crate::openai_transcribe::OpenAITranscriber;
use crate::output_handler::OutputHandler;
use crate::config_handler::Config;
use std::sync::{Arc, Mutex};

pub struct HotkeyHandler {
    #[allow(dead_code)]
    manager: Arc<GlobalHotKeyManager>,
    hotkey: HotKey,
    global_hotkey_channel: Arc<crossbeam_channel::Receiver<GlobalHotKeyEvent>>,
    audio_recorder: Arc<Mutex<AudioRecorder>>,
    openai_transcriber: Arc<OpenAITranscriber>,
    output_handler: Arc<OutputHandler>,
}

// Implement Send for HotkeyHandler
unsafe impl Send for HotkeyHandler {}

impl HotkeyHandler {
    pub fn new(hotkey_str: &str, audio_recorder: Option<Arc<Mutex<AudioRecorder>>>, openai_transcriber: Option<Arc<OpenAITranscriber>>, output_handler: Option<Arc<OutputHandler>>) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = Arc::new(GlobalHotKeyManager::new()?);
        let hotkey = HotKey::new(None, hotkey_str.parse()?);
        manager.register(hotkey)?;

        Ok(Self {
            manager,
            hotkey,
            global_hotkey_channel: Arc::new(GlobalHotKeyEvent::receiver().clone()),
            audio_recorder: audio_recorder.unwrap_or_else(|| Arc::new(Mutex::new(AudioRecorder::new(&Config::default())))),
            openai_transcriber: openai_transcriber.unwrap_or_else(|| Arc::new(OpenAITranscriber::new(String::new()))),
            output_handler: output_handler.unwrap_or_else(|| Arc::new(OutputHandler::new(0, 0))),
        })
    }

    pub async fn handle_events(self, mut rx: mpsc::Receiver<()>) {
        println!("Press and hold {:?} to record audio. Release to stop recording.", self.hotkey);

        let audio_recorder = self.audio_recorder.clone();
        let openai_transcriber = self.openai_transcriber.clone();
        let output_handler = self.output_handler.clone();
        let global_hotkey_channel = Arc::clone(&self.global_hotkey_channel);

        loop {
            tokio::select! {
                _ = async {
                    if let Ok(hotkey_event) = global_hotkey_channel.try_recv() {
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
                                            let output_handler = output_handler.clone();
                                            tokio::spawn(async move {
                                                match transcriber.transcribe(&audio_file_path_str).await {
                                                    Ok(transcription) => {
                                                        println!("Transcription: {}", transcription);
                                                        output_handler.type_text(&transcription);
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
                    sleep(Duration::from_millis(10)).await;
                } => {}
                _ = rx.recv() => {
                    println!("Received exit signal. Stopping hotkey handler.");
                    break;
                }
            }
        }
    }
}
