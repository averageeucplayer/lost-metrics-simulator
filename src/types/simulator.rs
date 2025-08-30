use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use meter_core_fake::packets::definitions::{TripodIndex, TripodLevel};
use meter_core_fake::packets::structures::{StatPair, StatusEffectData};
use strum_macros::{AsRefStr, EnumString};

use crate::types::{Class, Skill, SkillBuff, Specialisation};


#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum HitOption {
    None = 0,
    BackAttack = 1,
    FrontalAttack = 2,
    FlankAttack = 3,
    Max = 4,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum HitFlag {
    Normal = 0,
    Critical = 1,
    Miss = 2,
    Invincible = 3,
    Dot = 4,
    Immune = 5,
    ImmuneSilenced = 6,
    FontSilenced = 7,
    DotCritical = 8,
    Dodge = 9,
    Reflect = 10,
    DamageShare = 11,
    DodgeHit = 12,
    Max = 13,
    Unknown = 14
}

#[derive(Default, Debug)]
pub struct Encounter {
    pub raid_instance_id: u32,
    pub local_player_id: u64,
    pub max_ticks: u64,
    pub zone_id: u32,
    pub zone_level: u32,
    pub parties: Vec<Party>,
    pub bosses: Vec<Boss>,
    pub esther: Vec<Esther>,
    pub sidereal_frequency: u64,
    pub use_vehicle_packet_instead_of_spawn: bool,
    pub supports_raid_begin_packet: bool,
    pub supports_zone_member_load_packet: bool,
    pub new_transit_packet: Option<u32>,
    pub invoke_zone_member_load: Option<(u32, u32)>,
    pub supports_npc_death_packet: bool,
    pub invoke_boss_kill_packet: bool,
    pub invoke_raid_result_packet: bool,
    pub invoke_zone_change_packet: bool,
    pub invoke_trigger_start_packet: Option<u32>,
    pub boss_attack_chance: f64,
    pub boss_knock_chance: f64,
    pub allowed_skill_cast_count: u32
}

#[derive(Debug)]
pub struct EstherInstance {
    pub id: u64,
    pub min_damage: i64,
    pub max_damage: i64,
    pub expires_at: u64
}

#[derive(Debug)]
pub struct Esther {
    pub npc_type_id: u32,
    pub initial_damage: i64,
    pub min_damage: i64,
    pub max_damage: i64,
    pub hp: i64
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub class: Class,
    pub specialisation: Specialisation,
    pub cooldown_reduction: f64,
    pub is_dead: bool,
    pub dies_at: u64,
    pub is_local: bool,
    pub is_support: bool,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub gear_level: f32,
    pub stats: Vec<StatPair>,
    pub status_effects: Vec<StatusEffectData>,
    pub attack_power: i64,
    pub crit_rate: f64,
    pub crit_damage: f64,
    pub multiplier: f64,
    pub buffs: HashMap<u32, Buff>,
    pub skills: Vec<SkillInstance>,
    pub traps: HashMap<u64, Trap>,
    pub summons: HashMap<u64, Summon>,
    pub incapacitate_time: u64,
}

#[derive(Debug, Clone)]
pub struct Summon {
    pub id: u64,
    pub skill_id: u32,
    pub damage_ratio: f64,
    pub expires_at: u64
}

#[derive(Debug, Clone)]
pub struct Trap {
    pub id: u64,
    pub skill_id: u32,
    pub damage_ratio: f64,
    pub expires_at: u64
}

#[derive(Debug, Clone)]
pub struct Party {
    pub id: u32,
    pub members: Vec<Player>,
}

#[derive(Debug, Default, Clone)]
pub struct SkillInstance {
    pub id: u32,
    pub is_counter: bool,
    pub initial_cooldown: u32,
    pub cooldown: u32,
    pub cooldown_ends_at: u64,
    pub hit_option: PreferredHitOption,
    pub tripod_index: Option<TripodIndex>,
    pub tripod_level: Option<TripodLevel>,
    pub effects: Vec<SkillEffect>
}


