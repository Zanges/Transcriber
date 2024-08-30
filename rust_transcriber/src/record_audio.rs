use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct AudioRecorder {
    is_recording: Arc<AtomicBool>,
}

impl AudioRecorder {
    pub fn new() -> Self {
        AudioRecorder {
            is_recording: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start_recording(&self) -> Result<(), Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host.default_input_device().expect("No input device available");

        let config = device.default_input_config()?;

        let temp_dir = PathBuf::from("temporary");
        fs::create_dir_all(&temp_dir)?;

        let file_path = temp_dir.join("recorded_audio.wav");
        let spec = hound::WavSpec {
            channels: config.channels() as _,
            sample_rate: config.sample_rate().0 as _,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let writer = Arc::new(std::sync::Mutex::new(hound::WavWriter::create(file_path, spec)?));

        let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

        let is_recording = self.is_recording.clone();
        is_recording.store(true, Ordering::SeqCst);

        let stream = match config.sample_format() {
            SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| write_input_data::<f32, i16>(data, &writer, &is_recording),
                err_fn,
                None,
            )?,
            SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &_| write_input_data::<i16, i16>(data, &writer, &is_recording),
                err_fn,
                None,
            )?,
            SampleFormat::U16 => device.build_input_stream(
                &config.into(),
                move |data: &[u16], _: &_| write_input_data::<u16, i16>(data, &writer, &is_recording),
                err_fn,
                None,
            )?,
        };

        stream.play()?;

        // Keep the stream alive until recording is stopped
        while self.is_recording.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }

    pub fn stop_recording(&self) {
        self.is_recording.store(false, Ordering::SeqCst);
    }
}

fn write_input_data<T, U>(input: &[T], writer: &Arc<std::sync::Mutex<hound::WavWriter<std::io::BufWriter<std::fs::File>>>>, is_recording: &Arc<AtomicBool>)
where
    T: Sample,
    U: Sample + hound::Sample,
{
    if !is_recording.load(Ordering::SeqCst) {
        return;
    }

    if let Ok(mut guard) = writer.try_lock() {
        for &sample in input.iter() {
            let sample: U = U::from(&sample);
            guard.write_sample(sample).unwrap();
        }
    }
}
