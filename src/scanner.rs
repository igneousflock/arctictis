use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio_serial::{SerialPortBuilderExt, SerialPortType, SerialStream};
use tokio_util::codec::Framed;

use crate::{
    codec::{Codec, DecoderError, ResponseError},
    command::{Command, Response},
};

const VENDOR_ID: u16 = 0x1965;
const PRODUCT_ID: u16 = 0x0017;
const TIMEOUT: Duration = Duration::from_mins(2);
const BAUD_RATE: u32 = 115_200;

#[derive(Debug, thiserror::Error)]
pub enum ScannerError {
    #[error("port closed")]
    PortClosed,

    #[error("scanner not found")]
    ScannerNotFound,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Serial(#[from] tokio_serial::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum EncodingError<E> {
    #[error(transparent)]
    Scanner(#[from] ScannerError),

    #[error(transparent)]
    Decoder(#[from] DecoderError),

    #[error(transparent)]
    ResponseParserError(#[from] ResponseError<E>),
}

#[derive(Debug)]
pub struct Scanner(Framed<SerialStream, Codec>);

impl Scanner {
    pub fn open() -> Result<Self, ScannerError> {
        let ports = tokio_serial::available_ports()?;
        let Some(scanner_port_path) = ports.iter().find_map(|port| {
            let SerialPortType::UsbPort(usb_port_info) = &port.port_type else {
                return None;
            };
            (usb_port_info.vid == VENDOR_ID && usb_port_info.pid == PRODUCT_ID)
                .then_some(port.port_name.clone())
        }) else {
            return Err(ScannerError::ScannerNotFound);
        };

        let port = tokio_serial::new(&scanner_port_path, BAUD_RATE)
            .timeout(TIMEOUT)
            .open_native_async()?;

        let framed = Framed::new(port, Codec::new());

        Ok(Self(framed))
    }

    pub async fn command<Cmd: Command>(
        &mut self,
        cmd: Cmd,
    ) -> Result<Cmd::Response, EncodingError<<Cmd::Response as Response>::Error>> {
        self.0.send(cmd).await.map_err(ScannerError::from)?;
        let raw_response = self.0.next().await.ok_or(ScannerError::PortClosed)??;
        let response = raw_response.deserialize::<Cmd>()?;
        Ok(response)
    }
}
