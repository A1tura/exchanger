
mod transport;

use tokio::net::{TcpListener};

use engine::{engine::Engine, events::Event};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:1337").await.unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(1024);

    tokio::spawn(async move {
        let mut engine = Engine::new();
        engine.new_book("INTC".to_string());
        while let Some(req) = rx.recv().await {
            let _ = engine.handle_event(req);
        }
    });

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let tx_clone = tx.clone();
        tokio::spawn(async move {
            transport::handle_connection(&mut socket, tx_clone).await;
        });
    }
}
