use bytes::Bytes;
use simplelog::warn;
use std::sync::{Arc, Weak};

use color_eyre::eyre::{Error, Result, eyre};
use tokio::{
    sync::watch::{Receiver, Sender, channel},
    task::spawn_blocking,
};
use v4l::{
    Device, FourCC,
    buffer::Type,
    io::{mmap::Stream, traits::CaptureStream},
    video::Capture,
};

use crate::quic::protocol::telemetry::Direction;

pub type FrameResult = Result<Option<Bytes>, Arc<Error>>;

pub struct CameraSettings {
    pub index: u16,
    pub width: u32,
    pub height: u32,
    pub rate: u8,
    pub format: FourCC,
}

pub struct Camera {
    direction: Direction,

    guard: Arc<()>,
    receiver: Receiver<FrameResult>,
}

impl Camera {
    pub fn setup(settings: CameraSettings, direction: Direction) -> Camera {
        let guard = Arc::new(());
        let weak: Weak<()> = Arc::downgrade(&guard);

        let (sender, receiver) = channel(Ok(None));

        spawn_blocking(move || {
            if let Err(error) = Self::capture(weak, &settings, &sender) {
                let _ = sender.send(Err(Arc::new(error)));
            }
        });

        Self {
            direction,
            guard,
            receiver,
        }
    }

    pub fn latest_frame(&self) -> Result<Option<Bytes>> {
        self.receiver
            .borrow()
            .clone()
            .map_err(|error| eyre!("A capture error was detected: {:?}", error))
    }

    fn capture(
        flag: Weak<()>,
        settings: &CameraSettings,
        sender: &Sender<FrameResult>,
    ) -> Result<()> {
        let device = Device::with_path(format!("/dev/video{}", settings.index))?;

        let mut format = device.format()?;
        format.width = settings.width;
        format.height = settings.height;
        format.fourcc = settings.format;
        if device.set_format(&format)?.fourcc != format.fourcc {
            warn!(
                "Camera {} did not accept the {} format",
                settings.index, settings.format
            );
        }

        let mut parameters = device.params()?;
        parameters.interval.numerator = 1;
        parameters.interval.denominator = settings.rate as u32;
        device.set_params(&parameters)?;

        let mut stream = Stream::with_buffers(&device, Type::VideoCapture, 4)?;

        while flag.strong_count() > 0 {
            let (buffer, metadata) = stream.next()?;
            let payload = &buffer[..metadata.bytesused as usize];

            if sender
                .send(Ok(Some(Bytes::copy_from_slice(payload))))
                .is_err()
            {
                break;
            }
        }

        Ok(())
    }
}
