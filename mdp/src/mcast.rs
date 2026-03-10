use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use order_book::order::{Price, Side};

use protocol::Message;
use tokio::sync::RwLock;

use crate::{Snapshot, utils};

pub async fn run(
    snapshots: Arc<RwLock<HashMap<u32, Snapshot>>>,
    symbols: Arc<RwLock<HashMap<String, u32>>>,
) {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:1338").await.unwrap();
    socket
        .join_multicast_v4("224.0.1.1".parse().unwrap(), "0.0.0.0".parse().unwrap())
        .unwrap();
    let mut buf = [0u8; 4096];
    loop {
        let (len, _) = socket.recv_from(&mut buf).await.unwrap();
        let message =
            protocol::Message::decode(buf[0], &mut bytes::BytesMut::from(&buf[1..len])).unwrap();
        match message {
            Message::PriceLevel(price_level) => {
                if price_level.quantity == 0 {
                    continue;
                }

                let mut write = snapshots.write().await;
                let snapshot = write
                    .entry(price_level.symbol_id)
                    .or_insert_with(|| Snapshot {
                        bids: BTreeMap::new(),
                        asks: BTreeMap::new(),
                    });

                let levels = if price_level.side == Side::Bid as u8 {
                    &mut snapshot.bids
                } else {
                    &mut snapshot.asks
                };

                levels.insert(Price::from(price_level.price), price_level.quantity);
                // lock dropped here at end of scope
            }
            Message::NewSymbol(symbol) => {
                let mut write = symbols.write().await;
                write.insert(utils::bytes_to_symbol(&symbol.ticker), symbol.symbol_id);
            }
            _ => continue,
        }
    }
}
