use serde::{Serialize, Deserialize};
use crate::SKU;
use crate::tf2_enum::Quality;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub sku: SKU,
    pub full_sku: SKU,
    pub name: String,
    pub defindex: i32,
    pub quality: Quality,
    pub num_for_sale: i32,
    pub price: Option<i32>,
}