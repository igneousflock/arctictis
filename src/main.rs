use uniden::{Backlight, Command, Scanner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    dbg!(&scanner);
    dbg!(scanner.firmware_version().await?);
    dbg!(scanner.command(Command::Prg).await?);
    // tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    dbg!(scanner.command(Command::Bsv(None)).await?);
    dbg!(scanner.command(Command::Bsv(Some(4))).await?);
    dbg!(scanner.command(Command::Bsv(None)).await?);
    dbg!(scanner.command(Command::Epg).await?);
    Ok(())
}
