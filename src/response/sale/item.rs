use serde::{Serialize, Deserialize};
use crate::SKU;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub original_id: u64,
    pub name: String,
    pub price: i32,
    pub sku: SKU,
    pub full_sku: Option<SKU>,
}