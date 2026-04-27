use uniden::{
    Backlight, BatteryChargeTime, EnterProgramMode, ExitProgramMode, GetBacklight, GetBatteryInfo,
    GetFirmwareVersion, GetModelInfo, Scanner, SetBacklight, SetBatteryInfo,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    dbg!(&scanner);
    dbg!(scanner.command(GetFirmwareVersion).await?);
    dbg!(scanner.command(GetModelInfo).await?);
    dbg!(scanner.command(EnterProgramMode).await?);
    dbg!(scanner.command(GetBatteryInfo).await?);
    dbg!(
        scanner
            .command(SetBatteryInfo {
                charge_time: BatteryChargeTime::new(16)
            })
            .await?
    );
    dbg!(scanner.command(GetBatteryInfo).await?);
    dbg!(scanner.command(ExitProgramMode).await?);
    Ok(())
}
