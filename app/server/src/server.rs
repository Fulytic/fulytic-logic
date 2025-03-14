use std::{collections::HashMap, sync::Arc};

use fulytic_logic::{
    core::{GameJoinC2S, GameJoinS2C},
    ClientStat, Game, GameCreateC2S, GameCreateS2C,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::client::Client;

pub struct Server {
    clients: Arc<RwLock<HashMap<Uuid, Arc<Client>>>>,
    games: Arc<RwLock<HashMap<Uuid, Game>>>,
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
        self.games.write().await.insert(game.id(), game);
    }

    pub async fn create_game(&self, packet: GameCreateC2S) -> GameCreateS2C {
        let game = Game::new(&packet.game_name, Uuid::new_v4());
        if let Some(game) = game {
            let id = game.id();
            self.new_game(game).await;
            GameCreateS2C::GameCreated(id)
        } else {
            GameCreateS2C::InvalidGameName
        }
    }

    pub async fn join_game(&self, packet: GameJoinC2S) -> GameJoinS2C {
        let Some(game) = self.games.read().await.get(&packet.game_uuid).cloned() else {
            return GameJoinS2C::MissingGameId;
        };
        {
            let map = self.clients.read().await;
            let Some(client) = map.get(&packet.player.id) else {
                return GameJoinS2C::MissingPlayerInfo;
            };
            client.change_stat(ClientStat::Playing(game.clone())).await;
        }
        if let Err(err) = game.join(packet.player).await {
            return err;
        };
        let Some(raw_data) = game.raw_data().await else {
            return GameJoinS2C::ServerError;
        };
        GameJoinS2C::RawGameData(raw_data)
    }
}
