use crate::SteamID;
use serde::{Serialize, Deserialize};

/// The type of ban.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BanType {
    /// Community ban type.
    #[serde(rename = "COMMUNITY")]
    Community,
}

/// A ban.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ban {
    /// The time the user was banned.
    pub time: i32,
    /// The type of ban.
    pub r#type: BanType,
}

/// Details about a user.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The ID of the user.
    pub id: i32,
    /// The Steam ID of the user.
    pub steamid: SteamID,
    /// The name of the user.
    pub name: String,
    /// Whether the user is banned.
    pub banned: bool,
    /// Whether the user is a seller.
    pub seller: bool,
    /// The ban.
    pub ban: Option<Ban>,
}