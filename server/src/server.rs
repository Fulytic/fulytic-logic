use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::client::Client;

pub struct Server {
    clients: Arc<Mutex<HashMap<Uuid, Arc<Client>>>>,
}

impl Server {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            clients: Default::default(),
        })
    }

    pub async fn add_client(&self, client: Arc<Client>) {
        self.clients.lock().await.insert(client.uuid, client);
    }
}
