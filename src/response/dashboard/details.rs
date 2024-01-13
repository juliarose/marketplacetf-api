use super::DashboardItem;
use serde::{Serialize, Deserialize};

/// The details of a dashboard
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Details {
    /// The number of item groups.
    pub num_item_groups: u32,
    /// The total number of items.
    pub total_items: u32,
    /// A list of items.
    pub items: Vec<DashboardItem>
}
