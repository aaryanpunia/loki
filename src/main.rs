use std::net::SocketAddr;

use errors::LokiError;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

pub mod errors;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8854").await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        let _ =
            tokio::spawn(
                async move { main_error_handler(process_socket(stream, addr).await).await },
            )
            .await;
    }
}

async fn main_error_handler(result: Result<(), LokiError>) {
    match result {
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

async fn process_socket(mut stream: TcpStream, addr: SocketAddr) -> Result<(), LokiError> {
    println!("processing socket: {addr}");
    let written = stream.write(b"hi").await?;
    dbg!(written);
    Ok(())
}
