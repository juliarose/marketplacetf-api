use serde::{Serialize, Deserialize};
use super::DashboardItem;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Details {
    pub num_item_groups: u32,
    pub total_items: u32,
    pub items: Vec<DashboardItem>
}
