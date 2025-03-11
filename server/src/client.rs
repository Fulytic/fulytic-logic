use std::{
    net::SocketAddr,
    sync::{atomic::AtomicBool, Arc},
};

use fulytic_logic::core::PlayerInfo;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct Client {
    pub uuid: Uuid,
    pub player_info: PlayerInfo,
    pub connection_reader: Arc<Mutex<tokio::net::tcp::OwnedReadHalf>>,
    pub connection_writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
    pub address: SocketAddr,
    pub closed: AtomicBool,
}

impl Client {
    pub fn new(
        player_info: PlayerInfo,
        connection: tokio::net::TcpStream,
        address: SocketAddr,
    ) -> Self {
        let (connection_reader, connection_writer) = connection.into_split();

        Self {
            uuid: Uuid::new_v4(),
            player_info,
            connection_reader: Arc::new(Mutex::new(connection_reader)),
            connection_writer: Arc::new(Mutex::new(connection_writer)),
            address,
            closed: AtomicBool::new(false),
        }
    }

    pub fn close(&self) {
        self.closed
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(std::sync::atomic::Ordering::Relaxed)
    }
}
