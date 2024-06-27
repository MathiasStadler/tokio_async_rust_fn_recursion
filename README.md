# explain / explore fn call from parent call

## install/check cargo-edit for command ```cargo add```

```bash
cargo add cargo-edit
```

## init project

```bash
cd # go to the home folder
pwd # check i'm inside the home folder
mkdir tokio_async_rust_fn_recursion && cd $_ # create and change into the project folder
cargo init . # initialize the project
cargo add log # logging is always helpful
cargo add env_logger # nice log output of console
cargo add tokio --features full # add tokio with all default features
# update all packages of the latest version
cargo update --verbose
# show package dependency
cargo tree  --workspace

```

## [hello_world.rs](https://github.com/tokio-rs/tokio/blob/master/examples/hello_world.rs)

```rust
//! A simple client that opens a TCP stream, writes "hello world\n", and closes
//! the connection.
//!
//! To start a server that this client can talk to on port 6142, you can use this command:
//!
//!     ncat -l 6142
//!
//! And then in another terminal run:
//!
//!     cargo run --example hello_world

#![warn(rust_2018_idioms)]

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    println!("created stream");

    let result = stream.write_all(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());

    Ok(())
}
```

## my simplest async

```rust
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
```

## [Why Async? inside Rust](https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html)
