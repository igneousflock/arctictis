#![allow(clippy::unwrap_used)]

use arctictis::{
    Command, Scanner,
    bc125at::volume::{GetVolume, SetVolume, Volume},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    println!("{scanner:#?}");
    print_response(&mut scanner, GetVolume).await?;
    print_response(&mut scanner, SetVolume(Volume::new(5).unwrap())).await?;
    print_response(&mut scanner, GetVolume).await?;

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
