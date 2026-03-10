use super::responses::{Snapshot as ResponseSnapshot, Symbol};
use crate::Snapshot;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

type SharedSnapshots = Arc<RwLock<HashMap<u32, Snapshot>>>;
type SharedSymbols = Arc<RwLock<HashMap<String, u32>>>;

#[derive(Clone)]
struct SharedType {
    snapshots: SharedSnapshots,
    symbols: SharedSymbols,
}

async fn get_snapshot(
    Path(symbol_id): Path<u32>,
    State(shared_type): State<SharedType>,
) -> Json<ResponseSnapshot> {
    let snapshot = {
        let read = shared_type.snapshots.read().await;
        read.get(&symbol_id).cloned()
    };
    match snapshot {
        Some(snapshot) => Json::from(ResponseSnapshot::from(snapshot)),
        None => Json::from(ResponseSnapshot {
            bids: Vec::new(),
            asks: Vec::new(),
        }),
    }
}

async fn get_symbol_id(
    Path(symbol): Path<String>,
    State(shared_type): State<SharedType>,
) -> Json<u32> {
    let symbol_entry = {
        let read = shared_type.symbols.read().await;
        read.get(&symbol).cloned()
    };
    match symbol_entry {
        Some(s) => Json(s),
        None => Json(0),
    }
}

async fn get_symbols(State(shared_type): State<SharedType>) -> Json<HashMap<String, u32>> {
    let symbols = {
        let read = shared_type.symbols.read().await;
        read.clone()
    };

    return Json(symbols)
}

pub async fn run(snapshots: SharedSnapshots, symbols: SharedSymbols) {
    let shared_type = SharedType { snapshots, symbols };
    let app = Router::new()
        .route("/snapshot/{symbol_id}", get(get_snapshot))
        .route("/symbols", get(get_symbols))
        .route("/symbol/{symbol}", get(get_symbol_id))
        .with_state(shared_type);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
