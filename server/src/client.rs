use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use bytes::{Buf, BytesMut};
use fulytic_logic::{
    core::{BufQueue, Codec, PlayerInfo},
    Game, GameJoinC2S,
};
use tokio::{
    io::AsyncReadExt,
    sync::{Mutex, RwLock},
};

use crate::server::Server;

#[derive(Debug, Clone)]
pub enum ClientStat {
    Waiting,
    Playing(Arc<Game>),
}

pub struct Client {
    pub player_info: PlayerInfo,
    connection_reader: Arc<Mutex<tokio::net::tcp::OwnedReadHalf>>,
    connection_writer: Arc<Mutex<tokio::net::tcp::OwnedWriteHalf>>,
    pub address: SocketAddr,
    stat: RwLock<ClientStat>,
    closed: AtomicBool,
    s2c: Arc<Mutex<BufQueue>>,
    c2s: Arc<Mutex<BufQueue>>,
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
            stat: RwLock::new(ClientStat::Waiting),
            closed: AtomicBool::new(false),
            s2c: Default::default(),
            c2s: Default::default(),
        })
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::Relaxed);
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }

    pub async fn change_stat(&self, stat: ClientStat) {
        *self.stat.write().await = stat;
    }

    pub async fn poll_connection(&self, server: &Server) -> bool {
        loop {
            if self.is_closed() {
                return false;
            }
            let mut c2s = self.c2s.lock().await;

            match self.stat.read().await.clone() {
                ClientStat::Waiting => match c2s.decode::<GameJoinC2S>() {
                    Ok(packet) => {
                        if packet.player != self.player_info {
                            self.close();
                            return false;
                        }
                        let game = server.join_game(packet).await;
                        let Some((game, packet)) = game else {
                            self.close();
                            return false;
                        };
                        self.change_stat(ClientStat::Playing(game)).await;
                        self.s2c.lock().await.encode(packet);
                    }
                    Err(true) => {
                        self.close();
                        return false;
                    }
                    Err(false) => {}
                },
                ClientStat::Playing(game) => {}
            }

            match self
                .connection_reader
                .lock()
                .await
                .read_buf(c2s.mut_buf())
                .await
            {
                Ok(0) => {
                    self.close();
                    return false;
                }
                Ok(_) => {}
                Err(_) => {
                    self.close();
                    return false;
                }
            }
        }
    }
}
