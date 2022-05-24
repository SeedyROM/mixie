use color_eyre::eyre::{eyre, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, OutputCallbackInfo, StreamConfig, SupportedStreamConfig,
};
use crossbeam::channel::Receiver;
use tracing::{error, info};

use crate::audio::{AudioBuffer, WavFile};

use super::{AudioProcessor, AudioProcessorData, MAX_BLOCK_SIZE};

/// The global audio system
pub struct AudioSystem {
    // TODO: Move me into my own struct
    host: Host,
    device: Device,
    supported_stream_config: SupportedStreamConfig,
    stream_config: StreamConfig,

    /// Handle shutdown
    shutdown_rx: Receiver<()>,
}

impl AudioSystem {
    /// Create a new audio system to play some sounds.
    pub fn new(shutdown_rx: Receiver<()>) -> Result<Self> {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .ok_or_else(|| eyre!("Failed to load default output device"))?;

        let supported_stream_config = device.default_output_config()?;
        let stream_config = supported_stream_config.clone().into();

        Ok(Self {
            host,
            device,
            supported_stream_config,
            stream_config,
            shutdown_rx,
        })
    }

    /// Add a struct that implements AudioResource to the system.
    // pub fn add_resource(&mut self, resource: impl AudioResource + 'static) {
    //     self.resources.lock().push(Box::new(resource));
    // }

    /// Run the audio system and start a stream with the specified sample format.
    pub fn run(&self) -> Result<()> {
        match self.supported_stream_config.sample_format() {
            cpal::SampleFormat::I16 => self.stream::<i16>(),
            cpal::SampleFormat::U16 => self.stream::<u16>(),
            cpal::SampleFormat::F32 => self.stream::<f32>(),
        }
    }

    /// Start an audio stream
    fn stream<S>(&self) -> Result<()>
    where
        S: cpal::Sample,
    {
        info!(
            "Starting stream at host {:?} with device: {}",
            self.host.id(),
            self.device.name().unwrap_or("Unknown Device".into())
        );

        let channels = self.stream_config.channels as usize;
        let sample_rate = self.stream_config.sample_rate.0 as f32;

        let mut input_buffer = AudioBuffer::new(channels);
        let mut output_buffer = AudioBuffer::new(channels);

        let mut wav_file = WavFile::from_path("./data/lighter.wav".into())?;

        let stream = self.device.build_output_stream(
            &self.stream_config,
            move |data: &mut [S], output_callback_info: &OutputCallbackInfo| {
                Self::stream_callback(
                    channels,
                    sample_rate,
                    &mut input_buffer,
                    &mut output_buffer,
                    &mut wav_file,
                    data,
                    output_callback_info,
                )
            },
            |err| error!("Stream callback error: {}", err),
        )?;

        stream.play()?;

        // Wait for shutdown signal
        self.shutdown_rx.recv()?;

        info!(
            "Stopping stream at host {:?} with device: {}",
            self.host.id(),
            self.device.name().unwrap_or("Unknown Device".into())
        );

        Ok(())
    }

    /// Send data to our audio stream
    fn stream_callback<S>(
        channels: usize,
        sample_rate: f32,
        input_buffer: &mut AudioBuffer,
        output_buffer: &mut AudioBuffer,
        wav_file: &mut WavFile,
        data: &mut [S],
        _: &OutputCallbackInfo,
    ) where
        S: cpal::Sample,
    {
        input_buffer.zero();
        output_buffer.zero();

        // Handle variable blockSize
        for chunk in data.chunks_mut(MAX_BLOCK_SIZE - (MAX_BLOCK_SIZE % channels)) {
            // Create an AudioProcessorData struct and pass it to the processor.
            let processor_data = &mut AudioProcessorData {
                sample_rate,
                input_buffer,
                input_buffer_size: chunk.len(),
                output_buffer,
                output_buffer_size: chunk.len(),
            };

            // Create some white noise
            wav_file.process(processor_data);

            // Write the output buffer to the chunk
            chunk
                .iter_mut()
                .zip(output_buffer.iter())
                .for_each(|(o, i)| *o = S::from(i));
        }
    }
}
