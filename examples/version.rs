#![allow(clippy::unwrap_used)]

use std::fmt::Debug;

use arctictis::{Command, Scanner, bc125at::firmware_version::GetFirmwareVersion};

#[tokio::main]
async fn main() {
    let mut scanner = Scanner::open().unwrap();
    println!("{scanner:#?}");
    print_response(&mut scanner, GetFirmwareVersion).await;
}

async fn print_response<Cmd>(scanner: &mut Scanner, cmd: Cmd)
where
    Cmd: Command + Debug + 'static,
    Cmd::Response: std::fmt::Debug,
{
    let name = String::from_utf8_lossy(Cmd::TEXT);
    let r = scanner.command(cmd).await.unwrap();
    println!("{name} => {r:?}");
}
