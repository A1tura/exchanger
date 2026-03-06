use std::{collections::HashMap, sync::{Arc}};
use tokio::sync::RwLock;

use engine::events::{EngineEvent, Event};
use tokio::sync::mpsc::{Receiver, Sender, channel};

pub struct Client {
    pub client_id: u32,
    pub events_rx: Option<Receiver<EngineEvent>>,
}

#[derive(Clone)]
pub struct Router {
    clients: HashMap<u32, Sender<EngineEvent>>,
}

pub type SharedRouter = Arc<RwLock<Router>>;

impl Router {
    pub fn new() -> Self {
        return Router {
            clients: HashMap::new(),
        };
    }

    pub fn add_client(&mut self) -> Client {
        let client_id = rand::random::<u32>();
        let (events_tx, events_rx) = channel::<EngineEvent>(1024);

        self.clients.insert(client_id, events_tx);

        return Client {
            client_id,
            events_rx: Some(events_rx),
        };
    }

    pub async fn route_events(&mut self, events: Vec<EngineEvent>) {
        for event in events {
            if let Some(client_id) = event.get_client_id() {
                let sender = self.clients.get_mut(&client_id);
                if let Some(sender) = sender {
                    let _ = sender.send(event).await;
                    continue;
                }

                if event.get_client_id().is_none() {
                    self.send_to_all(event).await;
                }
            }
        }
    }

    async fn send_to_all(&mut self, event: EngineEvent) {
        for client in &self.clients {
            let event = event.clone();
            let _ = client.1.send(event).await;
        }
    }
}
