mod api;
mod responses;
mod mcast;

mod utils;

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use order_book::order::Price;
use tokio::sync::RwLock;

use crate::api::run as run_api;
use crate::mcast::run as run_mcast;

#[derive(Debug, Clone)]
struct Snapshot {
    pub bids: BTreeMap<Price, u32>,
    pub asks: BTreeMap<Price, u32>,
}

#[tokio::main]
async fn main() {
    let snapshots: Arc<RwLock<HashMap<u32, Snapshot>>> = Arc::new(RwLock::new(HashMap::new()));
    let symbols: Arc<RwLock<HashMap<String, u32>>> = Arc::new(RwLock::new(HashMap::new()));

    let mcast = tokio::spawn(run_mcast(snapshots.clone(), symbols.clone()));
    let api = tokio::spawn(run_api(snapshots.clone(), symbols.clone()));

    let _ = tokio::join!(mcast, api);
}
