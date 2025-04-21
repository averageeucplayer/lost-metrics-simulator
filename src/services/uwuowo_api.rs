use serde::{Deserialize, Serialize};

use crate::GetStatsResult;

const DATA: &str = r#"{
    "classPopularity": [
        { "name": "Souleater", "value": 50154 },
        { "name": "Paladin", "value": 47719 },
        { "name": "Artist", "value": 44996 },
        { "name": "Slayer", "value": 40533 },
        { "name": "Sorceress", "value": 39936 }
    ],
    "itemLevelBreakdown": [
        { "name": "1700+", "value": 15544 },
        { "name": "1690+", "value": 21466 },
        { "name": "1680+", "value": 51790 },
        { "name": "1670+", "value": 28110 },
        { "name": "1660+", "value": 80854 }
    ],
    "serverPopulation": {
        "na": {
            "name": "NA",
            "naw": [
                { "name": "Brelshaza", "value": 73361 },
                { "name": "Thaemine", "value": 68091 }
            ],
            "nae": [
                { "name": "Luterra", "value": 63037 },
                { "name": "Nineveh", "value": 53245 },
                { "name": "Vairgrys", "value": 50995 },
                { "name": "Balthorr", "value": 44136 },
                { "name": "Inanna", "value": 32918 }
            ]
        },
        "eu": {
            "name": "EU",
            "metrics": [
                { "name": "Ratik", "value": 70678 },
                { "name": "Elpon", "value": 62986 },
                { "name": "Gienah", "value": 59813 },
                { "name": "Arcturus", "value": 53465 },
                { "name": "Ortuus", "value": 23918 }
            ]
        }
    },
    "metrics": [
        { "name": "Total characters", "value": 656186 },
        { "name": "Total Rosters", "value": 146527 },
        { "name": "Total Guilds", "value": 33795 },
        { "name": "Average Roster Size", "value": 4.48 }
    ]
}"#;

pub struct UwuOwoApi {
    base_url: String,
}

impl UwuOwoApi {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_stats(&self) -> Result<GetStatsResult, serde_json::Error> {
        let stats = serde_json::from_str(DATA)?;

        Ok(stats)
    }

}