use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use fulytic_logic::{
    core::{BufQueue, GameJoinC2S, PlayerInfo},
    ClientStat,
};
use tokio::{
    io::AsyncReadExt,
    net::tcp::OwnedReadHalf,
    sync::{Mutex, RwLock},
};

use crate::server::Server;

pub struct Client {
    pub player_info: Arc<PlayerInfo>,
    pub address: SocketAddr,
    stat: RwLock<ClientStat>,
    closed: AtomicBool,
    pub s2c: Arc<Mutex<BufQueue>>,
    s2c_sender: tokio::sync::mpsc::Sender<()>,
    c2s: Arc<Mutex<BufQueue>>,
}

impl Client {
    // TODO: generate uuid on server, and response to client
    pub fn new(
        s2c_sender: tokio::sync::mpsc::Sender<()>,
        player_info: PlayerInfo,
        address: SocketAddr,
    ) -> Arc<Self> {
        Arc::new(Self {
            player_info: Arc::new(player_info),
            address,
            stat: RwLock::new(ClientStat::Waiting),
            closed: AtomicBool::new(false),
            s2c: Default::default(),
            s2c_sender,
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

    pub async fn poll_connection(
        &self,
        server: &Server,
        connection_reader: Arc<Mutex<OwnedReadHalf>>,
    ) {
        loop {
            if self.is_closed() {
                return;
            }
            let mut c2s = self.c2s.lock().await;
            if c2s.is_missed() {
                self.close();
                return;
            }

            if !c2s.is_empty() {
                match self.stat.read().await.clone() {
                    ClientStat::Waiting => match c2s.decode::<GameJoinC2S>() {
                        Ok(packet) => {
                            if packet.player != *self.player_info {
                                self.close();
                                return;
                            }
                            log::info!("joining game {:#?}", packet);
                            let s2c_packet = server.join_game(packet).await;
                            log::info!("joined game s2c packet: {:#?}", s2c_packet);
                            self.s2c.lock().await.encode(s2c_packet);
                            let _ = self.s2c_sender.send(()).await;
                        }
                        Err(true) => {
                            self.close();
                            return;
                        }
                        Err(false) => {}
                    },
                    ClientStat::Playing(game) => {
                        game.decode_c2s_packet(
                            c2s.split(),
                            self.s2c_sender.clone(),
                            self.player_info.clone(),
                            self.s2c.clone(),
                        );
                    }
                }
            }

            c2s.reserve(1024);

            match connection_reader.lock().await.read_buf(c2s.mut_buf()).await {
                Ok(0) => {
                    self.close();
                    return;
                }
                Ok(_) => {}
                Err(_) => {
                    self.close();
                    return;
                }
            }
        }
    }
}
