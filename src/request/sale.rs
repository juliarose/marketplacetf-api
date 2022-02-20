use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetSales {
    pub num: Option<u32>,
    pub start_before: Option<u32>,
}