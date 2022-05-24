pub mod audio;
pub mod logging;

use color_eyre::eyre::Result;

use audio::AudioSystem;
use tracing::info;

fn main() -> Result<()> {
    // Setup logging
    logging::setup()?;

    // Handle shutdown
    let (shutdown_tx, shutdown_rx) = crossbeam::channel::bounded::<()>(1);
    ctrlc::set_handler(move || {
        info!("Shutting down...");
        shutdown_tx
            .send(())
            .expect("Failed to send shutdown signal...");
    })?;

    // Create the audio system and add our wave file resource and some noise for fun!
    let audio_sys = AudioSystem::new(shutdown_rx.clone())?;

    // 🎶 Make some NOISE! 🎶
    audio_sys.run()?;

    Ok(())
}