#[derive(Debug, Clone)]
pub enum SkillEffect {
    Damage {
        damage_ratio: f64,
    },
    Buff {
        effect_id: u32,
        unique_group: u32,
        is_party: bool,
        duration: u32,
        buff_type: BuffType
    },
    Summon {
        duration: u32,
        skill_id: u32,
        npc_id: u32,
        damage_ratio: f64
    },
    Projectile {
        damage_ratio: f64
    },
    Trap {
        duration: u32,
        skill_id: u32,
        damage_ratio: f64
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BuffType {
    None,
    HP(f64),
    Swiftness(f64),
    CritRate(f64),
    Specialisation(f64),
    AttackAndMovementSpeed(f64),
    AttackPower(f64),
    Damage(f64),
    Brand(f64),
    ArkPassive,
    ClassSkill,
    Identity,
    BattleItem,
    Ability,
    SupportBuff,
    Etc
}

#[derive(Debug, Default, Copy, Clone)]
#[repr(u8)]
pub enum PreferredHitOption {
    #[default]
    Any = 0,
    BackAttack = 1,
    FrontalAttack = 2,
}

impl SkillInstance {
    pub fn is_ready(&self, tick: u64) -> bool {
        tick >= self.cooldown_ends_at
    }
}

#[derive(Debug, Clone)]
pub struct Boss {
    pub id: u64,
    pub type_id: u32,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub min_damage: i64,
    pub max_damage: i64,
    pub skill_ids: Vec<u32>,
    pub stats: Vec<StatPair>,
    pub debuffs: HashMap<u32, Buff>,
}

#[derive(Debug, Clone)]
pub struct Buff {
    pub id: u32,
    pub is_party: bool,
    pub unique_group: u32,
    pub effect_id: u32,
    pub buff_type: BuffType,
    pub duration: u32,
    pub expires_at: u64,
}

#[derive(Debug, AsRefStr)]
pub enum EncounterEvent {
    NewTransit(u32),
    RaidBossKill,
    RaidResult,
    Trigger(u32),
    ZoneChange(u64),
    RaidBegin(u32),
    ZoneMemberLoad {
        zone_id: u32,
        zone_level: u32,
    },
    SpawnLocalPlayer(Player),
    SpawnPlayer(Player),
    PartyInfo {
        party_instance_id: u32,
        raid_instance_id: u32,
        members: Vec<Player>
    },
    SpawnBoss(Boss),
    NewVehicle(Player),
    Projectile {
        id: u64,
        skill_id: u32,
        skill_effect_id: Option<u32>,
        owner_id: u64,
    },
    Trap {
        id: u64,
        owner_id: u64,
        skill_id: u32
    },
    Npc {
        id: u64,
        npc_id: u32,
        hp: i64
    },
    NpcSummon {
        id: u64,
        owner_id: u64,
        npc_id: u32,
        hp: i64
    },
    Counter(u64),
    Death(u64),
    SkillStart {
        source_id: u64,
        skill_id: u32,
        tripod_index: Option<TripodIndex>,
        tripod_level: Option<TripodLevel>
    },
    SkillCast {
        source_id: u64,
        skill_id: u32,
    },
    RemoveObject(u64),
    AbnormalDamage {
        source_id: u64,
        target_id: u64,
        damage: i64,
        skill_id: u32,
        hit_option: HitOption,
        hit_flag: HitFlag,
        cur_hp: i64,
        max_hp: i64,
        down_time: Option<f32>,
        stand_up_time: Option<f32>,
        move_time: Option<f32>,
    },
    Damage {
        source_id: u64,
        target_id: u64,
        damage: i64,
        skill_id: u32,
        hit_option: HitOption,
        hit_flag: HitFlag,
        cur_hp: i64,
        max_hp: i64,
    },
    AddPartyBuff {
        id: u32,
        effect_id: u32,
        duration: u32,
        source_id: u64,
        target_id: u64,
    },
    AddBuff {
        id: u32,
        effect_id: u32,
        duration: u32,
        source_id: u64,
        target_id: u64,
    },
    RemovePartyBuff {
        id: u32,
        target_id: u64
    },
    RemoveBuff {
        id: u32,
        target_id: u64
    },
}
