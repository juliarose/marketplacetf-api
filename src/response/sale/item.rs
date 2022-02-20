use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Item {
    pub id: u64,
    pub original_id: u64,
    pub name: String,
    pub price: u32,
    pub sku: String,
    pub full_sku: Option<String>,
}