use uniden::{
    Backlight, BandPlan, BatteryChargeTime, EnterProgramMode, ExitProgramMode, GetBacklight,
    GetBandPlan, GetBatteryInfo, GetFirmwareVersion, GetModelInfo, Scanner, SetBacklight,
    SetBandPlan, SetBatteryInfo,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    dbg!(&scanner);
    dbg!(scanner.command(GetFirmwareVersion).await?);
    dbg!(scanner.command(GetModelInfo).await?);
    dbg!(scanner.command(EnterProgramMode).await?);
    dbg!(scanner.command(GetBandPlan).await?);
    dbg!(scanner.command(SetBandPlan(BandPlan::Usa)).await?);
    dbg!(scanner.command(GetBandPlan).await?);
    dbg!(scanner.command(ExitProgramMode).await?);
    Ok(())
}
