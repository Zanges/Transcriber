use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AudioRecorder {
    is_recording: Arc<AtomicBool>,
    stream: Option<cpal::Stream>,
}

impl AudioRecorder {
    pub fn new() -> Self {
        AudioRecorder {
            is_recording: Arc::new(AtomicBool::new(false)),
            stream: None,
        }
    }

    pub fn start_recording(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_recording.load(Ordering::SeqCst) {
            return Ok(());
        }

        let host = cpal::default_host();
        let device = host.default_input_device().expect("No input device available");

        let config = device.default_input_config()?;

        let temp_dir = PathBuf::from("temporary");
        fs::create_dir_all(&temp_dir)?;

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let file_path = temp_dir.join(format!("recorded_audio_{}.wav", timestamp));
        let spec = hound::WavSpec {
            channels: config.channels() as _,
            sample_rate: config.sample_rate().0 as _,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let writer = Arc::new(std::sync::Mutex::new(match hound::WavWriter::create(file_path, spec) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Error creating WavWriter: {}", e);
                return Err(Box::new(e));
            }
        }));

        let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

        let is_recording = self.is_recording.clone();
        is_recording.store(true, Ordering::SeqCst);

        let stream = match config.sample_format() {
            SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| write_input_data(data, &writer, &is_recording),
                err_fn,
                None,
            )?,
            SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &_| write_input_data(data, &writer, &is_recording),
                err_fn,
                None,
            )?,
            SampleFormat::U16 => device.build_input_stream(
                &config.into(),
                move |data: &[u16], _: &_| write_input_data(data, &writer, &is_recording),
                err_fn,
                None,
            )?,
            _ => return Err("Unsupported sample format".into()),
        };

        stream.play()?;
        self.stream = Some(stream);

        println!("Recording started.");
        Ok(())
    }

    pub fn stop_recording(&mut self) {
        if !self.is_recording.load(Ordering::SeqCst) {
            return;
        }

        self.is_recording.store(false, Ordering::SeqCst);
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
        println!("Recording stopped and saved.");
    }
}

fn write_input_data<T>(input: &[T], writer: &Arc<std::sync::Mutex<hound::WavWriter<std::io::BufWriter<std::fs::File>>>>, is_recording: &Arc<AtomicBool>)
where
    T: Sample<Float = f32>,
{
    if !is_recording.load(Ordering::SeqCst) {
        return;
    }

    if let Ok(mut guard) = writer.try_lock() {
        for &sample in input.iter() {
            let sample_f32: f32 = sample.to_float_sample();
            if let Err(e) = guard.write_sample(sample_f32) {
                eprintln!("Error writing sample: {}", e);
                is_recording.store(false, Ordering::SeqCst);
                return;
            }
        }
    }
}
