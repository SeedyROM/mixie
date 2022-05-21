pub mod audio;
pub mod logging;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    logging::setup()?;

    Ok(())
}
