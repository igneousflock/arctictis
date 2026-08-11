#![allow(clippy::unwrap_used)]

use std::fmt::Debug;

use arctictis::{
    NonProgramModeCommand, Scanner,
    bc125at::volume::{GetVolume, SetVolume, Volume},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open().unwrap();
    println!("{scanner:#?}");
    print_response(&mut scanner, GetVolume).await;
    print_response(&mut scanner, SetVolume(Volume::new(5).unwrap())).await;
    print_response(&mut scanner, GetVolume).await;

    Ok(())
}

async fn print_response<Cmd>(scanner: &mut Scanner, cmd: Cmd)
where
    Cmd: NonProgramModeCommand + Debug + 'static,
    Cmd::Response: std::fmt::Debug,
{
    let name = String::from_utf8_lossy(Cmd::TEXT);
    let r = scanner.command(cmd).await.unwrap();
    println!("{name} => {r:?}");
}
