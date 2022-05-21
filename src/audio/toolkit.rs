use crate::audio::{
    processor::{AudioProcessor, AudioProcessorData},
    traits::FramedSamplesTrait,
};

pub struct Identity;

impl AudioProcessor for Identity {
    fn process(&mut self, data: &mut AudioProcessorData) {
        for (input_frame, output_frame) in data.frames() {
            for (input_sample, output_sample) in (input_frame, output_frame).samples() {
                *output_sample = *input_sample;
            }
        }
    }
}

pub struct Gain {
    amplitude: f32,
}

impl AudioProcessor for Gain {
    fn process(&mut self, data: &mut AudioProcessorData) {
        for (input_frame, output_frame) in data.frames() {
            for (input_sample, output_sample) in (input_frame, output_frame).samples() {
                *output_sample = *input_sample * self.amplitude;
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
        let mut input_buffer = AudioBuffer::new(1);
        let mut output_buffer: crate::audio::processor::AudioBuffer = AudioBuffer::new(1);

        // Write some samples into the input_buffer
        for i in 0..8 {
            input_buffer.data[i] = i as f32;
        }

        let mut id = Identity;
        let mut data = AudioProcessorData::new(&mut input_buffer, &mut output_buffer, 44_100.0);

        // Process our samples into the output buffer, input = output (identity)
        id.process(&mut data);

        // If they're the same, we've processed the data into the output buffer.
        assert_eq!(input_buffer.data, output_buffer.data);
    }

    #[test]
    fn test_audio_processor_gain() {
        let mut input_buffer = AudioBuffer::new(2);
        let mut output_buffer = AudioBuffer::new(2);

        // Write some samples into the input_buffer
        for i in 0..8 {
            input_buffer.data[i] = i as f32;
        }

        let amplitude = 0.5;
        let mut gain = Gain { amplitude };
        let mut data = AudioProcessorData::new(&mut input_buffer, &mut output_buffer, 44_100.0);

        // Process our samples into the output buffer, input = output (identity)
        gain.process(&mut data);

        // If they're the same, we've processed the data into the output buffer.
        assert_eq!(input_buffer.data.map(|x| x * amplitude), output_buffer.data);
    }
}
