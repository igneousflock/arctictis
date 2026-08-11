#![allow(clippy::unwrap_used)]

use std::fmt::Debug;

use arctictis::{
    Command, Scanner,
    bc125at::{
        channel_info::{ChannelIndex, GetChannelInfo},
        program_mode::{EnterProgramMode, ExitProgramMode},
    },
};

#[tokio::main]
async fn main() {
    let idx = std::env::args().nth(1).unwrap().parse().unwrap();
    let idx = ChannelIndex::new(idx).unwrap();

    let mut scanner = Scanner::open().unwrap();
    println!("{scanner:#?}");
    print_response(&mut scanner, EnterProgramMode).await;
    print_response(&mut scanner, GetChannelInfo(idx)).await;
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
