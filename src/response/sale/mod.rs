mod item;

pub use item::Item as SaleItem;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sale {
    pub id: String,
    pub earned_credit: u32,
    pub paid: bool,
    pub time: u32,
    pub items: Vec<SaleItem>,
}
