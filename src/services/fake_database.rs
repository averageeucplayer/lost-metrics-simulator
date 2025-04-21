use anyhow::*;
use uuid::Uuid;

use crate::{Encounter, EncounterPreview, EncounterStats, GetPastEncounterResult};

pub struct FakeDatabaseBuildOptions {
    pub encounter_count: u64,
}

pub struct FakeDatabase {
    encounters: Vec<Encounter>,
}

impl FakeDatabase {
    pub fn new() -> Self {
        Self {
            encounters: vec![],
        }
    }

    pub fn build(&mut self, options: FakeDatabaseBuildOptions) {

        for it in 1..=options.encounter_count  {
            
            let encounter = Encounter {
                id: Uuid::now_v7(),
                name: format!("Encounter {}", it),
                participants: vec![],
                started_on: chrono::Utc::now(),
                updated_on: chrono::Utc::now(),
                parties: vec![],
                duration: None,
                boss: Default::default(),
                stats: EncounterStats {
                    total_damage: 1000.into()
                }
                // duration: Some(chrono::Duration::seconds(it as i64)),
            };

            self.encounters.push(encounter);
        }

    }

    pub async fn get_past_encounters(&self, page: u64, page_size: u64) -> Result<GetPastEncounterResult> {
        let start = (page - 1) * page_size;
        let end = start + page_size;
        let items = self.encounters.iter().skip(start as usize).take(page_size as usize).cloned().map(|pr| 
            EncounterPreview { 
                id: pr.id,
                name: pr.name,
                participants: vec![],
                started_on: pr.started_on,
                duration: pr.duration.map(|pr| pr.to_string()).unwrap_or_else(|| "00:00".to_string()),
            }).collect();
        let total = self.encounters.len() as u64;
        
        let result = GetPastEncounterResult {
            items,
            total,
            page,
            page_size
        };

        Ok(result)
    }
}

    // let result = GetPastEncounterResult {
    //     items: vec![
    //         EncounterPreview {
    //             id: uuid::Uuid::now_v7(),
    //             name: "Echidna G1".to_string(),
    //             participants: vec![
    //                 PlayerSlim {
    //                     id: 2,
    //                     name: "Player".to_string(),
    //                     class_id: 1,
    //                     class_name: "Warrior".to_string(),
    //                 },
    //             ],
    //             duration: "10m".to_string(),
    //             started_on: chrono::Utc::now(),
    //         },
    //         EncounterPreview {
    //             id: uuid::Uuid::now_v7(),
    //             name: "Aegir G2".to_string(),
    //             duration: "10m".to_string(),
    //             started_on: chrono::Utc::now(),
    //         },
    //         EncounterPreview {
    //             id: uuid::Uuid::now_v7(),
    //             name: "Brelshaza G2".to_string(),
    //             duration: "10m".to_string(),
    //             started_on: chrono::Utc::now(),
    //         },
    //         EncounterPreview {
    //             id: uuid::Uuid::now_v7(),
    //             name: "Brelshaza G1".to_string(),
    //             duration: "10m".to_string(),
    //             started_on: chrono::Utc::now(),
    //         },
    //     ],
    //     total: 3,
    //     page: criteria.page,
    //     page_size: criteria.page_size,
    // };