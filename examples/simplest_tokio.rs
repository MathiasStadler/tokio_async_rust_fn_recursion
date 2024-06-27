//  https://hackmd.io/@nikomatsakis/SJggBfQbd
#![warn(rust_2018_idioms)]

// use tokio::io::AsyncWriteExt;
// use tokio::net::TcpStream;

use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    println!("empty body");

    Ok(())
}

// cargo run --example simplest_tokio