use serde::{Deserialize};
use super::DashboardItem;

#[derive(Deserialize, Clone, Debug)]
pub struct Details {
    pub num_item_groups: u32,
    pub total_items: u32,
    pub items: Vec<DashboardItem>
}
