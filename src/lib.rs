mod codec;

use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio_serial::{SerialPortBuilderExt, SerialPortType, SerialStream};
use tokio_util::codec::{AnyDelimiterCodecError, Framed};

use crate::codec::Codec;

pub use crate::codec::Command;

const VENDOR_ID: u16 = 0x1965;
const PRODUCT_ID: u16 = 0x0017;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("port closed")]
    PortClosed,

    #[error("scanner not found")]
    ScannerNotFound,

    #[error(transparent)]
    Codec(#[from] AnyDelimiterCodecError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Serial(#[from] tokio_serial::Error),
}

#[derive(Debug)]
pub struct Scanner(Framed<SerialStream, Codec>);

impl Scanner {
    pub fn open() -> Result<Self, Error> {
        let ports = tokio_serial::available_ports()?;
        let Some(scanner_port_path) = ports.iter().find_map(|port| {
            let SerialPortType::UsbPort(usb_port_info) = &port.port_type else {
                return None;
            };
            (usb_port_info.vid == VENDOR_ID && usb_port_info.pid == PRODUCT_ID)
                .then_some(port.port_name.clone())
        }) else {
            return Err(Error::ScannerNotFound);
        };

        let port = tokio_serial::new(&scanner_port_path, 115200)
            .timeout(Duration::from_secs(120))
            .open_native_async()?;

        let framed = Framed::new(port, Codec::new());

        Ok(Self(framed))
    }

    pub async fn command(&mut self, cmd: Command) -> Result<String, Error> {
        self.0.send(cmd).await?;
        let r = self.0.next().await.ok_or(Error::PortClosed)??;
        Ok(r)
    }

    pub async fn firmware_version(&mut self) -> Result<String, Error> {
        self.command(Command::Ver).await
    }
}
