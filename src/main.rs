use uniden::{GetFirmwareVersion, Scanner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    dbg!(&scanner);
    dbg!(scanner.command(GetFirmwareVersion).await?);
    Ok(())
}
