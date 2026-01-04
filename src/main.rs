use uniden::Scanner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    dbg!(&scanner);
    dbg!(scanner.firmware_version().await?);
    dbg!(scanner.command("STS").await?);
    Ok(())
}
