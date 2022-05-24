use std::{iter::Zip, ops::Deref, slice::ChunksMut};

pub const MAX_BLOCK_SIZE: usize = 1024;

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

    pub fn frames(&mut self, size: usize) -> ChunksMut<f32> {
        self.data[..size].chunks_mut(self.channels)
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
    pub input_buffer_size: usize,
    pub output_buffer: &'a mut AudioBuffer,
    pub output_buffer_size: usize,
}

impl<'a> AudioProcessorData<'a> {
    pub fn new(
        input_buffer: &'a mut AudioBuffer,
        input_buffer_size: usize,
        output_buffer: &'a mut AudioBuffer,
        output_buffer_size: usize,
        sample_rate: f32,
    ) -> Self {
        Self {
            sample_rate,
            input_buffer,
            input_buffer_size,
            output_buffer,
            output_buffer_size,
        }
    }

    pub fn frames(&mut self) -> Zip<ChunksMut<'_, f32>, ChunksMut<'_, f32>> {
        self.input_buffer
            .frames(self.input_buffer_size)
            .zip(self.output_buffer.frames(self.output_buffer_size))
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

        let mut frames = buf.frames(16);

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
