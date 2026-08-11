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
    #[error("scanner not found")]
    ScannerNotFound,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Serial(#[from] tokio_serial::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError<Cmd: Command> {
    #[error("port closed")]
    PortClosed,
    #[error("response is empty")]
    ResponseEmpty,
    #[error("command not acceptable at this time")]
    CommandNotAcceptable,
    #[error("scanner returned error")]
    ErrorResponse,
    // TODO: add expected/received fields
    #[error("response is for wrong command")]
    WrongCommand,
    // TODO: add expected/received fields
    #[error("incorrect number of fields in response")]
    WrongNumberOfFields,

    #[error(transparent)]
    FieldDecodeError(<Cmd::Response as Response>::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl<Cmd: Command> From<DecoderError> for CommandError<Cmd> {
    fn from(error: DecoderError) -> Self {
        use tokio_util::codec::AnyDelimiterCodecError;
        match error {
            DecoderError::ResponseEmpty => Self::ResponseEmpty,
            DecoderError::CommandNotAcceptable => Self::CommandNotAcceptable,
            DecoderError::ErrorResponse => Self::ErrorResponse,
            DecoderError::DelimiterError(e) => match e {
                AnyDelimiterCodecError::MaxChunkLengthExceeded => {
                    unreachable!("we do not limit the chunk length")
                }
                AnyDelimiterCodecError::Io(io_error) => Self::Io(io_error),
            },
            DecoderError::Io(io_error) => Self::Io(io_error),
        }
    }
}

impl<Cmd: Command> From<ResponseError<<Cmd::Response as Response>::Error>> for CommandError<Cmd> {
    fn from(error: ResponseError<<Cmd::Response as Response>::Error>) -> Self {
        match error {
            ResponseError::WrongCommand => Self::WrongCommand,
            ResponseError::WrongNumberOfFields => Self::WrongNumberOfFields,
            ResponseError::InvalidFields(e) => Self::FieldDecodeError(e),
        }
    }
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
    ) -> Result<Cmd::Response, CommandError<Cmd>> {
        self.0.send(cmd).await?;

        let raw_response = self.0.next().await.ok_or(CommandError::PortClosed)??;
        let response = raw_response.deserialize::<Cmd>()?;

        Ok(response)
    }
}
