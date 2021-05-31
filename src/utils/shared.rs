use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use std::io::Result;

pub type Tx = mpsc::UnboundedSender<String>;
pub type Rx = mpsc::UnboundedReceiver<String>;

pub struct Shared {
    pub peers: HashMap<SocketAddr, Tx>,
    pub name: HashMap<String, SocketAddr>,
}

impl Shared {
    pub fn new() -> Self {
        Shared {
            peers: HashMap::new(),
            name: HashMap::new(),
        }
    }

    pub async fn broadcast(&mut self, sender: &SocketAddr, message: &str) {
        for peer in self.peers.iter() {
            if peer.0 != sender {
                peer.1.send(message.to_string()).expect("failed to breadcast the message");
            }
        }
    }
}