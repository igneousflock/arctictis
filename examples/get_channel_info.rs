#![allow(clippy::unwrap_used)]

use arctictis::{
    Command, Scanner,
    bc125at::{
        channel_info::{ChannelIndex, GetChannelInfo},
        program_mode::{EnterProgramMode, ExitProgramMode},
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let idx = std::env::args().nth(1).unwrap().parse().unwrap();
    let idx = ChannelIndex::new(idx).unwrap();

    let mut scanner = Scanner::open()?;
    println!("{scanner:#?}");
    scanner.command(EnterProgramMode).await?;
    print_response(&mut scanner, GetChannelInfo(idx)).await?;
    scanner.command(ExitProgramMode).await?;

    Ok(())
}

async fn print_response<Cmd>(
    scanner: &mut Scanner,
    cmd: Cmd,
) -> Result<(), Box<dyn std::error::Error>>
where
    Cmd: Command + 'static,
    Cmd::Response: std::fmt::Debug,
{
    let name = String::from_utf8_lossy(Cmd::TEXT);
    let r = scanner.command(cmd).await?;
    println!("{name} => {r:?}");
    Ok(())
}
