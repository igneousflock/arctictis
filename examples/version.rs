use arctictis::{
    Command, Scanner,
    bc125at::{GetFirmwareVersion, GetModelInfo},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    println!("{scanner:#?}");
    print_response(&mut scanner, GetFirmwareVersion).await?;
    print_response(&mut scanner, GetModelInfo).await?;

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
