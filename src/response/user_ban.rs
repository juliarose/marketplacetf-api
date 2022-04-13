use serde::{Serialize, Deserialize};
use crate::SteamID;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BanType {
    // todo fill in other ban types
    #[serde(rename = "COMMUNITY")]
    Community,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ban {
    pub time: i32,
    pub r#type: BanType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserBan {
    pub id: i32,
    pub steamid: SteamID,
    pub name: String,
    pub banned: bool,
    pub seller: bool,
    pub ban: Option<Ban>,
}