#![allow(clippy::unwrap_used)]

use std::fmt::Debug;

use arctictis::{
    Command, ProgramModeScanner, Scanner,
    bc125at::channel_info::{ChannelIndex, GetChannelInfo},
};

#[tokio::main]
async fn main() {
    let mut scanner = Scanner::open().unwrap();
    println!("{scanner:#?}");

    scanner
        .with_program_mode(async |mut scanner| {
            for i in 1..=500 {
                let idx = ChannelIndex::new(i).unwrap();
                let channel_info = scanner.command(GetChannelInfo(idx)).await.unwrap();
                println!(
                    "{i}: {}, {}",
                    String::from_utf8_lossy(channel_info.frequency.value()),
                    String::from_utf8_lossy(channel_info.name.value()),
                );
            }
        })
        .await
        .unwrap();
}
