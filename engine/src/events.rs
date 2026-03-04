use order_book::{order::{OrderReq, Price}, order_book::{OrderBookErrors, order_book::Snapshot}};

#[derive(Debug)]
pub enum Event {
    NewOrder { symbol: String, order_req: OrderReq },
    CancelOrder { symbol: String, order_id: u32 },
    GetSnapshot { symbol: String, depth: Option<usize> },
}

#[derive(Debug)]
pub enum EngineEvent {
    OrderAccepted { order_id: u32 },
    OrderCancelled { order_id: u32 },
    OrderPartiallyFilled { order_id: u32, remaining: u32 },
    OrderFilled { order_id: u32 },
    Trade { maker_order_id: u32, taker_order_id: u32, price: Price, quantity: u32 },
    BookSnapshot { snapshot: Snapshot },
}

#[derive(Debug)]
pub enum EngineError {
    InvalidBook,
    OrderBookError(OrderBookErrors),
}
