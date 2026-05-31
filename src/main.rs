use std::time::Duration;

use color_eyre::eyre::Result;
use simplelog::info;
use tokio::{time::{Instant, interval, sleep_until}};

use crate::{logging::{print_ascii_art, setup_logging}, quic::protocol::telemetry::Direction, telemetry::visual::camera::{Camera, CameraSettings}};

mod logging;
pub mod quic;
pub mod telemetry;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    setup_logging()?;
    print_ascii_art("Controller", "0.1.0", &["HttpRafa"]);

    info!("Starting...");

    let camera1 = Camera::setup(CameraSettings {
        index: 2,
        width: 1280,
        height: 720,
        rate: 30
    }, Direction::Forward);
    let camera2 = Camera::setup(CameraSettings {
        index: 4,
        width: 1280,
        height: 720,
        rate: 30
    }, Direction::Forward);
    let camera3 = Camera::setup(CameraSettings {
        index: 6,
        width: 1280,
        height: 720,
        rate: 30
    }, Direction::Forward);
    let camera4 = Camera::setup(CameraSettings {
        index: 8,
        width: 1280,
        height: 720,
        rate: 30
    }, Direction::Forward);

    sleep_until(Instant::now() + Duration::from_secs(1)).await;

    let mut interval = interval(Duration::from_millis(1000 / 30));
    while let (Some(frame1), Some(frame2), Some(frame3), Some(frame4)) = (camera1.frame().await, camera2.frame().await, camera3.frame().await, camera4.frame().await) {
        interval.tick().await;

        info!("Got a frames with size of: {}", frame1.len() + frame2.len() + frame3.len() + frame4.len());
    }

    Ok(())
}
