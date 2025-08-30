#[derive(Debug, Default, Clone)]
pub struct RaidTemplateEsther {
    pub name: &'static str,
    pub npc_id: u32,
    pub skill_id: u32
}

#[derive(Debug, Default, Clone)]
pub struct RaidTemplateNpc {
    pub name: &'static str,
    pub npc_id: u32,
    pub hp: i64,
}

#[derive(Debug, Default, Clone)]
pub struct RaidTemplate {
    pub gate: u8,
    pub npcs: Vec<RaidTemplateNpc>,
    pub party_count: u8,
    pub zone_id: u32,
    pub zone_level: u32,
    pub esther: [RaidTemplateEsther; 3],
}

pub fn mordum_g3() -> RaidTemplate {
    RaidTemplate {
        gate: 3,
        npcs: vec![
            RaidTemplateNpc {
                npc_id: 485800,
                name: "Mordum, the Abyssal Punisher",
                hp: 1.2e14 as i64,
            },
            RaidTemplateNpc {
                npc_id: 485802,
                name: "Mordum's Hammer",
                hp: 1e6 as i64,
            },
            RaidTemplateNpc {
                npc_id: 485803,
                name: "Mordum's Hammer",
                hp: 1e6 as i64,
            },
            RaidTemplateNpc {
                npc_id: 485805,
                name: "Flash of Punishment",
                hp: 1e6 as i64,
            },
        ],
        party_count: 2,
        zone_id: 37533,
        zone_level: 1,
        esther: [
            RaidTemplateEsther{
                name: "Balthor",
                npc_id: 53300,
                skill_id: 533000
            },
            RaidTemplateEsther{
                name: "Bastian",
                npc_id: 58800,
                skill_id: 99999
            },
            RaidTemplateEsther{
                name: "Shandi",
                npc_id: 56700,
                skill_id: 537000
            },
        ]
    }
}