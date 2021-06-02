use tokio::net::{TcpStream, TcpListener};
use std::{error::Error, io::{self, Read}, net::SocketAddr, result};
use chatnet::utils::Shared;
use chatnet::client::Peer;
use tokio::sync::{Mutex};
use std::sync::Arc;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Please input the address on which your server running: ");
    //let mut socket_string = String::new();
    //io::stdin().read_to_string(&mut socket_string)?;
    //let socket = socket_string.as_str();
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let socket = listener.local_addr().unwrap();
    println!("server is running on {}", socket);
    let state = Arc::new(Mutex::new(Shared::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            println!("Successfully accepted connection!");
            if let Err(e) = process(state, stream, addr).await {
                println!("an error occurred; error = {:?}", e);
            }
                });
    }
}

async fn try_read_line(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    lines.send("please input your username:").await?;
    let username = match lines.next().await {
        Some(Ok(line)) => line,
        _ => {
            println!("Failed to get username from {}. Client disconnected.", addr);
            return Ok(());
        }
    }

    let mut peer = Peer::new(ref_name.to_string(), pswd, state.clone(), lines).await.unwrap();
    
    let msg = format!("user {} has joined the chat", ref_name.to_string());
    state.lock().await.broadcast(&addr, &msg).await;
    println!("broadcast over");

    loop {
        let mut msg_send = String::default();

        tokio::select! {
            Some(msg_recv) = peer.rx.recv() => {
                peer.stream.
            }

            result = try_read_line(&mut msg_send) => match result {
                Ok(n) => {
                    let mut state = state.lock().await;
                    let msg = format!("{}: {}", ref_name, msg_send);

                    state.broadcast(&addr, msg.as_str()).await;
                }

                Err(e) => {
                    println!("an error occurred while processing messages for {}; error = {:?}", ref_name.to_string(), e);
                    break;
                }
            }
        }
    }

    {
        let mut state = state.lock().await;
        state.peers.remove(&addr);
        state.name.remove(&name);
        let msg = format!("{} has left the chat", ref_name.to_string());
        state.broadcast(&addr, msg.as_str()).await;
    }
    
    Ok(())
}