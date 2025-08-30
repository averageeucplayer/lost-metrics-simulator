use std::hash::Hash;
use serde::{Deserialize, Serialize};
use strum_macros::VariantArray;
use strum_macros::{AsRefStr, EnumString};

#[derive(Default, Debug, Serialize, Deserialize, Copy, Hash, Clone, PartialEq, Eq, AsRefStr, EnumString, VariantArray)]
#[repr(u32)]
pub enum Class {
    #[default]
    Unknown = 0,
    #[strum(serialize = "Warrior (Male)")]
    WarriorMale = 101,
    Berserker = 102,
    Destroyer = 103,
    Gunlancer = 104,
    Paladin = 105,
    #[strum(serialize = "Warrior (Female)")]
    WarriorFemale = 111,
    Slayer = 112,
    Valkyrie = 113,
    Mage = 201,
    Arcanist = 202,
    Summoner = 203,
    Bard = 204,
    Sorceress = 205,
    #[strum(serialize = "Martial Artist (Female)")]
    MartialArtistFemale = 301,
    Wardancer = 302,
    Scrapper = 303,
    Soulfist = 304,
    Glaivier = 305,
    #[strum(serialize = "Martial Artist (Male)")]
    MartialArtistMale = 311,
    Striker = 312,
    Breaker = 313,
    Assassin = 401,
    Deathblade = 402,
    Shadowhunter = 403,
    Reaper = 404,
    Souleater = 405,
    #[strum(serialize = "Gunner (Male)")]
    GunnerMale = 501,
    Sharpshooter = 502,
    Deadeye = 503,
    Artillerist = 504,
    Machinist = 505,
    #[strum(serialize = "Gunner (Female)")]
    GunnerFemale = 511,
    Gunslinger = 512,
    Specialist = 601,
    Artist = 602,
    Aeromancer = 603,
    Wildsoul = 604,
}

impl From<Class> for u32 {
    fn from(class: Class) -> Self {
        class as u32
    }
}
