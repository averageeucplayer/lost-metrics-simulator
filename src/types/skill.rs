use crate::types::{BuffType, Class};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SkillType {
    #[default]
    Normal,
    Identity,
    HyperAwakeningTechnique,
    Awakening,
    HyperAwakening,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: u32,
    pub name: &'static str,
    pub cooldown: u32,
    pub is_counter: bool,
    pub summon: Option<u32>,
    pub is_trap: bool,
    pub is_projectile: bool,
    pub class_id: Class,
    pub kind: SkillType,
    pub skill_buff: Option<SkillBuff>,
}

#[derive(Debug, Clone)]
pub struct SkillBuff {
    pub id: u32,
    pub name: &'static str,
    pub is_party: bool,
    pub unique_group: u32,
    pub buff_type: BuffType,
    pub duration: u32,
}

impl Default for Skill {
    fn default() -> Self {
        Skill {
            id: 0,
            name: "",
            cooldown: 0,
            is_counter: false,
            is_projectile: false,
            is_trap: false,
            summon: None,
            class_id: Class::Unknown,
            kind: SkillType::Normal,
            skill_buff: None,
        }
    }
}


// pub const CC_DEBUFFS: [SkillBuff; 2] = [
//     SkillBuff { id: 322680, duration: 3 },
//     SkillBuff { id: 322680, duration: 3 },
// ];

// pub fn get_random_cc_debuff() -> SkillBuff {
//     (322680, 3)
// }