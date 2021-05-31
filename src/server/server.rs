use tokio::net::{TcpStream, TcpListener};
use std::{error::Error, io::{self, Read}, net::SocketAddr};
use super::super::{utils::Shared, client::{Peer, process}};
use tokio::sync::{Mutex};
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Please input the address on which your server running: ");
    let mut socket_string = String::new();
    io::stdin().read_to_string(&mut socket_string)?;
    let socket = socket_string.as_str();
    let listener = TcpListener::bind(socket).await?;
    println!("server is running on {}", socket);
    let state = Arc::new(Mutex::new(Shared::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            println!("Successfully accepted connection!");
            if let Err(e) = process(state, stream, addr).await {
                println!("Error occured when processing the stream, error = {:?}", e);
            }
        });
    }

    Ok(())
}