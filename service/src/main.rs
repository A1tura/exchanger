mod router;
mod session;
mod transport;
mod utils;

use std::sync::{Arc};
use tokio::sync::RwLock;

use router::Router;

use tokio::net::TcpListener;

use engine::{
    engine::Engine,
    events::{EngineEvent, Event},
};

use crate::{router::SharedRouter, transport::Connection};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:1337").await.unwrap();
    let mut router: SharedRouter = Arc::new(RwLock::new(Router::new()));

    let (order_book_tx, mut order_book_rx) = tokio::sync::mpsc::channel::<Event>(1024);

    let mut router_clone = router.clone();
    tokio::spawn(async move {
        let mut engine = Engine::new();
        engine.new_book("INTC".to_string());
        while let Some(req) = order_book_rx.recv().await {
            let events = engine.handle_event(req).unwrap();
            let mut router = router_clone.write().await;
            router.route_events(events).await;
        }
    });

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let order_book_tx_clone = order_book_tx.clone();

        let mut router = router.write().await;
        let client = router.add_client();
        tokio::spawn(async move {
            let mut connection = Connection::new(client, socket, order_book_tx_clone);
            connection.handle_connection().await;
        });
    }
}
