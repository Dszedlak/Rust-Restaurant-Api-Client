use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: i64,
    pub preparation_time: u32,
    pub price_yen: u32,
    pub name: String,
}

pub struct Order {
    pub id: i64,
    pub table_session_id: i64,
    pub timestamp: String,
    pub order_items: Vec<OrderItem>
}

pub struct OrderItem {
    pub item_id: i64,
    pub amount: u8
}

pub struct TableSession {
    pub id: i64,
    pub table_nr: u8,
    pub customers: u8,
    pub session_start: String,
    pub session_end: String,
    pub active: bool
}