use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Item {
    pub sku: String,
    pub full_sku: String,
    pub name: String,
    pub defindex: i32,
    pub quality: u8,
    pub num_for_sale: u16,
    pub price: Option<u32>,
}