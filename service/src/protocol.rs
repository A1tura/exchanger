pub enum MessageType {
    CreateLimitOrder = 1,
    CreateMarketOrder = 2,
    CancelOrder = 3,
}

pub struct Header {
    pub version: u32,
    pub seq_num: u32,
    pub msg_type: u8,
}

pub enum Message {
    CreateLimitOrder { symbol: u32, side: u8, price: u32, quantity: u32 },
    CreateMarketOrder { symbol: u32, side: u8, quantity: u32 },
    CancelOrder { symbol: u32, order_id: u32 }
}
