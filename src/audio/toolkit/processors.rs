use std::{fs::File, io::BufReader, path::PathBuf};

use color_eyre::eyre::Result;
use hound::{WavReader, WavSpec};
use rand::{thread_rng, Rng};
use tracing::error;

use crate::audio::{
    processor::{AudioProcessor, AudioProcessorData},
    traits::FramedSamplesTrait,
};

/// Idenity processor, does literally nothing.
pub struct Identity;

impl AudioProcessor for Identity {
    fn process(&mut self, data: &mut AudioProcessorData) {
        // Less fine grained sample processing
        for mut frame in data.frames() {
            for (input_sample, output_sample) in frame.samples() {
                *output_sample = *input_sample;
            }
        }
    }
}

// Gain processor
pub struct Gain {
    amplitude: f32,
}

impl AudioProcessor for Gain {
    fn process(&mut self, data: &mut AudioProcessorData) {
        // Most fine grained processor approach
        for (input_frame, output_frame) in data.frames() {
            for (input_sample, output_sample) in (input_frame, output_frame).samples() {
                *output_sample = *input_sample * self.amplitude;
            }
        }
    }
}

// White noise generator
pub struct WhiteNoise;

impl AudioProcessor for WhiteNoise {
    fn process(&mut self, data: &mut AudioProcessorData) {
        for mut frame in data.frames() {
            for (_input_sample, output_sample) in frame.samples() {
                *output_sample = ((thread_rng().gen::<f32>() * 2.0) - 1.0) * 0.5;
            }
        }
    }
}

/// WAV file player
pub struct WavFile {
    reader: WavReader<BufReader<File>>,
}

impl WavFile {
    /// Load a wav file from a specified PathBuf
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let reader = WavReader::open(path)?;

        Ok(Self { reader })
    }

    /// Get the next sample from the file
    fn next_sample(&mut self) -> Result<f32> {
        let WavSpec {
            sample_format,
            bits_per_sample,
            ..
        } = self.reader.spec();

        match sample_format {
            // If it's a float the sample is already f32, just unwrap it
            hound::SampleFormat::Float => {
                Ok(self.reader.samples::<f32>().next().unwrap_or(Ok(0.0))?)
            }
            // Handle PCM encoded samples
            hound::SampleFormat::Int => {
                let next_pcm_sample = self.reader.samples::<i32>().next().unwrap_or(Ok(0))?;
                // Normalize the sample based on the pow(2, bits_per_sample).
                let normalized_sample =
                    next_pcm_sample as f32 / f32::powi(2.0, bits_per_sample as i32);
                Ok(normalized_sample)
            }
        }
    }
}

impl AudioProcessor for WavFile {
    fn process(&mut self, data: &mut AudioProcessorData) {
        for (input_frame, output_frame) in data.frames() {
            for (_input_sample, output_sample) in (input_frame, output_frame).samples() {
                *output_sample = self.next_sample().unwrap_or_else(|err| {
                    error!("Failed to process sample: {}", err);
                    0.0
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::audio::{
        processor::AudioBuffer,
        toolkit::{Gain, Identity},
    };

    use super::*;

    #[test]
    fn test_audio_processor_identity() {
        let size = 16 as usize;
        let mut input_buffer = AudioBuffer::new(1);
        let mut output_buffer = AudioBuffer::new(1);

        // Write some samples into the input_buffer
        for i in 0..8 {
            input_buffer.data[i] = i as f32;
        }

        let mut id = Identity;
        let mut data =
            AudioProcessorData::new(&mut input_buffer, size, &mut output_buffer, size, 44_100.0);

        // Process our samples into the output buffer, input = output (identity)
        id.process(&mut data);

        // If they're the same, we've processed the data into the output buffer.
        assert_eq!(input_buffer.data, output_buffer.data);
    }

    #[test]
    fn test_audio_processor_gain() {
        let size = 16 as usize;
        let mut input_buffer = AudioBuffer::new(2);
        let mut output_buffer = AudioBuffer::new(2);

        // Write some samples into the input_buffer
        for i in 0..8 {
            input_buffer.data[i] = i as f32;
        }

        let amplitude = 0.5;
        let mut gain = Gain { amplitude };
        let mut data =
            AudioProcessorData::new(&mut input_buffer, size, &mut output_buffer, size, 44_100.0);

        // Process our samples into the output buffer, output = input * amplitude
        gain.process(&mut data);

        // If they're the same, we've correctly processed the data into the output buffer.
        assert_eq!(input_buffer.data.map(|x| x * amplitude), output_buffer.data);
    }
}
