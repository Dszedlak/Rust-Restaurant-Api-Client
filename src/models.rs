use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub id: i64,
    pub preparation_time: u32,
    pub price_yen: u32,
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Order {
    pub id: i64,
    pub table_session_id: i64,
    pub timestamp: String,
    pub order_items: Vec<OrderItem>
}
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderItem {
    pub item_id: i64,
    pub amount: u8
}
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct TableSession {
    pub id: i64,
    pub table_nr: u8,
    pub customers: u8,
    pub session_start: String,
    #[serde(skip_deserializing)]
    pub session_end: String,
    pub active: bool
}

#[derive(Default)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TableSessionOut {
    pub customers: u8,
    pub session_start: String,
    pub active: bool
}

impl TableSessionOut {
    pub fn new() -> Self {
        TableSessionOut { customers: (0), session_start: ("".to_string()), active: (false) }
    }
}