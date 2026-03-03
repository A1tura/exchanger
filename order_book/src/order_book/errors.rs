use std::fmt::Display;


#[derive(Debug)]
pub enum OrderBookErrors {
    OrderNotExist,
}

impl std::error::Error for OrderBookErrors {}
impl Display for OrderBookErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBookErrors::OrderNotExist => {
                write!(f, "Order does not exist");
            }
        };

        Ok(())
    }
}
