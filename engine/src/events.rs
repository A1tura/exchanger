use order_book::{order::{OrderReq, Price}, order_book::{OrderBookErrors, order_book::Snapshot}};

#[derive(Debug)]
pub enum Event {
    NewOrder { symbol: String, order_req: OrderReq },
    CancelOrder { symbol: String, order_id: u32, client_id: u32},
    GetSnapshot { symbol: String, depth: Option<usize>, client_id: u32 },
}

#[derive(Debug)]
pub enum EngineEvent {
    OrderAccepted { client_id: u32, order_id: u32 },
    OrderCancelled { client_id: u32, order_id: u32 },
    OrderPartiallyFilled { client_id: u32, order_id: u32, remaining: u32 },
    OrderFilled { client_id: u32, order_id: u32 },
    Trade { maker_client_id: u32, maker_order_id: u32, taker_client_id: u32, taker_order_id: u32, price: Price, quantity: u32 },
    BookSnapshot { client_id: u32, snapshot: Snapshot },
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
        }
    }
}
