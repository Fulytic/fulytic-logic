use std::{collections::HashMap, sync::Arc};

use fulytic_logic::Game;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::client::Client;

pub struct Server {
    clients: Arc<RwLock<HashMap<Uuid, Arc<Client>>>>,
    games: Arc<RwLock<HashMap<Uuid, Arc<Game>>>>,
}

impl Server {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            clients: Default::default(),
            games: Default::default(),
        })
    }

    pub async fn add_client(&self, client: Arc<Client>) {
        self.clients
            .write()
            .await
            .insert(client.player_info.id, client);
    }

    pub async fn remove_client(&self, uuid: Uuid) {
        self.clients.write().await.remove(&uuid);
    }

    pub async fn new_game(&self, game: Game) {
        self.games.write().await.insert(game.id(), Arc::new(game));
    }
}
