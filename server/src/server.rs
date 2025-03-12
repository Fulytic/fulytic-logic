use std::{collections::HashMap, sync::Arc};

use fulytic_logic::{Game, GameJoinC2S, GameJoinS2C};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::client::{Client, ClientStat};

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

    pub async fn join_game(&self, packet: GameJoinC2S) -> Option<(Arc<Game>, GameJoinS2C)> {
        let game = self.games.read().await.get(&packet.game_uuid)?.clone();
        {
            let map = self.clients.read().await;
            let client = map.get(&packet.player.id)?;
            client.change_stat(ClientStat::Playing(game.clone())).await;
        }
        game.join(packet.player).await.ok()?;
        let raw_data = game.raw_data().await?;
        Some((game, GameJoinS2C::RawGameData(raw_data)))
    }
}
