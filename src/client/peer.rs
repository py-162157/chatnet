use tokio::sync::{mpsc, Mutex};
use std::{io, error::Error, net::SocketAddr};
use tokio::net::TcpStream;
use super::super::utils::{Rx, Tx, Shared}; 
use std::sync::Arc;

pub struct Peer {
    name: String,
    pswd: String,
    stream: TcpStream,
    pub rx: Rx,
}

impl Peer {
    pub async fn new(name: String, pswd: String, state: Arc<Mutex<Shared>>, stream: TcpStream)
        -> io::Result<Peer> {
            let addr = stream.peer_addr().expect("Failed tp get socket address from that TCP stream");
            let (tx, rx) = mpsc::unbounded_channel();
            let ref_name = name.as_str();
            {
                let mut state = state.lock().await;
                state.peers.insert(addr, tx).expect("Failed to insert socket address to hashmap");
                state.name.insert(ref_name.to_string(), addr);
            }
            Ok(
                Peer {
                    name: ref_name.to_string(), 
                    pswd, stream, rx
                }
            )
        }
}

async fn try_read_line(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

pub async fn process(
    state: Arc<Mutex<Shared>>, 
    stream: TcpStream, 
    addr: SocketAddr
) -> Result<(), Box<dyn Error>> {
    println!("Please enter your username:");
    let mut name = String::default();
    io::stdin().read_line(&mut name).expect("Failed to get name from input");
    let ref_name = name.as_str();

    println!("Please enter your password:");
    let mut pswd = String::default();
    io::stdin().read_line(&mut pswd).expect("Failed to get password from input");

    let mut peer = Peer::new(ref_name.to_string(), pswd, state.clone(), stream).await?;
    
    let msg = format!("user {} has joined the chat", ref_name.to_string());
    state.lock().await.broadcast(&addr, &msg).await;

    loop {
        let mut msg_send = String::default();

        tokio::select! {
            Some(msg_recv) = peer.rx.recv() => {
                println!("{}", msg_recv);
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
