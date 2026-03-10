use std::io::Write;

use engine::events::EngineEvent;
use tokio::sync::broadcast;

use crate::utils;

pub async fn run(feed_tx: broadcast::Sender<Vec<EngineEvent>>) {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let multicast_addr = "224.0.1.1:1338";

    let mut rx = feed_tx.subscribe();

    while let Ok(events) = rx.recv().await {
        for event in events {
            let (event_bytes, event_type) = &utils::engine_event_as_bytes(&event);
            let mut buf: Vec<u8> = Vec::new();


            buf.push(*event_type as u8);
            let _ = buf.write_all(event_bytes);
            let _ = socket.send_to(buf.as_slice(), multicast_addr).await;
        }
    }
}
