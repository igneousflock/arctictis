use arctictis::{
    Command, Scanner,
    command::{GetFirmwareVersion, GetModelInfo},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::open()?;
    println!("{scanner:#?}");
    print_response(&mut scanner, GetFirmwareVersion).await?;
    print_response(&mut scanner, GetModelInfo).await?;

    Ok(())
}

async fn print_response<T>(scanner: &mut Scanner, cmd: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Command,
{
    let name = cmd.to_string();
    let r = scanner.command(cmd.clone()).await?;
    println!("{name} => {r:#?}");
    Ok(())
}
