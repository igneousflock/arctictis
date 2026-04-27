use uniden::{
    EnterProgramMode, ExitProgramMode, GetFirmwareVersion, GetKeyBeep, GetModelInfo, Scanner,
    SetKeyBeep,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    dbg!(&scanner);
    dbg!(scanner.command(GetFirmwareVersion).await?);
    dbg!(scanner.command(GetModelInfo).await?);
    dbg!(scanner.command(EnterProgramMode).await?);

    dbg!(scanner.command(GetKeyBeep).await?);
    dbg!(
        scanner
            .command(SetKeyBeep {
                beep_level: uniden::BeepLevel::Auto,
                key_lock_status: uniden::KeyLockStatus::Off
            })
            .await?
    );
    dbg!(scanner.command(GetKeyBeep).await?);

    dbg!(scanner.command(ExitProgramMode).await?);
    Ok(())
}
