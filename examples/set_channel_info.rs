#![allow(clippy::unwrap_used)]

use std::fmt::Debug;

use arctictis::{
    Command, Scanner,
    bc125at::{
        channel_info::{ChannelIndex, ChannelInfo, Frequency, GetChannelInfo, SetChannelInfo},
        program_mode::{EnterProgramMode, ExitProgramMode},
    },
};

#[tokio::main]
async fn main() {
    let idx = std::env::args().nth(1).unwrap().parse().unwrap();
    let idx = ChannelIndex::new(idx).unwrap();

    let freq = std::env::args().nth(2).unwrap();
    let freq = Frequency::new(freq.as_bytes()).unwrap();

    let mut scanner = Scanner::open().unwrap();
    println!("{scanner:#?}");
    scanner.command(EnterProgramMode).await.unwrap();
    print_response(&mut scanner, EnterProgramMode).await;
    let current_info = scanner.command(GetChannelInfo(idx)).await.unwrap();
    let new_info = ChannelInfo {
        frequency: freq,
        ..current_info
    };
    print_response(&mut scanner, SetChannelInfo(new_info)).await;
    print_response(&mut scanner, ExitProgramMode).await;
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
