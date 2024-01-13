use serde::{Serialize, Deserialize};
use crate::SteamID;

/// A marketplace.tf bot.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bot {
    /// The bot's ID.
    id: i32,
    /// The bot's Steam ID.
    steamid: SteamID,
    /// The type of bot.
    r#type: BotType,
}

/// The type of bot.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BotType {
    /// A standard bot.
    #[serde(rename = "STANDARD")]
    Standard,
    /// A bot that consolidates items.
    #[serde(rename = "CONSOLIDATION")]
    Consolidation,
}