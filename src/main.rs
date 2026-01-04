use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio_serial::{SerialPortBuilderExt, SerialPortType, SerialStream};
use tokio_util::codec::{AnyDelimiterCodec, AnyDelimiterCodecError, Framed};

const VENDOR_ID: u16 = 0x1965;
const PRODUCT_ID: u16 = 0x0017;

#[derive(Debug)]
struct Scanner(Framed<SerialStream, AnyDelimiterCodec>);

impl Scanner {
    fn open() -> Result<Option<Self>, tokio_serial::Error> {
        let ports = tokio_serial::available_ports()?;
        let Some(scanner_port_path) = ports.iter().find_map(|port| {
            let SerialPortType::UsbPort(usb_port_info) = &port.port_type else {
                return None;
            };
            (usb_port_info.vid == VENDOR_ID && usb_port_info.pid == PRODUCT_ID)
                .then_some(port.port_name.clone())
        }) else {
            return Ok(None);
        };

        let port = tokio_serial::new(&scanner_port_path, 115200)
            .timeout(Duration::from_secs(120))
            .open_native_async()?;

        let framed = Framed::new(port, AnyDelimiterCodec::new(b"\r".to_vec(), b"\r".to_vec()));

        Ok(Some(Self(framed)))
    }

    async fn response(&mut self) -> String {
        let r = self.0.next().await.unwrap().unwrap();
        String::from_utf8_lossy(&r).into_owned()
    }

    async fn command(&mut self, cmd: &str) -> Result<String, AnyDelimiterCodecError> {
        self.0.send(cmd).await?;
        Ok(self.response().await)
    }

    async fn firmware_version(&mut self) -> Result<String, AnyDelimiterCodecError> {
        self.command("VER").await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?.ok_or("scanner not found")?;
    dbg!(&scanner);
    dbg!(scanner.firmware_version().await?);
    dbg!(scanner.command("STS").await?);
    Ok(())
}
