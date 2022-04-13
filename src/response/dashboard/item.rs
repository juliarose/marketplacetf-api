use serde::{Serialize, Deserialize};
use crate::SKU;
use marketplace_sku::tf2_enum::Quality;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub sku: SKU,
    pub full_sku: SKU,
    pub name: String,
    pub defindex: i32,
    pub quality: Quality,
    pub num_for_sale: i32,
    pub price: Option<i32>,
}