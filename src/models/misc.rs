use serde::{Deserialize, Serialize};

use super::EncounterPreview;

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPastEncounterResult {
    pub items: Vec<EncounterPreview>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatsResult {
    pub class_popularity: Vec<Metric>,
    pub item_level_breakdown: Vec<Metric>,
    pub server_population: ServerPopulation,
    pub metrics: Vec<Metric>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerPopulation {
    pub na: NorthAmericaNode,
    pub eu: EuropeNode
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NorthAmericaNode {
    pub name: String,
    pub naw: Vec<Metric>,
    pub nae: Vec<Metric>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EuropeNode {
    pub name: String,
    pub metrics: Vec<Metric>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    pub name: String,
    pub value: f32
}