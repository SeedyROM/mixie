use std::{iter::Zip, ops::Deref, slice::ChunksMut};

const MAX_BLOCK_SIZE: usize = 1024;

type AudioBufferInner = [f32; MAX_BLOCK_SIZE];

pub struct AudioBuffer {
    pub channels: usize,
    pub data: AudioBufferInner,
}

impl AudioBuffer {
    pub fn new(channels: usize) -> Self {
        Self {
            channels,
            data: [0.0; MAX_BLOCK_SIZE],
        }
    }

    pub fn frames(&mut self) -> ChunksMut<f32> {
        self.data.chunks_mut(self.channels)
    }

    pub fn zero(&mut self) {
        self.data.fill(0.0);
    }
}

impl Deref for AudioBuffer {
    type Target = AudioBufferInner;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct AudioProcessorData<'a> {
    pub sample_rate: f32,
    pub input_buffer: &'a mut AudioBuffer,
    pub output_buffer: &'a mut AudioBuffer,
}

impl<'a> AudioProcessorData<'a> {
    pub fn new(
        input_buffer: &'a mut AudioBuffer,
        output_buffer: &'a mut AudioBuffer,
        sample_rate: f32,
    ) -> Self {
        Self {
            sample_rate,
            input_buffer,
            output_buffer,
        }
    }

    pub fn frames(&mut self) -> Zip<ChunksMut<'_, f32>, ChunksMut<'_, f32>> {
        self.input_buffer.frames().zip(self.output_buffer.frames())
    }
}

pub trait AudioProcessor {
    fn process(&mut self, data: &mut AudioProcessorData);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_buffer() {
        let mut buf = AudioBuffer::new(2);

        for i in 0..4 {
            buf.data[i] = i as f32;
        }

        let mut frames = buf.frames();

        let mut samples = frames.next().unwrap();

        // Expect to get two samples at a time
        assert_eq!(samples[0], 0.0);
        assert_eq!(samples[1], 1.0);

        samples = frames.next().unwrap();

        // Same again
        assert_eq!(samples[0], 2.0);
        assert_eq!(samples[1], 3.0);
    }
}
