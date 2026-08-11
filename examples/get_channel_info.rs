#![allow(clippy::unwrap_used)]

use std::fmt::Debug;

use arctictis::{
    Command, ProgramModeScanner, Scanner,
    bc125at::channel_info::{ChannelIndex, GetChannelInfo},
};

#[tokio::main]
async fn main() {
    let idx = std::env::args().nth(1).unwrap().parse().unwrap();
    let idx = ChannelIndex::new(idx).unwrap();

    let mut scanner = Scanner::open().unwrap();
    println!("{scanner:#?}");

    scanner
        .with_program_mode(async |mut scanner| {
            print_response(&mut scanner, GetChannelInfo(idx)).await;
        })
        .await
        .unwrap();
}

async fn print_response<Cmd>(scanner: &mut ProgramModeScanner<'_>, cmd: Cmd)
where
    Cmd: Command + Debug + 'static,
    Cmd::Response: std::fmt::Debug,
{
    let name = String::from_utf8_lossy(Cmd::TEXT);
    let r = scanner.command(cmd).await.unwrap();
    println!("{name} => {r:?}");
}
