use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncounterTemplate {
    pub id: i64,
    pub name: String,
    pub boss_name: String,
    pub enrage_timer: String,
    pub party_count: u8,
    pub max_hp: i64,
    pub hp_bars: u16,
}

pub fn get_templates() -> Vec<EncounterTemplate> {
    vec![
        EncounterTemplate {
            id: 2,
            name: "Echidna G1".to_string(),
            boss_name: "Red Doom Narkiel".to_string(),
            enrage_timer: "10:00".to_string(),
            max_hp: 3e11 as i64,
            party_count: 2,
            hp_bars: 300,
        },
        EncounterTemplate {
            id: 3,
            name: "Echidna G2".to_string(),
            boss_name: "Covetous Master Echidna".to_string(),
            enrage_timer: "10:00".to_string(),
            max_hp: 3e11 as i64,
            party_count: 2,
            hp_bars: 300,
        },
        EncounterTemplate {
            id: 4,
            name: "Aegir G1".to_string(),
            boss_name: "Akkan, Lord of Death".to_string(),
            enrage_timer: "10:00".to_string(),
            max_hp: 4e11 as i64,
            party_count: 2,
            hp_bars: 300,
        },
        EncounterTemplate {
            id: 5,
            name: "Aegir G2".to_string(),
            boss_name: "Aegir, the Oppressor".to_string(),
            enrage_timer: "10:00".to_string(),
            max_hp: 4e11 as i64,
            party_count: 2,
            hp_bars: 300,
        },
        EncounterTemplate {
            id: 6,
            name: "Act 2: Brelshaza G1".to_string(),
            boss_name: "Narok the Butcher".to_string(),
            enrage_timer: "10:00".to_string(),
            max_hp: 4e11 as i64,
            party_count: 2,
            hp_bars: 300,
        },
        EncounterTemplate {
            id: 7,
            name: "Act 2: Brelshaza G2".to_string(),
            boss_name:  "Phantom Manifester Brelshaza".to_string(),
            enrage_timer: "10:00".to_string(),
            max_hp: 4e11 as i64,
            party_count: 2,
            hp_bars: 400,
        },
        EncounterTemplate {
            id: 8,
            name: "Behemoth G1".to_string(),
            boss_name: "Behemoth, the Storm Commander".to_string(),
            enrage_timer: "12:00".to_string(),
            max_hp: 2.08e11 as i64,
            party_count: 4,
            hp_bars: 500,
        },
        EncounterTemplate {
            id: 9,
            name: "Behemoth G2".to_string(),
            boss_name: "Behemoth, Cruel Storm Slayer".to_string(),
            enrage_timer: "12:00".to_string(),
            max_hp: 2.93e11 as i64,
            party_count: 4,
            hp_bars: 705,
        },
    ]
}