use color_eyre::eyre::Result;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

pub mod quic;
pub mod visual;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    setup_logging()?;

    Ok(())
}

fn setup_logging() -> Result<()> {
    let mut config = ConfigBuilder::new();
    config.set_location_level(LevelFilter::Error);
    TermLogger::new(
        LevelFilter::Debug,
        config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    Ok(())
}
