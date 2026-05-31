use simplelog::{error, warn};
use std::{
    sync::{Arc, Weak},
    thread::spawn,
};

use color_eyre::eyre::Result;
use tokio::sync::watch::{Receiver, Sender, channel};
use v4l::{
    Device, FourCC,
    buffer::Type,
    io::{mmap::Stream, traits::CaptureStream},
    video::Capture,
};

use crate::quic::protocol::telemetry::Direction;

pub type RawFrame = Arc<Vec<u8>>;

pub struct CameraSettings {
    pub index: u16,
    pub width: u32,
    pub height: u32,
    pub rate: u8,
}

pub struct Camera {
    direction: Direction,

    guard: Arc<()>,
    receiver: Receiver<Option<RawFrame>>,
}

impl Camera {
    pub fn setup(settings: CameraSettings, direction: Direction) -> Camera {
        let guard = Arc::new(());

        let (sender, receiver) = channel(None);

        {
            let weak: Weak<()> = Arc::downgrade(&guard);
            spawn(move || {
                if let Err(error) = Self::capture(weak, &settings, sender) {
                    error!("Camera {} crashed:", &settings.index);
                    error!("{:?}", error);
                }
            });
        }

        Self {
            direction,
            guard,
            receiver,
        }
    }

    pub async fn frame(&self) -> Option<RawFrame> {
        self.receiver.borrow().clone()
    }

    fn capture(
        flag: Weak<()>,
        settings: &CameraSettings,
        sender: Sender<Option<RawFrame>>,
    ) -> Result<()> {
        let device = Device::with_path(format!("/dev/video{}", settings.index))?;

        let mut format = device.format()?;
        format.width = settings.width;
        format.height = settings.height;
        format.fourcc = FourCC::new(b"MJPG");
        if device.set_format(&format)?.fourcc != format.fourcc {
            warn!("Camera {} did not accept the MJPEG format", settings.index);
        }

        let mut parameters = device.params()?;
        parameters.interval.numerator = 1;
        parameters.interval.denominator = settings.rate as u32;
        device.set_params(&parameters)?;

        let mut stream = Stream::with_buffers(&device, Type::VideoCapture, 4)?;

        while flag.strong_count() > 0 {
            let (buffer, metadata) = stream.next()?;
            let payload = &buffer[..metadata.bytesused as usize];

            if sender.send(Some(Arc::new(payload.to_vec()))).is_err() {
                break;
            }
        }

        Ok(())
    }
}
