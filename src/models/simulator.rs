use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSimulation {
    pub parties: Vec<Party>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub dps: Vec<Dps>,
    pub support: Support
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dps {
    pub name: String,
    pub class: u32,
    pub attack_power: i64,
    pub crit_rate: f32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Support {
    pub name: String,
    pub class: u32,
    pub attack_power: i64,
    pub crit_rate: f32,
}
