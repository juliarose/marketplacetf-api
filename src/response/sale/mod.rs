mod item;

pub use item::Item;

use serde::{Serialize, Deserialize};

/// A sale.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sale {
    /// The ID of the sale.
    pub id: String,
    /// The earned credit.
    pub earned_credit: u32,
    /// Whether the sale was paid.
    pub paid: bool,
    /// The time of the sale as a UNIX timestamp.
    pub time: u32,
    /// The items in the sale.
    pub items: Vec<Item>,
}
