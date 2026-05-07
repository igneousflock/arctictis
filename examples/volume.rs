use arctictis::{
    Command, Scanner,
    bc125at::{GetVolumeLevel, SetVolumeLevel, VolumeLevel},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let volume = std::env::args()
        .nth(1)
        .expect("usage: `volume <VOLUME>`")
        .parse()
        .expect("volume should be a number between 0-15");
    let volume = VolumeLevel::new(volume).expect("invalid volume");

    let mut scanner = Scanner::open()?;
    println!("{scanner:#?}");
    print_response(&mut scanner, GetVolumeLevel).await?;
    print_response(&mut scanner, SetVolumeLevel(volume)).await?;
    print_response(&mut scanner, GetVolumeLevel).await?;

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
