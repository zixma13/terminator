use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

/// Captures PCM audio from the default input device, resamples to 16kHz mono float32.
pub struct AudioCapture {
    stream: Option<cpal::Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: u32,
}

/// Shared live audio level for waveform display.
pub type AudioLevel = Arc<Mutex<Vec<f32>>>;

impl AudioCapture {
    pub fn new() -> Result<(Self, AudioLevel)> {
        let level = Arc::new(Mutex::new(vec![0.0f32; 60]));
        Ok((
            Self {
                stream: None,
                buffer: Arc::new(Mutex::new(Vec::new())),
                sample_rate: 48000,
            },
            level,
        ))
    }

    /// Start recording from the microphone at its native sample rate.
    pub fn start(&mut self, level: &AudioLevel) -> Result<()> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device found"))?;

        let default_config = device.default_input_config()?;
        self.sample_rate = default_config.sample_rate().0;
        let channels = default_config.channels();

        let config = cpal::StreamConfig {
            channels,
            sample_rate: cpal::SampleRate(self.sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        self.buffer.lock().unwrap().clear();

        let buf = Arc::clone(&self.buffer);
        let lvl = Arc::clone(level);
        let ch = channels as usize;
        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mono: Vec<f32> = if ch > 1 {
                    data.chunks(ch)
                        .map(|frame| frame.iter().sum::<f32>() / ch as f32)
                        .collect()
                } else {
                    data.to_vec()
                };
                buf.lock().unwrap().extend_from_slice(&mono);

                // Update live waveform (keep recent peak samples for display)
                if !mono.is_empty() {
                    let mut levels = lvl.lock().unwrap();
                    // Sample every Nth value to get wave shape, not just amplitude
                    let step = (mono.len() / 4).max(1);
                    for i in (0..mono.len()).step_by(step) {
                        levels.push(mono[i]); // actual sample value, can be negative
                    }
                    let excess = levels.len().saturating_sub(200);
                    if excess > 0 {
                        levels.drain(..excess);
                    }
                }
            },
            |err| eprintln!("Audio error: {err}"),
            None,
        )?;

        stream.play()?;
        self.stream = Some(stream);
        Ok(())
    }

    /// Stop recording and return PCM resampled to 16kHz mono float32.
    pub fn stop(&mut self) -> Vec<f32> {
        self.stream = None;
        let mut buf = self.buffer.lock().unwrap();
        let raw = std::mem::take(&mut *buf);

        if raw.is_empty() || self.sample_rate == 16000 {
            return raw;
        }

        let ratio = 16000.0 / self.sample_rate as f64;
        let out_len = (raw.len() as f64 * ratio) as usize;
        let mut resampled = Vec::with_capacity(out_len);

        for i in 0..out_len {
            let src_idx = i as f64 / ratio;
            let idx = src_idx as usize;
            let frac = src_idx - idx as f64;
            let s0 = raw[idx.min(raw.len() - 1)];
            let s1 = raw[(idx + 1).min(raw.len() - 1)];
            resampled.push(s0 + (s1 - s0) * frac as f32);
        }

        resampled
    }

    /// Encode a PCM f32 buffer as base64 bytes.
    pub fn encode_base64(pcm: &[f32]) -> String {
        let bytes: Vec<u8> = pcm.iter().flat_map(|&s| s.to_le_bytes()).collect();
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes)
    }
}
