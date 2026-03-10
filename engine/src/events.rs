use order_book::{
    order::{OrderReq, Price},
    order_book::{OrderBookErrors, order_book::Snapshot},
};

#[derive(Debug)]
pub enum Event {
    NewOrder {
        symbol_id: u32,
        order_req: OrderReq,
    },
    CancelOrder {
        symbol_id: u32,
        order_id: u32,
        client_id: u32,
    },
    GetSnapshot {
        symbol_id: u32,
        depth: Option<usize>,
        client_id: u32,
    },
}

#[derive(Debug, Clone)]
pub enum EngineEvent {
    OrderAccepted {
        symbol_id: u32,
        client_id: u32,
        order_id: u32,
    },
    OrderCancelled {
        symbol_id: u32,
        client_id: u32,
        order_id: u32,
    },
    OrderPartiallyFilled {
        symbol_id: u32,
        client_id: u32,
        order_id: u32,
        remaining: u32,
    },
    OrderFilled {
        symbol_id: u32,
        client_id: u32,
        order_id: u32,
    },
    Trade {
        symbol_id: u32,
        maker_client_id: u32,
        maker_order_id: u32,
        taker_client_id: u32,
        taker_order_id: u32,
        price: Price,
        quantity: u32,
    },
    BookSnapshot {
        symbol_id: u32,
        client_id: u32,
        snapshot: Snapshot,
    },
    PriceLevel {
        symbol_id: u32,
        side: u8,
        price: f64,
        quantity: u32,
    },
    NewSymbol {
        symbol_id: u32,
        ticker: [u8; 16],
    }
}

#[derive(Debug)]
pub enum EngineError {
    InvalidBook,
    OrderBookError(OrderBookErrors),
}

impl EngineEvent {
    pub fn get_client_id(&self) -> Option<u32> {
        match self {
            Self::OrderAccepted { client_id, .. } => return Some(*client_id),
            Self::OrderCancelled { client_id, .. } => return Some(*client_id),
            Self::OrderPartiallyFilled { client_id, .. } => return Some(*client_id),
            Self::BookSnapshot { client_id, .. } => return Some(*client_id),
            Self::OrderFilled { client_id, .. } => return Some(*client_id),
            Self::Trade { .. } => return None,
            Self::PriceLevel { .. } => return None,
            Self::NewSymbol { .. } => None,
        }
    }
}
