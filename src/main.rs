use uniden::{Command, KeyBeepLevel, KeyBeepSettings, Scanner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    dbg!(&scanner);
    dbg!(scanner.firmware_version().await?);
    dbg!(scanner.command(Command::Prg).await?);
    // tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    dbg!(
        scanner
            .command(Command::Kbp(Some(KeyBeepSettings {
                beep_level: KeyBeepLevel::Auto,
                lock_status: false,
            })))
            .await?
    );
    dbg!(scanner.command(Command::Kbp(None)).await?);
    dbg!(scanner.command(Command::Epg).await?);
    Ok(())
}
