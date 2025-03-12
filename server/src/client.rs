use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use fulytic_logic::core::PlayerInfo;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ClientStat {
    Waiting,
    Playing,
}

pub struct Client {
    pub player_info: PlayerInfo,
    connection_reader: Arc<Mutex<tokio::net::tcp::OwnedReadHalf>>,
    connection_writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
    pub address: SocketAddr,
    closed: AtomicBool,
}

impl Client {
    // TODO: generate uuid on server, and response to client
    pub fn new(
        player_info: PlayerInfo,
        connection: tokio::net::TcpStream,
        address: SocketAddr,
    ) -> Arc<Self> {
        let (connection_reader, connection_writer) = connection.into_split();

        Arc::new(Self {
            player_info,
            connection_reader: Arc::new(Mutex::new(connection_reader)),
            connection_writer: Arc::new(Mutex::new(connection_writer)),
            address,
            closed: AtomicBool::new(false),
        })
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::Relaxed);
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }

    pub async fn poll_connection(&self) -> bool {
        loop {
            if self.is_closed() {
                return false;
            }
        }
    }
}
