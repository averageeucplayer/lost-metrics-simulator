use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::FormattedValue;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetSimulationTemplateCriteriaArgs {
    pub criteria: GetSimulationTemplateCriteria
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetSimulationTemplateCriteria {
    pub name: String
}

pub struct Damage {
    pub value: i64,
    pub skill_id: u32,
    pub hit_flag: u32,
    pub is_critical: bool,
}


#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: u64,
    pub is_support: bool,
    pub name: String,
    pub class_id: u32,
    pub class_name: String,
    pub stats: PlayerStats
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    pub total_damage: FormattedValue,
    pub total_damage_percentage_to_party: f32,
    pub total_damage_percentage_to_raid: f32,
    pub brand_uptime: f32,
    pub atk_power_uptime: f32,
    pub identity_uptime: f32
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Boss {
    pub id: u64,
    pub name: String,
    pub current_hp: u64,
    pub hp: u64,
    pub damage_taken: u64
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub id: u64,
    pub member_ids: HashSet<u64>,
    pub total_damage: FormattedValue,
    pub total_damage_percentage_to_raid: f32,
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterPreview {
    pub id: Uuid,
    pub name: String,
    pub participants: Vec<PlayerSlim>,
    pub started_on: DateTime<Utc>,
    pub duration: String,
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSlim {
    pub id: u64,
    pub name: String,
    pub class_id: u32,
    pub class_name: String,
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterStats {
    pub total_damage: FormattedValue,
}


#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub id: Uuid,
    pub name: String,
    pub duration: Option<i64>,
    pub started_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub parties: Vec<Party>,
    pub participants: Vec<Player>,
    pub boss: Boss,
    pub stats: EncounterStats,
}


