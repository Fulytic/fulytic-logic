use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use bytes::BytesMut;
use fulytic_logic::{
    core::{Codec, PlayerInfo},
    Game, GameJoinC2S,
};
use tokio::{io::AsyncReadExt, sync::Mutex};

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
    stat: ClientStat,
    closed: AtomicBool,
    s2c: Arc<Mutex<BytesMut>>,
    c2s: Arc<Mutex<BytesMut>>,
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
            stat: ClientStat::Waiting,
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

    pub async fn poll_connection(&self) -> bool {
        loop {
            if self.is_closed() {
                return false;
            }
            let mut c2s = self.c2s.lock().await;

            if !c2s.is_empty() {
                match &self.stat {
                    ClientStat::Waiting => {
                        let mut c2s = c2s.split();
                        match GameJoinC2S::decode(&mut c2s) {
                            Ok((packet, _)) => {}
                            Err(err) => match err {
                                bincode::error::DecodeError::UnexpectedEnd { .. } => {}
                                _ => {
                                    self.close();
                                    return false;
                                }
                            },
                        }
                    }
                    ClientStat::Playing(game) => {}
                }
            }

            match self
                .connection_reader
                .lock()
                .await
                .read_buf(&mut *c2s)
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
