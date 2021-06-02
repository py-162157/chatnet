use tokio::sync::{mpsc, Mutex};
use std::{io, error::Error, net::SocketAddr};
use tokio::net::TcpStream;
use super::super::utils::{Rx, Tx, Shared}; 
use std::sync::Arc;
use tokio_util::codec::{Framed, LinesCodec};
pub struct Peer {
    name: String,
    pswd: String,
    lines: Framed<TcpStream, LinesCodec>,
    pub rx: Rx,
}

impl Peer {
    pub async fn new(name: String, pswd: String, state: Arc<Mutex<Shared>>, lines: Framed<TcpStream, LinesCodec>)
        -> io::Result<Peer> {
            let addr = lines.get_ref().peer_addr()?;
            //let addr = stream.peer_addr().expect("Failed to get socket address from that TCP stream");
            let (tx, rx) = mpsc::unbounded_channel();
            let ref_name = name.as_str();
            {
                let mut state = state.lock().await;
                state.peers.insert(addr, tx);
                state.name.insert(ref_name.to_string(), addr);
            }
            Ok(
                Peer {
                    name: ref_name.to_string(), 
                    pswd, lines, rx
                }
            )
        }
}

