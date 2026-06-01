use std::time::Duration;

use color_eyre::eyre::Result;
use tokio::time::{Instant, interval, sleep_until};
use v4l::FourCC;

use crate::{
    application::{
        logging::{print_ascii_art, setup_logging},
        version::VERSION,
    },
    quic::protocol::telemetry::Direction,
    telemetry::visual::camera::{Camera, CameraSettings},
};

mod application;
pub mod quic;
pub mod telemetry;

pub const AUTHORS: [&str; 1] = ["HttpRafa"];

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    setup_logging()?;
    print_ascii_art("Controller", &VERSION, &AUTHORS);

    let camera = Camera::setup(
        CameraSettings {
            index: 0,
            width: 1280,
            height: 720,
            rate: 30,
            format: FourCC::new(b"MJPG"),
        },
        Direction::Forward,
    );

    sleep_until(Instant::now() + Duration::from_secs(1)).await;

    let mut interval = interval(Duration::from_millis(1000 / 30));

    loop {
        interval.tick().await;

        let _frame = camera.latest_frame()?;
    }

    Ok(())
}
