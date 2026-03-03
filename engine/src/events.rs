use order_book::{order::OrderReq, order_book::{OrderBookErrors, order_book::Snapshot}};

#[derive(Debug)]
pub enum Event {
    NewOrder { symbol: String, order_req: OrderReq },
    CancelOrder { symbol: String, order_id: u32 },
    GetSnapshot { symbol: String, depth: Option<usize> },
}

#[derive(Debug)]
pub enum EngineEvent {
    OrderAccepted { order_id: u32 },
    BookSnapshot { snapshot: Snapshot },
    OrderCancelled { order_id: u32 },
}

#[derive(Debug)]
pub enum EngineError {
    InvalidBook,
    OrderBookError(OrderBookErrors),
}
