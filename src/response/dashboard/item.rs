use crate::SKU;
use serde::{Serialize, Deserialize};
use tf2_sku::tf2_enum::Quality;

/// A dashboard item.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// The SKU.
    pub sku: SKU,
    /// The full SKU (including attributes such as paint, sheen, etc.).
    pub full_sku: SKU,
    /// The name of the item.
    pub name: String,
    /// The defindex of the item.
    pub defindex: i32,
    /// The quality of the item.
    pub quality: Quality,
    /// The number of items for sale.
    pub num_for_sale: i32,
    /// The price of the item, if set.
    pub price: Option<i32>,
}