use std::collections::HashSet;

use chrono::Utc;
use rand::{rng, Rng};
use uuid::Uuid;

use crate::models::*;

pub struct FakeEncounter {
    encounter: Encounter
}

impl FakeEncounter {
    pub fn new() -> Self {
        let participants = vec![
            Player {
                id: 1,
                name: "Berserker".into(),
                class_id: 102,
                class_name: "Berserker".into(),
                ..Default::default()
            },
            Player {
                id: 2,
                name: "Wildsoul".into(),
                class_id: 604,
                class_name: "Wildsoul".into(),
                ..Default::default()
            },
            Player {
                id: 3,
                name: "Aeromancer".into(),
                class_id: 603,
                class_name: "Aeromancer".into(),
                ..Default::default()
            },
            Player {
                id: 4,
                name: "Bard".into(),
                is_support: true,
                class_id: 204,
                class_name: "Bard".into(),
                ..Default::default()
            },
            Player {
                id: 5,
                name: "Slayer".into(),
                class_id: 112,
                class_name: "Slayer".into(),
                ..Default::default()
            },
            Player {
                id: 6,
                name: "Mage".into(),
                class_id: 201,
                class_name: "Mage".into(),
                ..Default::default()
            },
            Player {
                id: 7,
                name: "Deadeye".into(),
                class_id: 503,
                class_name: "Deadeye".into(),
                ..Default::default()
            },
            Player {
                id: 8,
                name: "Paladin".into(),
                is_support: true,
                class_id: 105,
                class_name: "Paladin".into(),
                ..Default::default()
            }
        ];

        let encounter = Encounter {
            id: Uuid::now_v7(),
            name: "Narok the Butcher".into(),
            duration: Some(1000),
            started_on: Utc::now(),
            updated_on: Utc::now(),
            stats: crate::EncounterStats { 
                total_damage: 0.into(),
            },
            participants,
            boss: Boss {
                id: 1,
                name: "Narok the Butcher".into(),
                hp: 1e9 as u64,
                current_hp: 1e9 as u64,
                damage_taken: 9
            },
            parties: vec![
                Party {
                    id: 1,
                    member_ids: vec![1, 2, 3, 4].into_iter().collect(),
                    total_damage: 0.into(),
                    total_damage_percentage_to_raid: 0.0
                },
                Party {
                    id: 2,
                    member_ids: vec![5, 6, 7, 8].into_iter().collect(),
                    total_damage: 0.into(),
                    total_damage_percentage_to_raid: 0.0
                }
            ]
        };

        Self {
            encounter
        }
    }

    pub fn tick(&mut self) -> bool {

        let encounter = &mut self.encounter;

        if encounter.boss.current_hp == 0 {
            return false;
        }

        let mut rng = rng();
        let index = rng.random_range(1..8);

        let participant = &mut encounter.participants[index];
        let mut min_damage = 1e6 as u64;
        let mut max_damage = 1e7 as u64;

        if participant.is_support {
            min_damage = 1e5 as u64;
            max_damage = 1e6 as u64;
        }

        let mut damage = rng.random_range(min_damage..max_damage);

        if damage >= encounter.boss.current_hp {
            damage = encounter.boss.current_hp;
        }

        if participant.is_support {
            participant.stats.brand_uptime = rng.random_range(0.6..0.9);
            participant.stats.atk_power_uptime = rng.random_range(0.5..0.8);
            participant.stats.identity_uptime = rng.random_range(0.3..0.4);
        }

        Self::update_participant_stats(
            &mut encounter.parties,
            participant,
            damage,
            encounter.stats.total_damage.raw);
        let participants= &mut encounter.participants;
        Self::update_stats(
            &mut encounter.stats,
            &mut encounter.parties,
            participants,
            damage);

        encounter.boss.current_hp -= damage;
        encounter.boss.damage_taken += damage;
        encounter.updated_on = Utc::now();

        true
    }

    pub fn perform_attack(participant: &Player) -> Damage {
        
        Damage {
            value: 0,
            skill_id: 0,
            hit_flag: 0,
            is_critical: false
        }
    }

    pub fn update_participant_stats(
        parties: &mut Vec<Party>,
        participant: &mut Player,
        damage: u64,
        total_damage: u64) {
        participant.stats.total_damage += damage;

        for party in parties.iter_mut() {

            if party.member_ids.contains(&participant.id) {
                party.total_damage += damage;
            }

            party.total_damage_percentage_to_raid = party.total_damage.raw as f32 / total_damage as f32;
        }
    }

    pub fn update_stats(
        stats: &mut EncounterStats,
        parties: &mut Vec<Party>,
        participants: &mut Vec<Player>,
        damage: u64) {
        stats.total_damage += damage;
        
        for participant in participants.iter_mut() {
            let participant_stats = &mut participant.stats;
            let total_damage = &participant_stats.total_damage;

            participant_stats.total_damage_percentage_to_raid = total_damage.raw as f32 / stats.total_damage.raw as f32;

            for party in parties.iter_mut() {

                if party.member_ids.contains(&participant.id) {
                    participant_stats.total_damage_percentage_to_party = total_damage.raw as f32 / party.total_damage.raw as f32;
                }
            }
        }
    }

    pub fn get(&self) -> &Encounter {
        &self.encounter
    }
}