mod order;
mod order_book;

use order_book::OrderBook;
use order::{Order, Side};

fn main() {
    let mut ob = OrderBook::new();
    let order = Order::new(Side::Ask, 0.8, 100);

    ob.add_order(order);
}
