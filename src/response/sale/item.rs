use crate::SKU;
use serde::{Serialize, Deserialize};

/// An item from a sale.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// The ID of the item.
    pub id: u64,
    /// The original ID of the item.
    pub original_id: u64,
    /// The name of the item.
    pub name: String,
    /// The sale price (after fees).
    pub price: i32,
    /// The SKU of the item.
    pub sku: SKU,
    /// The full SKU of the item (if it differs from the SKU).
    pub full_sku: Option<SKU>,
}