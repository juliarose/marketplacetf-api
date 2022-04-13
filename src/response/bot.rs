use serde::{Serialize, Deserialize};
use crate::SteamID;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bot {
    id: i32,
    steamid: SteamID,
    r#type: BotType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BotType {
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "CONSOLIDATION")]
    Consolidation,
}