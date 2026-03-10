use serde::{Serialize, Deserialize};

use super::Snapshot as RawSnapshot;

#[derive(Serialize,  Deserialize)]
pub struct Level {
    pub price: f64,
    pub quantity: u32
}

#[derive(Serialize,  Deserialize)]
pub struct Snapshot {
    pub bids: Vec<Level>,
    pub asks: Vec<Level>
}

impl From<RawSnapshot> for Snapshot {
    fn from(value: RawSnapshot) -> Self {
        let mut snapshot = Snapshot { bids: Vec::new(), asks: Vec::new() };

        for ask in value.asks {
            let level = Level { price: ask.0.as_float(), quantity: ask.1 };
            snapshot.asks.push(level);
        }

        for bid in value.bids {
            let level = Level { price: bid.0.as_float(), quantity: bid.1 };
            snapshot.bids.push(level);
        }

        return snapshot;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Symbol {
    pub symbol_id: u32,
    pub ticker: String,
}
