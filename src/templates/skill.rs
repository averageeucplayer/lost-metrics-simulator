#![allow(dead_code)]

use crate::{templates::skill_buff::*, types::{Class, Skill, SkillBuff, SkillType}};

pub const UNKNOWN_SKILL: Skill = Skill {
    id: 0,
    name: "Unknown",
    cooldown: 0,
    class_id: Class::Unknown,
    kind: SkillType::Normal,
    skill_buff: None,
	is_counter: false,
	is_projectile: true,
	is_trap: false,
	summon: None
};

pub const TUMBLING_10060: Skill = Skill {
    id: 10060,
    class_id: Class::WarriorMale,
    name: "Tumbling",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHOULDER_CHARGE_16061: Skill = Skill {
    id: 16061,
    class_id: Class::Berserker,
    name: "Shoulder Charge",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHAIN_OF_VENGEANCE: Skill = Skill {
    id: 16700,
    class_id: Class::Berserker,
    name: "Chain of Vengeance",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HELL_BLADE_TEMP_16181: Skill = Skill {
    id: 16181,
    class_id: Class::Berserker,
    name: "Hell Blade Temp",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DIVING_SLASH: Skill = Skill {
    id: 16210,
    class_id: Class::Berserker,
    name: "Diving Slash",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FINISH_STRIKE: Skill = Skill {
    id: 16300,
    class_id: Class::Berserker,
    name: "Finish Strike",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HELL_BLADE_TEMP_16180: Skill = Skill {
    id: 16180,
    class_id: Class::Berserker,
    name: "Hell Blade Temp",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOOD_SLASH: Skill = Skill {
    id: 16660,
    class_id: Class::Berserker,
    name: "Blood Slash",
    cooldown: 65,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOODY_RUSH_16145: Skill = Skill {
    id: 16145,
    class_id: Class::Berserker,
    name: "Bloody Rush",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MAELSTROM_16620: Skill = Skill {
    id: 16620,
    class_id: Class::Berserker,
    name: "Maelstrom",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRAVE_SLASH: Skill = Skill {
    id: 16630,
    class_id: Class::Berserker,
    name: "Brave Slash",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAGE_DEATHBLADE: Skill = Skill {
    id: 16730,
    class_id: Class::Berserker,
    name: "Rage Deathblade",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WIND_BLADE: Skill = Skill {
    id: 16610,
    class_id: Class::Berserker,
    name: "Wind Blade",
    cooldown: 13,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THEATRIC_COMBAT_SWITCH: Skill = Skill {
    id: 16160,
    class_id: Class::Berserker,
    name: "Theatric Combat Switch",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRIME_HAZARD: Skill = Skill {
    id: 16050,
    class_id: Class::Berserker,
    name: "Crime Hazard",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOODY_RUSH_16147: Skill = Skill {
    id: 16147,
    class_id: Class::Berserker,
    name: "Bloody Rush",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHARGE_STRIKE: Skill = Skill {
    id: 16150,
    class_id: Class::Berserker,
    name: "Charge Strike",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RED_DUST: Skill = Skill {
    id: 16120,
    class_id: Class::Berserker,
    name: "Red Dust",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HELL_BLADE_TEMP_16170: Skill = Skill {
    id: 16170,
    class_id: Class::Berserker,
    name: "Hell Blade Temp",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DARK_RUSH: Skill = Skill {
    id: 16141,
    class_id: Class::Berserker,
    name: "Dark Rush",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHAIN_SWORD: Skill = Skill {
    id: 16500,
    class_id: Class::Berserker,
    name: "Chain Sword",
    cooldown: 13,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOODY_RUSH_16140: Skill = Skill {
    id: 16140,
    class_id: Class::Berserker,
    name: "Bloody Rush",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TUMBLING_16040: Skill = Skill {
    id: 16040,
    class_id: Class::Berserker,
    name: "Tumbling",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FURY_METHOD: Skill = Skill {
    id: 16650,
    class_id: Class::Berserker,
    name: "Fury Method",
    cooldown: 55,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WHIRLWIND: Skill = Skill {
    id: 16070,
    class_id: Class::Berserker,
    name: "Whirlwind",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const POWER_BREAK: Skill = Skill {
    id: 16030,
    class_id: Class::Berserker,
    name: "Power Break",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HELL_BLADE: Skill = Skill {
    id: 16080,
    class_id: Class::Berserker,
    name: "Hell Blade",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const OVERDRIVE: Skill = Skill {
    id: 16640,
    class_id: Class::Berserker,
    name: "Overdrive",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOODY_SURGE: Skill = Skill {
    id: 16720,
    class_id: Class::Berserker,
    name: "Bloody Surge",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWORD_STORM: Skill = Skill {
    id: 16600,
    class_id: Class::Berserker,
    name: "Sword Storm",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHOULDER_CHARGE_16060: Skill = Skill {
    id: 16060,
    class_id: Class::Berserker,
    name: "Shoulder Charge",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ASSAULT_BLADE: Skill = Skill {
    id: 16110,
    class_id: Class::Berserker,
    name: "Assault Blade",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BERSERK_FURY: Skill = Skill {
    id: 16710,
    class_id: Class::Berserker,
    name: "Berserk Fury",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FIST_ATTACK: Skill = Skill {
    id: 16010,
    class_id: Class::Berserker,
    name: "Fist Attack",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BURST_16130: Skill = Skill {
    id: 16130,
    class_id: Class::Berserker,
    name: "Burst",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOODY_RUSH_16146: Skill = Skill {
    id: 16146,
    class_id: Class::Berserker,
    name: "Bloody Rush",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOUNTAIN_CRASH: Skill = Skill {
    id: 16220,
    class_id: Class::Berserker,
    name: "Mountain Crash",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STAMPEDE_16041: Skill = Skill {
    id: 16041,
    class_id: Class::Berserker,
    name: "Stampede",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DOUBLE_SLASH: Skill = Skill {
    id: 16100,
    class_id: Class::Berserker,
    name: "Double Slash",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TEMPEST_SLASH: Skill = Skill {
    id: 16190,
    class_id: Class::Berserker,
    name: "Tempest Slash",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BASIC_3_CHAIN_HITS: Skill = Skill {
    id: 18000,
    class_id: Class::Destroyer,
    name: "Basic 3 Chain Hits",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GALAXY_BREAK: Skill = Skill {
    id: 18240,
    class_id: Class::Destroyer,
    name: "Galaxy Break",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHAIN_STRIKE: Skill = Skill {
    id: 18260,
    class_id: Class::Destroyer,
    name: "Chain Strike",
    cooldown: 45,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FULL_SWING: Skill = Skill {
    id: 18070,
    class_id: Class::Destroyer,
    name: "Full Swing",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HEAVY_CRUSH: Skill = Skill {
    id: 18050,
    class_id: Class::Destroyer,
    name: "Heavy Crush",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ACTIVATE_HYPERGRAVITY: Skill = Skill {
    id: 18010,
    class_id: Class::Destroyer,
    name: "Activate Hypergravity",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SEISMIC_HAMMER: Skill = Skill {
    id: 18130,
    class_id: Class::Destroyer,
    name: "Seismic Hammer",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const POWER_SHOULDER: Skill = Skill {
    id: 18220,
    class_id: Class::Destroyer,
    name: "Power Shoulder",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EARTH_EATER: Skill = Skill {
    id: 18150,
    class_id: Class::Destroyer,
    name: "Earth Eater",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JUMPING_SMASH: Skill = Skill {
    id: 18160,
    class_id: Class::Destroyer,
    name: "Jumping Smash",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENDURE_PAIN: Skill = Skill {
    id: 18140,
    class_id: Class::Destroyer,
    name: "Endure Pain",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GRAVITATIONAL_ENERGY_18200: Skill = Skill {
    id: 18200,
    class_id: Class::Destroyer,
    name: "Gravitational Energy",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PERFECT_SWING: Skill = Skill {
    id: 18170,
    class_id: Class::Destroyer,
    name: "Perfect Swing",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HYPER_BIG_BANG: Skill = Skill {
    id: 18250,
    class_id: Class::Destroyer,
    name: "Hyper Big Bang",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const VORTEX_GRAVITY: Skill = Skill {
    id: 18011,
    class_id: Class::Destroyer,
    name: "Vortex Gravity",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BIG_BANG: Skill = Skill {
    id: 18230,
    class_id: Class::Destroyer,
    name: "Big Bang",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GRAVITATIONAL_ENERGY_18201: Skill = Skill {
    id: 18201,
    class_id: Class::Destroyer,
    name: "Gravitational Energy",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUPERNOVA_18270: Skill = Skill {
    id: 18270,
    class_id: Class::Destroyer,
    name: "Supernova",
    cooldown: 72,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NEUTRALIZER: Skill = Skill {
    id: 18100,
    class_id: Class::Destroyer,
    name: "Neutralizer",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DREADNAUGHT: Skill = Skill {
    id: 18110,
    class_id: Class::Destroyer,
    name: "Dreadnaught",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RUN_18020: Skill = Skill {
    id: 18020,
    class_id: Class::Destroyer,
    name: "Run",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TERRA_BREAK: Skill = Skill {
    id: 18120,
    class_id: Class::Destroyer,
    name: "Terra Break",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EARTH_WAVE: Skill = Skill {
    id: 18090,
    class_id: Class::Destroyer,
    name: "Earth Wave",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EARTH_SMASHER: Skill = Skill {
    id: 18080,
    class_id: Class::Destroyer,
    name: "Earth Smasher",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GRAVITY_IMPACT: Skill = Skill {
    id: 18060,
    class_id: Class::Destroyer,
    name: "Gravity Impact",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const POWER_STRIKE: Skill = Skill {
    id: 18180,
    class_id: Class::Destroyer,
    name: "Power Strike",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GRAVITY_COMPRESSION: Skill = Skill {
    id: 18190,
    class_id: Class::Destroyer,
    name: "Gravity Compression",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HYPERGRAVITY_BASIC_ATTACK: Skill = Skill {
    id: 18030,
    class_id: Class::Destroyer,
    name: "Hypergravity Basic Attack",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RUNNING_CRASH: Skill = Skill {
    id: 18210,
    class_id: Class::Destroyer,
    name: "Running Crash",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEFENSIVE_STANCE_DEACTIVATION: Skill = Skill {
    id: 17810,
    class_id: Class::Gunlancer,
    name: "Defensive Stance Deactivation",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHIELD_SHOCK_17101: Skill = Skill {
    id: 17101,
    class_id: Class::Gunlancer,
    name: "Shield Shock",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RISING_GUNLANCE: Skill = Skill {
    id: 17070,
    class_id: Class::Gunlancer,
    name: "Rising Gunlance",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DASH_UPPER_FIRE: Skill = Skill {
    id: 17080,
    class_id: Class::Gunlancer,
    name: "Dash Upper Fire",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SURGE_CANNON: Skill = Skill {
    id: 17200,
    class_id: Class::Gunlancer,
    name: "Surge Cannon",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GUNLANCE_SHOT: Skill = Skill {
    id: 17160,
    class_id: Class::Gunlancer,
    name: "Gunlance Shot",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHIELD_CHARGE: Skill = Skill {
    id: 17050,
    class_id: Class::Gunlancer,
    name: "Shield Charge",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHARGED_STINGER: Skill = Skill {
    id: 17210,
    class_id: Class::Gunlancer,
    name: "Charged Stinger",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEAP_ATTACK: Skill = Skill {
    id: 17110,
    class_id: Class::Gunlancer,
    name: "Leap Attack",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BASIC_ATTACK_3_COMBO_HITS: Skill = Skill {
    id: 17000,
    class_id: Class::Gunlancer,
    name: "Basic Attack 3 Combo Hits",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FIST_ATTACK_3_CHAIN_HITS: Skill = Skill {
    id: 17010,
    class_id: Class::Gunlancer,
    name: "Fist Attack 3 Chain Hits",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JUSTICE_SERVED: Skill = Skill {
    id: 17260,
    class_id: Class::Gunlancer,
    name: "Justice Served",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BATTLEFIELD_SHIELD: Skill = Skill {
    id: 17820,
    class_id: Class::Gunlancer,
    name: "Battlefield Shield",
    cooldown: 90,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GUARDIANS_THUNDERCRACK: Skill = Skill {
    id: 17140,
    class_id: Class::Gunlancer,
    name: "Guardian's Thundercrack",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHIELD_BASH: Skill = Skill {
    id: 17150,
    class_id: Class::Gunlancer,
    name: "Shield Bash",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FIRE_BULLET: Skill = Skill {
    id: 17060,
    class_id: Class::Gunlancer,
    name: "Fire Bullet",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHARGE_17020: Skill = Skill {
    id: 17020,
    class_id: Class::Gunlancer,
    name: "Charge",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BASH: Skill = Skill {
    id: 17040,
    class_id: Class::Gunlancer,
    name: "Bash",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHIELD_DASH: Skill = Skill {
    id: 17230,
    class_id: Class::Gunlancer,
    name: "Shield Dash",
    cooldown: 45,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LANCE_OF_JUDGMENT: Skill = Skill {
    id: 17220,
    class_id: Class::Gunlancer,
    name: "Lance of Judgment",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHIELD_SHOCK_17100: Skill = Skill {
    id: 17100,
    class_id: Class::Gunlancer,
    name: "Shield Shock",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHARP_GUNLANCE: Skill = Skill {
    id: 17030,
    class_id: Class::Gunlancer,
    name: "Sharp Gunlance",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEFENSIVE_STANCE: Skill = Skill {
    id: 17800,
    class_id: Class::Gunlancer,
    name: "Defensive Stance",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPEAR_DASH: Skill = Skill {
    id: 17240,
    class_id: Class::Gunlancer,
    name: "Spear Dash",
    cooldown: 75,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHOUT_OF_HATRED: Skill = Skill {
    id: 17180,
    class_id: Class::Gunlancer,
    name: "Shout of Hatred",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOOK_CHAIN: Skill = Skill {
    id: 17090,
    class_id: Class::Gunlancer,
    name: "Hook Chain",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GUARDIANS_PROTECTION: Skill = Skill {
    id: 17170,
    class_id: Class::Gunlancer,
    name: "Guardian's Protection",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NELASIAS_ENERGY: Skill = Skill {
    id: 17130,
    class_id: Class::Gunlancer,
    name: "Nelasia's Energy",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COUNTER_GUNLANCE: Skill = Skill {
    id: 17190,
    class_id: Class::Gunlancer,
    name: "Counter Gunlance",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GUARDIANS_OATH: Skill = Skill {
    id: 17250,
    class_id: Class::Gunlancer,
    name: "Guardian's Oath",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ALITHANESS_JUDGMENT: Skill = Skill {
    id: 36210,
    class_id: Class::Paladin,
    name: "Alithanes's Judgment",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWORD_OF_JUSTICE: Skill = Skill {
    id: 36080,
    class_id: Class::Paladin,
    name: "Sword of Justice",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ALITHANESS_RAGE: Skill = Skill {
    id: 36240,
    class_id: Class::Paladin,
    name: "Alithanes's Rage",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_THRUST: Skill = Skill {
    id: 36040,
    class_id: Class::Paladin,
    name: "Flash Thrust",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PVP_FORCED_WAKEUP: Skill = Skill {
    id: 36026,
    class_id: Class::Paladin,
    name: "PvP Forced Wakeup",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPRINT_36020: Skill = Skill {
    id: 36020,
    class_id: Class::Paladin,
    name: "Sprint",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOLY_PROTECTION: Skill = Skill {
    id: 36140,
    class_id: Class::Paladin,
    name: "Holy Protection",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WRATH_OF_GOD: Skill = Skill {
    id: 36170,
    class_id: Class::Paladin,
    name: "Wrath of God",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: Some(WRATH_OF_GOD_361710),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_SLASH: Skill = Skill {
    id: 36090,
    class_id: Class::Paladin,
    name: "Flash Slash",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ALITHANESS_DEVOTION: Skill = Skill {
    id: 36230,
    class_id: Class::Paladin,
    name: "Alithanes's Devotion",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOLY_EXPLOSION: Skill = Skill {
    id: 36100,
    class_id: Class::Paladin,
    name: "Holy Explosion",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOLY_SWORD: Skill = Skill {
    id: 36190,
    class_id: Class::Paladin,
    name: "Holy Sword",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HEAVENLY_GUARDIAN_BASIC_ATTACK: Skill = Skill {
    id: 36001,
    class_id: Class::Paladin,
    name: "Heavenly Guardian Basic Attack",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PUNISHMENT: Skill = Skill {
    id: 36110,
    class_id: Class::Paladin,
    name: "Punishment",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHT_OF_JUDGMENT: Skill = Skill {
    id: 36060,
    class_id: Class::Paladin,
    name: "Light of Judgment",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHARGE_36070: Skill = Skill {
    id: 36070,
    class_id: Class::Paladin,
    name: "Charge",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOLY_AURA: Skill = Skill {
    id: 36800,
    class_id: Class::Paladin,
    name: "Holy Aura",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: Some(HOLY_AURA_360102),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SACRED_EXECUTIONER_36910: Skill = Skill {
    id: 36910,
    class_id: Class::Paladin,
    name: "Sacred Executioner",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DIVINE_WAVE: Skill = Skill {
    id: 36280,
    class_id: Class::Paladin,
    name: "Divine Wave",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GODS_DECREE: Skill = Skill {
    id: 36150,
    class_id: Class::Paladin,
    name: "God's Decree",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DIVINE_JUSTICE: Skill = Skill {
    id: 36260,
    class_id: Class::Paladin,
    name: "Divine Justice",
    cooldown: 60,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ALITHANESS_LIGHT: Skill = Skill {
    id: 36220,
    class_id: Class::Paladin,
    name: "Alithanes's Light",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DASH_SLASH: Skill = Skill {
    id: 36130,
    class_id: Class::Paladin,
    name: "Dash Slash",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIN_SLASH: Skill = Skill {
    id: 36030,
    class_id: Class::Paladin,
    name: "Spin Slash",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHT_SHOCK: Skill = Skill {
    id: 36050,
    class_id: Class::Paladin,
    name: "Light Shock",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const UNARMED_BASIC_ATTACK: Skill = Skill {
    id: 36010,
    class_id: Class::Paladin,
    name: "Unarmed Basic Attack",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EXECUTION_OF_JUSTICE: Skill = Skill {
    id: 36180,
    class_id: Class::Paladin,
    name: "Execution of Justice",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOLY_AREA: Skill = Skill {
    id: 36120,
    class_id: Class::Paladin,
    name: "Holy Area",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EXECUTION_OF_JUDGMENT: Skill = Skill {
    id: 36250,
    class_id: Class::Paladin,
    name: "Execution of Judgment",
    cooldown: 45,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HEAVENLY_BLESSINGS: Skill = Skill {
    id: 36200,
    class_id: Class::Paladin,
    name: "Heavenly Blessings",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: Some(HEAVENLY_BLESSINGS_362008),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JUDGMENT_BLADE: Skill = Skill {
    id: 36270,
    class_id: Class::Paladin,
    name: "Judgment Blade",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EXECUTIONERS_SWORD: Skill = Skill {
    id: 36160,
    class_id: Class::Paladin,
    name: "Executioner's Sword",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SACRED_EXECUTIONER_36900: Skill = Skill {
    id: 36900,
    class_id: Class::Paladin,
    name: "Sacred Executioner",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOUNTAIN_CLEAVE: Skill = Skill {
    id: 45600,
    class_id: Class::Slayer,
    name: "Mountain Cleave",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const VOLCANIC_ERUPTION: Skill = Skill {
    id: 45080,
    class_id: Class::Slayer,
    name: "Volcanic Eruption",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HURRICANE_SWORD: Skill = Skill {
    id: 45070,
    class_id: Class::Slayer,
    name: "Hurricane Sword",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAGE_SLASHER: Skill = Skill {
    id: 45820,
    class_id: Class::Slayer,
    name: "Rage Slasher",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_BLADE: Skill = Skill {
    id: 45110,
    class_id: Class::Slayer,
    name: "Flash Blade",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BURST_45003: Skill = Skill {
    id: 45003,
    class_id: Class::Slayer,
    name: "Burst",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAGNA_DEATHBLADE: Skill = Skill {
    id: 45830,
    class_id: Class::Slayer,
    name: "Ragna Deathblade",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRUEL_PIERCE: Skill = Skill {
    id: 45740,
    class_id: Class::Slayer,
    name: "Cruel Pierce",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CROSS_BLADE: Skill = Skill {
    id: 45100,
    class_id: Class::Slayer,
    name: "Cross Blade",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FINAL_BLOW: Skill = Skill {
    id: 45190,
    class_id: Class::Slayer,
    name: "Final Blow",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLYING_STRIKE: Skill = Skill {
    id: 45210,
    class_id: Class::Slayer,
    name: "Flying Strike",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FATAL_SWORD: Skill = Skill {
    id: 45300,
    class_id: Class::Slayer,
    name: "Fatal Sword",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DASH_45010: Skill = Skill {
    id: 45010,
    class_id: Class::Slayer,
    name: "Dash",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRAL_DEATHBLADE: Skill = Skill {
    id: 45750,
    class_id: Class::Slayer,
    name: "Spiral Deathblade",
    cooldown: 60,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PUNISHING_DRAW: Skill = Skill {
    id: 45700,
    class_id: Class::Slayer,
    name: "Punishing Draw",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WILD_RUSH: Skill = Skill {
    id: 45060,
    class_id: Class::Slayer,
    name: "Wild Rush",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WILD_STOMP: Skill = Skill {
    id: 45220,
    class_id: Class::Slayer,
    name: "Wild Stomp",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GUILLOTINE: Skill = Skill {
    id: 45720,
    class_id: Class::Slayer,
    name: "Guillotine",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GROUND_SMASH: Skill = Skill {
    id: 45620,
    class_id: Class::Slayer,
    name: "Ground Smash",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOODLUST: Skill = Skill {
    id: 45004,
    class_id: Class::Slayer,
    name: "Bloodlust",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLAME_DEATHBLADE: Skill = Skill {
    id: 45760,
    class_id: Class::Slayer,
    name: "Flame Deathblade",
    cooldown: 80,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FURY_BLADE: Skill = Skill {
    id: 45050,
    class_id: Class::Slayer,
    name: "Fury Blade",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRUTAL_IMPACT: Skill = Skill {
    id: 45730,
    class_id: Class::Slayer,
    name: "Brutal Impact",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPINNING_SWORD: Skill = Skill {
    id: 45610,
    class_id: Class::Slayer,
    name: "Spinning Sword",
    cooldown: 13,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAGNA_BREAK: Skill = Skill {
    id: 45810,
    class_id: Class::Slayer,
    name: "Ragna Break",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WILD_DASH: Skill = Skill {
    id: 45011,
    class_id: Class::Slayer,
    name: "Wild Dash",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FURIOUS_CLAW: Skill = Skill {
    id: 45710,
    class_id: Class::Slayer,
    name: "Furious Claw",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EXECUTE: Skill = Skill {
    id: 45800,
    class_id: Class::Slayer,
    name: "Execute",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REQUIEM_RAIN: Skill = Skill {
    id: 48200,
    class_id: Class::Valkyrie,
    name: "Requiem Rain",
    cooldown: 34,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERAPHIC_OATH: Skill = Skill {
    id: 48250,
    class_id: Class::Valkyrie,
    name: "Seraphic Oath",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: Some(SERAPHIC_OATH_480031),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FORESIGHT_SLASH: Skill = Skill {
    id: 48100,
    class_id: Class::Valkyrie,
    name: "Foresight Slash",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RELEASE_LIGHT_48041: Skill = Skill {
    id: 48041,
    class_id: Class::Valkyrie,
    name: "Release Light",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHT_OF_THE_FAITHFUL_48052: Skill = Skill {
    id: 48052,
    class_id: Class::Valkyrie,
    name: "Light of the Faithful",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SALVATION_SITE: Skill = Skill {
    id: 48240,
    class_id: Class::Valkyrie,
    name: "Salvation Site",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHT_OF_THE_FAITHFUL_48051: Skill = Skill {
    id: 48051,
    class_id: Class::Valkyrie,
    name: "Light of the Faithful",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const METEOR_STRIKE: Skill = Skill {
    id: 48110,
    class_id: Class::Valkyrie,
    name: "Meteor Strike",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FINAL_SPLENDOR: Skill = Skill {
    id: 48070,
    class_id: Class::Valkyrie,
    name: "Final Splendor",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TRUTHS_DECREE: Skill = Skill {
    id: 48220,
    class_id: Class::Valkyrie,
    name: "Truth's Decree",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWORD_OF_REVELATION: Skill = Skill {
    id: 48160,
    class_id: Class::Valkyrie,
    name: "Sword of Revelation",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHT_OF_THE_FAITHFUL_48053: Skill = Skill {
    id: 48053,
    class_id: Class::Valkyrie,
    name: "Light of the Faithful",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const APPROACH_OF_REVELATION: Skill = Skill {
    id: 48170,
    class_id: Class::Valkyrie,
    name: "Approach of Revelation",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CIRCLE_OF_TRUTH: Skill = Skill {
    id: 48230,
    class_id: Class::Valkyrie,
    name: "Circle of Truth",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENTER: Skill = Skill {
    id: 48020,
    class_id: Class::Valkyrie,
    name: "Enter",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WHISPER_OF_JUDGMENT: Skill = Skill {
    id: 48150,
    class_id: Class::Valkyrie,
    name: "Whisper of Judgment",
    cooldown: 26,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RELEASE_LIGHT_48042: Skill = Skill {
    id: 48042,
    class_id: Class::Valkyrie,
    name: "Release Light",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REQUIEM_ASH: Skill = Skill {
    id: 48210,
    class_id: Class::Valkyrie,
    name: "Requiem Ash",
    cooldown: 35,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHT_OF_THE_FAITHFUL_48050: Skill = Skill {
    id: 48050,
    class_id: Class::Valkyrie,
    name: "Light of the Faithful",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHT_OF_THE_FAITHFUL_48054: Skill = Skill {
    id: 48054,
    class_id: Class::Valkyrie,
    name: "Light of the Faithful",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LUNGING_STAB: Skill = Skill {
    id: 48120,
    class_id: Class::Valkyrie,
    name: "Lunging Stab",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERAPHIC_LEAP: Skill = Skill {
    id: 48270,
    class_id: Class::Valkyrie,
    name: "Seraphic Leap",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: Some(SERAPHIC_LEAP_480034),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRUNDIAS_EPIPHANY: Skill = Skill {
    id: 48520,
    class_id: Class::Valkyrie,
    name: "Brundia's Epiphany",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EXECUTION_OF_REVELATION: Skill = Skill {
    id: 48180,
    class_id: Class::Valkyrie,
    name: "Execution of Revelation",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CATACLYSM: Skill = Skill {
    id: 48500,
    class_id: Class::Valkyrie,
    name: "Cataclysm",
    cooldown: 65,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLESSING_OF_SALVATION: Skill = Skill {
    id: 48260,
    class_id: Class::Valkyrie,
    name: "Blessing of Salvation",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JUDGMENT_STIGMATA: Skill = Skill {
    id: 48140,
    class_id: Class::Valkyrie,
    name: "Judgment Stigmata",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REQUIEM_STORM: Skill = Skill {
    id: 48190,
    class_id: Class::Valkyrie,
    name: "Requiem Storm",
    cooldown: 26,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RELEASE_LIGHT_48040: Skill = Skill {
    id: 48040,
    class_id: Class::Valkyrie,
    name: "Release Light",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHINING_KNIGHT: Skill = Skill {
    id: 48060,
    class_id: Class::Valkyrie,
    name: "Shining Knight",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRUNDIAS_RITUAL: Skill = Skill {
    id: 48530,
    class_id: Class::Valkyrie,
    name: "Brundia's Ritual",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DIVINE_CONFIRMATION: Skill = Skill {
    id: 48510,
    class_id: Class::Valkyrie,
    name: "Divine Confirmation",
    cooldown: 60,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRUNDIAS_SANCTUARY: Skill = Skill {
    id: 48550,
    class_id: Class::Valkyrie,
    name: "Brundia's Sanctuary",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRUNDIAS_INCARNATION: Skill = Skill {
    id: 48540,
    class_id: Class::Valkyrie,
    name: "Brundia's Incarnation",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRUSHING_CONDEMNATION: Skill = Skill {
    id: 48130,
    class_id: Class::Valkyrie,
    name: "Crushing Condemnation",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STREAM_OF_EDGE: Skill = Skill {
    id: 19150,
    class_id: Class::Arcanist,
    name: "Stream of Edge",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const UNLIMITED_SHUFFLE: Skill = Skill {
    id: 19120,
    class_id: Class::Arcanist,
    name: "Unlimited Shuffle",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CULL: Skill = Skill {
    id: 19281,
    class_id: Class::Arcanist,
    name: "Cull",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOUR_OF_A_KIND: Skill = Skill {
    id: 19160,
    class_id: Class::Arcanist,
    name: "Four of a Kind",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THE_TOWER: Skill = Skill {
    id: 19360,
    class_id: Class::Arcanist,
    name: "The Tower",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRAL_EDGE_19190: Skill = Skill {
    id: 19190,
    class_id: Class::Arcanist,
    name: "Spiral Edge",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BALANCE: Skill = Skill {
    id: 19099,
    class_id: Class::Arcanist,
    name: "Balance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const KNIGHT_OF_THE_EMPRESS: Skill = Skill {
    id: 19288,
    class_id: Class::Arcanist,
    name: "Knight of the Empress",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRAL_EDGE_19195: Skill = Skill {
    id: 19195,
    class_id: Class::Arcanist,
    name: "Spiral Edge",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DARK_RESURRECTION: Skill = Skill {
    id: 19050,
    class_id: Class::Arcanist,
    name: "Dark Resurrection",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JUDGMENT_19098: Skill = Skill {
    id: 19098,
    class_id: Class::Arcanist,
    name: "Judgment",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JOY: Skill = Skill {
    id: 19285,
    class_id: Class::Arcanist,
    name: "Joy",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CLOWN: Skill = Skill {
    id: 19284,
    class_id: Class::Arcanist,
    name: "Clown",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHECKMATE: Skill = Skill {
    id: 19040,
    class_id: Class::Arcanist,
    name: "Checkmate",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SECRET_GARDEN: Skill = Skill {
    id: 19310,
    class_id: Class::Arcanist,
    name: "Secret Garden",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EVOKE_19290: Skill = Skill {
    id: 19290,
    class_id: Class::Arcanist,
    name: "Evoke",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANCELLOR: Skill = Skill {
    id: 19286,
    class_id: Class::Arcanist,
    name: "Chancellor",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MYSTERIOUS_STAMPEDE_19210: Skill = Skill {
    id: 19210,
    class_id: Class::Arcanist,
    name: "Mysterious Stampede",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH: Skill = Skill {
    id: 19370,
    class_id: Class::Arcanist,
    name: "Death",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PRISMATIC_MIRROR: Skill = Skill {
    id: 19110,
    class_id: Class::Arcanist,
    name: "Prismatic Mirror",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOON: Skill = Skill {
    id: 19092,
    class_id: Class::Arcanist,
    name: "Moon",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ROYAL: Skill = Skill {
    id: 19096,
    class_id: Class::Arcanist,
    name: "Royal",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DANCING_OF_SPINEFLOWER: Skill = Skill {
    id: 19200,
    class_id: Class::Arcanist,
    name: "Dancing of Spineflower",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CORROSION: Skill = Skill {
    id: 19093,
    class_id: Class::Arcanist,
    name: "Corrosion",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SCRATCH_DEALER: Skill = Skill {
    id: 19320,
    class_id: Class::Arcanist,
    name: "Scratch Dealer",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MAYHEM: Skill = Skill {
    id: 19280,
    class_id: Class::Arcanist,
    name: "Mayhem",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EVOKE_19030: Skill = Skill {
    id: 19030,
    class_id: Class::Arcanist,
    name: "Evoke",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TWISTED_FATE: Skill = Skill {
    id: 19091,
    class_id: Class::Arcanist,
    name: "Twisted Fate",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const VANISH: Skill = Skill {
    id: 19100,
    class_id: Class::Arcanist,
    name: "Vanish",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEALERS_FLIP: Skill = Skill {
    id: 19230,
    class_id: Class::Arcanist,
    name: "Dealer's Flip",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CALL_OF_DESTINY: Skill = Skill {
    id: 19180,
    class_id: Class::Arcanist,
    name: "Call of Destiny",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THE_DEVIL: Skill = Skill {
    id: 19350,
    class_id: Class::Arcanist,
    name: "The Devil",
    cooldown: 60,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THREE_HEADED_SNAKE: Skill = Skill {
    id: 19097,
    class_id: Class::Arcanist,
    name: "Three-Headed Snake",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EMPRESS: Skill = Skill {
    id: 19283,
    class_id: Class::Arcanist,
    name: "Empress",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATHBOUND: Skill = Skill {
    id: 19330,
    class_id: Class::Arcanist,
    name: "Deathbound",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WHEEL_OF_FORTUNE: Skill = Skill {
    id: 19095,
    class_id: Class::Arcanist,
    name: "Wheel of Fortune",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOVEREIGN: Skill = Skill {
    id: 19287,
    class_id: Class::Arcanist,
    name: "Sovereign",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STAR: Skill = Skill {
    id: 19094,
    class_id: Class::Arcanist,
    name: "Star",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RETURN: Skill = Skill {
    id: 19260,
    class_id: Class::Arcanist,
    name: "Return",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EVOKE_19300: Skill = Skill {
    id: 19300,
    class_id: Class::Arcanist,
    name: "Evoke",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CELESTIAL_RAIN: Skill = Skill {
    id: 19140,
    class_id: Class::Arcanist,
    name: "Celestial Rain",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THE_SUN: Skill = Skill {
    id: 19340,
    class_id: Class::Arcanist,
    name: "The Sun",
    cooldown: 80,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERENDIPITY: Skill = Skill {
    id: 19240,
    class_id: Class::Arcanist,
    name: "Serendipity",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GHOST: Skill = Skill {
    id: 19090,
    class_id: Class::Arcanist,
    name: "Ghost",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const QUADRA_ACCELERATE: Skill = Skill {
    id: 19170,
    class_id: Class::Arcanist,
    name: "Quadra Accelerate",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EMPEROR: Skill = Skill {
    id: 19282,
    class_id: Class::Arcanist,
    name: "Emperor",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MYSTERIOUS_STAMPEDE_19215: Skill = Skill {
    id: 19215,
    class_id: Class::Arcanist,
    name: "Mysterious Stampede",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARIRIN_CHARGE_20138: Skill = Skill {
    id: 20138,
    class_id: Class::Summoner,
    name: "Maririn - Charge",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARIRIN: Skill = Skill {
    id: 20130,
    class_id: Class::Summoner,
    name: "Maririn",
    cooldown: 45,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const KELSION_20291: Skill = Skill {
    id: 20291,
    class_id: Class::Summoner,
    name: "Kelsion",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WINGED_SPIRIT_20220: Skill = Skill {
    id: 20220,
    class_id: Class::Summoner,
    name: "Winged Spirit",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JAHIA_LIGHEAS_20081: Skill = Skill {
    id: 20081,
    class_id: Class::Summoner,
    name: "Jahia & Ligheas",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EARTH_COLLAPSE: Skill = Skill {
    id: 20240,
    class_id: Class::Summoner,
    name: "Earth Collapse",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLEETING_GALE_BIRD: Skill = Skill {
    id: 20060,
    class_id: Class::Summoner,
    name: "Fleeting Gale Bird",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REINES_PROTECTION: Skill = Skill {
    id: 20040,
    class_id: Class::Summoner,
    name: "Reine's Protection",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ELECTRICITY_RELEASE_20231: Skill = Skill {
    id: 20231,
    class_id: Class::Summoner,
    name: "Electricity Release",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ELECTRICITY_RELEASE_20230: Skill = Skill {
    id: 20230,
    class_id: Class::Summoner,
    name: "Electricity Release",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_EXPLOSION_20050: Skill = Skill {
    id: 20050,
    class_id: Class::Summoner,
    name: "Flash Explosion",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRYSTALLINE_MAGICK: Skill = Skill {
    id: 20020,
    class_id: Class::Summoner,
    name: "Crystalline Magick",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARIRIN_TAUNT_20347: Skill = Skill {
    id: 20347,
    class_id: Class::Summoner,
    name: "Maririn - Taunt",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAURU: Skill = Skill {
    id: 20110,
    class_id: Class::Summoner,
    name: "Pauru",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STEED_CHARGE: Skill = Skill {
    id: 20030,
    class_id: Class::Summoner,
    name: "Steed Charge",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RELEASED_WILL: Skill = Skill {
    id: 20200,
    class_id: Class::Summoner,
    name: "Released Will",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAURU_FLAME_BREATH_20120: Skill = Skill {
    id: 20120,
    class_id: Class::Summoner,
    name: "Pauru - Flame Breath",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WATER_ELEMENTAL: Skill = Skill {
    id: 20250,
    class_id: Class::Summoner,
    name: "Water Elemental",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ELEMENTAL_WINGS: Skill = Skill {
    id: 20210,
    class_id: Class::Summoner,
    name: "Elemental Wings",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ELCID: Skill = Skill {
    id: 20090,
    class_id: Class::Summoner,
    name: "Elcid",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARIRIN_STAGGER_20349: Skill = Skill {
    id: 20349,
    class_id: Class::Summoner,
    name: "Maririn - Stagger",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ALIMAJI_20071: Skill = Skill {
    id: 20071,
    class_id: Class::Summoner,
    name: "Alimaji",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AKIR_20310: Skill = Skill {
    id: 20310,
    class_id: Class::Summoner,
    name: "Akir",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const IGNA: Skill = Skill {
    id: 20340,
    class_id: Class::Summoner,
    name: "Igna",
    cooldown: 85,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const KELSION_THUNDERCRACK: Skill = Skill {
    id: 20294,
    class_id: Class::Summoner,
    name: "Kelsion - Thundercrack",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHOENIX_20281: Skill = Skill {
    id: 20281,
    class_id: Class::Summoner,
    name: "Phoenix",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ELECTRIC_STORM: Skill = Skill {
    id: 20270,
    class_id: Class::Summoner,
    name: "Electric Storm",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAURU_SELF_DESTRUCT_20125: Skill = Skill {
    id: 20125,
    class_id: Class::Summoner,
    name: "Pauru - Self-Destruct",
    cooldown: 3,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WINGED_SPIRIT_20222: Skill = Skill {
    id: 20222,
    class_id: Class::Summoner,
    name: "Winged Spirit",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JUDGE_KELSION: Skill = Skill {
    id: 20350,
    class_id: Class::Summoner,
    name: "Judge Kelsion",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WINGED_SPIRIT_20221: Skill = Skill {
    id: 20221,
    class_id: Class::Summoner,
    name: "Winged Spirit",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_EXPLOSION_20051: Skill = Skill {
    id: 20051,
    class_id: Class::Summoner,
    name: "Flash Explosion",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const KELSION_20293: Skill = Skill {
    id: 20293,
    class_id: Class::Summoner,
    name: "Kelsion",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAURU_SELF_DESTRUCT_20335: Skill = Skill {
    id: 20335,
    class_id: Class::Summoner,
    name: "Pauru - Self-Destruct",
    cooldown: 3,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHOENIX_20280: Skill = Skill {
    id: 20280,
    class_id: Class::Summoner,
    name: "Phoenix",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BAGRONS_WRATH: Skill = Skill {
    id: 20300,
    class_id: Class::Summoner,
    name: "Bagron's Wrath",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ALIMAJI_20070: Skill = Skill {
    id: 20070,
    class_id: Class::Summoner,
    name: "Alimaji",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const KELSION_20290: Skill = Skill {
    id: 20290,
    class_id: Class::Summoner,
    name: "Kelsion",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAURU_FLAME_BREATH_20330: Skill = Skill {
    id: 20330,
    class_id: Class::Summoner,
    name: "Pauru - Flame Breath",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const OSH_20181: Skill = Skill {
    id: 20181,
    class_id: Class::Summoner,
    name: "Osh",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const KELSION_20292: Skill = Skill {
    id: 20292,
    class_id: Class::Summoner,
    name: "Kelsion",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STICKY_MOSS_SWAMP: Skill = Skill {
    id: 20180,
    class_id: Class::Summoner,
    name: "Sticky Moss Swamp",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AKIR_20311: Skill = Skill {
    id: 20311,
    class_id: Class::Summoner,
    name: "Akir",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AKIR_BURST: Skill = Skill {
    id: 20312,
    class_id: Class::Summoner,
    name: "Akir Burst",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHURDI: Skill = Skill {
    id: 20160,
    class_id: Class::Summoner,
    name: "Shurdi",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const IGNA_BREATH: Skill = Skill {
    id: 20357,
    class_id: Class::Summoner,
    name: "Igna - Breath",
    cooldown: 50,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ANCIENT_SPEAR: Skill = Skill {
    id: 20260,
    class_id: Class::Summoner,
    name: "Ancient Spear",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JAHIA_LIGHEAS_20080: Skill = Skill {
    id: 20080,
    class_id: Class::Summoner,
    name: "Jahia & Ligheas",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARIRIN_TAUNT_20137: Skill = Skill {
    id: 20137,
    class_id: Class::Summoner,
    name: "Maririn - Taunt",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARIRIN_STAGGER_20139: Skill = Skill {
    id: 20139,
    class_id: Class::Summoner,
    name: "Maririn - Stagger",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARIPOSA: Skill = Skill {
    id: 20320,
    class_id: Class::Summoner,
    name: "Mariposa",
    cooldown: 70,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARIRIN_CHARGE_20348: Skill = Skill {
    id: 20348,
    class_id: Class::Summoner,
    name: "Maririn - Charge",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WINGED_SPIRIT_20223: Skill = Skill {
    id: 20223,
    class_id: Class::Summoner,
    name: "Winged Spirit",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BAGRONS_FRENZY: Skill = Skill {
    id: 20370,
    class_id: Class::Summoner,
    name: "Bagron's Frenzy",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const OSH_20170: Skill = Skill {
    id: 20170,
    class_id: Class::Summoner,
    name: "Osh",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HARP_OF_RHYTHM: Skill = Skill {
    id: 21180,
    class_id: Class::Bard,
    name: "Harp of Rhythm",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARCH: Skill = Skill {
    id: 21270,
    class_id: Class::Bard,
    name: "March",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SONIC_VIBRATION: Skill = Skill {
    id: 21170,
    class_id: Class::Bard,
    name: "Sonic Vibration",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: Some(SONIC_VIBRATION_211751),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERENADE_OF_COURAGE_21141: Skill = Skill {
    id: 21141,
    class_id: Class::Bard,
    name: "Serenade of Courage",
    cooldown: 20,
    kind: SkillType::Identity,
    skill_buff: Some(SERENADE_OF_COURAGE_211410),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERENADE_OF_SALVATION_21132: Skill = Skill {
    id: 21132,
    class_id: Class::Bard,
    name: "Serenade of Salvation",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MAJOR_CHORD: Skill = Skill {
    id: 21340,
    class_id: Class::Bard,
    name: "Major Chord",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CONCERTO: Skill = Skill {
    id: 21330,
    class_id: Class::Bard,
    name: "Concerto",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TEMPEST_21149: Skill = Skill {
    id: 21149,
    class_id: Class::Bard,
    name: "Tempest",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RHAPSODY_OF_LIGHT: Skill = Skill {
    id: 21260,
    class_id: Class::Bard,
    name: "Rhapsody of Light",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOUND_WAVE: Skill = Skill {
    id: 21050,
    class_id: Class::Bard,
    name: "Sound Wave",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERENADE_OF_SALVATION_21131: Skill = Skill {
    id: 21131,
    class_id: Class::Bard,
    name: "Serenade of Salvation",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERENADE_OF_COURAGE_21140: Skill = Skill {
    id: 21140,
    class_id: Class::Bard,
    name: "Serenade of Courage",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SYMPHONIA: Skill = Skill {
    id: 21230,
    class_id: Class::Bard,
    name: "Symphonia",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DISSONANCE: Skill = Skill {
    id: 21060,
    class_id: Class::Bard,
    name: "Dissonance",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SYMPHONY_MELODY: Skill = Skill {
    id: 21320,
    class_id: Class::Bard,
    name: "Symphony Melody",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PRELUDE_OF_STORM: Skill = Skill {
    id: 21080,
    class_id: Class::Bard,
    name: "Prelude of Storm",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NOTE_BUNDLE: Skill = Skill {
    id: 21110,
    class_id: Class::Bard,
    name: "Note Bundle",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MINOR_CHORD: Skill = Skill {
    id: 21350,
    class_id: Class::Bard,
    name: "Minor Chord",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PRELUDE_OF_DEATH: Skill = Skill {
    id: 21240,
    class_id: Class::Bard,
    name: "Prelude of Death",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const VIVACE: Skill = Skill {
    id: 21310,
    class_id: Class::Bard,
    name: "Vivace",
    cooldown: 70,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOUND_SHOCK: Skill = Skill {
    id: 21020,
    class_id: Class::Bard,
    name: "Sound Shock",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERENADE_OF_COURAGE_21142: Skill = Skill {
    id: 21142,
    class_id: Class::Bard,
    name: "Serenade of Courage",
    cooldown: 20,
    kind: SkillType::Identity,
    skill_buff: Some(SERENADE_OF_COURAGE_211420),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERENADE_OF_COURAGE_21143: Skill = Skill {
    id: 21143,
    class_id: Class::Bard,
    name: "Serenade of Courage",
    cooldown: 20,
    kind: SkillType::Identity,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RHYTHM_BUCKSHOT: Skill = Skill {
    id: 21150,
    class_id: Class::Bard,
    name: "Rhythm Buckshot",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SONATINA: Skill = Skill {
    id: 21290,
    class_id: Class::Bard,
    name: "Sonatina",
    cooldown: 21,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TEMPEST_21147: Skill = Skill {
    id: 21147,
    class_id: Class::Bard,
    name: "Tempest",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ARIA: Skill = Skill {
    id: 21300,
    class_id: Class::Bard,
    name: "Aria",
    cooldown: 90,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TEMPEST_21148: Skill = Skill {
    id: 21148,
    class_id: Class::Bard,
    name: "Tempest",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOUNDHOLIC: Skill = Skill {
    id: 21120,
    class_id: Class::Bard,
    name: "Soundholic",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GUARDIAN_TUNE: Skill = Skill {
    id: 21250,
    class_id: Class::Bard,
    name: "Guardian Tune",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HEAVENLY_TUNE: Skill = Skill {
    id: 21160,
    class_id: Class::Bard,
    name: "Heavenly Tune",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: Some(HEAVENLY_TUNE_211606),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOUND_ILLUSION: Skill = Skill {
    id: 21100,
    class_id: Class::Bard,
    name: "Sound Illusion",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERENADE_OF_SALVATION_21130: Skill = Skill {
    id: 21130,
    class_id: Class::Bard,
    name: "Serenade of Salvation",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERENADE_OF_SALVATION_21133: Skill = Skill {
    id: 21133,
    class_id: Class::Bard,
    name: "Serenade of Salvation",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STIGMA: Skill = Skill {
    id: 21090,
    class_id: Class::Bard,
    name: "Stigma",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ORATORIO: Skill = Skill {
    id: 21280,
    class_id: Class::Bard,
    name: "Oratorio",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WIND_OF_MUSIC_21070: Skill = Skill {
    id: 21070,
    class_id: Class::Bard,
    name: "Wind of Music",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WIND_OF_MUSIC_21079: Skill = Skill {
    id: 21079,
    class_id: Class::Bard,
    name: "Wind of Music",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REVERSE_GRAVITY: Skill = Skill {
    id: 37280,
    class_id: Class::Sorceress,
    name: "Reverse Gravity",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const APOCALYPSE: Skill = Skill {
    id: 37390,
    class_id: Class::Sorceress,
    name: "Apocalypse",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ARCANE_RUPTURE_37101: Skill = Skill {
    id: 37101,
    class_id: Class::Sorceress,
    name: "Arcane Rupture",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ICE_SHOWER: Skill = Skill {
    id: 37220,
    class_id: Class::Sorceress,
    name: "Ice Shower",
    cooldown: 11,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DOOMSDAY: Skill = Skill {
    id: 37350,
    class_id: Class::Sorceress,
    name: "Doomsday",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const APOCALYPSE_CALL: Skill = Skill {
    id: 37370,
    class_id: Class::Sorceress,
    name: "Apocalypse Call",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_REACTION: Skill = Skill {
    id: 37260,
    class_id: Class::Sorceress,
    name: "Esoteric Reaction",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RUSHING_GLACIER: Skill = Skill {
    id: 37410,
    class_id: Class::Sorceress,
    name: "Rushing Glacier",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLINK: Skill = Skill {
    id: 37110,
    class_id: Class::Sorceress,
    name: "Blink",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SQUALL: Skill = Skill {
    id: 37310,
    class_id: Class::Sorceress,
    name: "Squall",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FROSTS_CALL_37341: Skill = Skill {
    id: 37341,
    class_id: Class::Sorceress,
    name: "Frost's Call",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EXPLOSION: Skill = Skill {
    id: 37330,
    class_id: Class::Sorceress,
    name: "Explosion",
    cooldown: 28,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENERGY_DISCHARGE: Skill = Skill {
    id: 37240,
    class_id: Class::Sorceress,
    name: "Energy Discharge",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHTNING_VORTEX: Skill = Skill {
    id: 37210,
    class_id: Class::Sorceress,
    name: "Lightning Vortex",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENVISKAS_MIGHT: Skill = Skill {
    id: 37360,
    class_id: Class::Sorceress,
    name: "Enviska's Might",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ARCANE_RUPTURE_37100: Skill = Skill {
    id: 37100,
    class_id: Class::Sorceress,
    name: "Arcane Rupture",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TRANSCENDENT_JUDGMENT: Skill = Skill {
    id: 37380,
    class_id: Class::Sorceress,
    name: "Transcendent Judgment",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PUNISHING_STRIKE_37270: Skill = Skill {
    id: 37270,
    class_id: Class::Sorceress,
    name: "Punishing Strike",
    cooldown: 28,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHASE_LEAP: Skill = Skill {
    id: 37020,
    class_id: Class::Sorceress,
    name: "Phase Leap",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LAVA_BLAST: Skill = Skill {
    id: 37400,
    class_id: Class::Sorceress,
    name: "Lava Blast",
    cooldown: 75,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLAZE: Skill = Skill {
    id: 37200,
    class_id: Class::Sorceress,
    name: "Blaze",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SERAPHIC_HAIL: Skill = Skill {
    id: 37320,
    class_id: Class::Sorceress,
    name: "Seraphic Hail",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ELEGIANS_TOUCH_37290: Skill = Skill {
    id: 37290,
    class_id: Class::Sorceress,
    name: "Elegian's Touch",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const INFERNO: Skill = Skill {
    id: 37230,
    class_id: Class::Sorceress,
    name: "Inferno",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ELEGIANS_TOUCH_37291: Skill = Skill {
    id: 37291,
    class_id: Class::Sorceress,
    name: "Elegian's Touch",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FROSTS_CALL_37340: Skill = Skill {
    id: 37340,
    class_id: Class::Sorceress,
    name: "Frost's Call",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHTNING_BOLT: Skill = Skill {
    id: 37300,
    class_id: Class::Sorceress,
    name: "Lightning Bolt",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RIME_ARROW: Skill = Skill {
    id: 37250,
    class_id: Class::Sorceress,
    name: "Rime Arrow",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ULTIMATE_SKILL_GREAT_RAGING_DEMON_KICK: Skill = Skill {
    id: 22370,
    class_id: Class::Wardancer,
    name: "Ultimate Skill: Great Raging Demon Kick",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_HEAT_FANG_22060: Skill = Skill {
    id: 22060,
    class_id: Class::Wardancer,
    name: "Flash Heat Fang",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_CALL_OF_THE_WIND_GOD_22110: Skill = Skill {
    id: 22110,
    class_id: Class::Wardancer,
    name: "Esoteric Skill: Call of the Wind God",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHTNING_KICK_22170: Skill = Skill {
    id: 22170,
    class_id: Class::Wardancer,
    name: "Lightning Kick",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWIFT_WIND_KICK_22280: Skill = Skill {
    id: 22280,
    class_id: Class::Wardancer,
    name: "Swift Wind Kick",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWEEPING_KICK_22320: Skill = Skill {
    id: 22320,
    class_id: Class::Wardancer,
    name: "Sweeping Kick",
    cooldown: 25,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_BLAST_FORMATION_22020: Skill = Skill {
    id: 22020,
    class_id: Class::Wardancer,
    name: "Esoteric Skill: Blast Formation",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHARGING_STEPS_22150: Skill = Skill {
    id: 22150,
    class_id: Class::Wardancer,
    name: "Charging Steps",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHOENIX_ADVENT_22130: Skill = Skill {
    id: 22130,
    class_id: Class::Wardancer,
    name: "Phoenix Advent",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ULTIMATE_SKILL_EIGHT_TRIGRAMS_CHAOTIC_STRIKE: Skill = Skill {
    id: 22360,
    class_id: Class::Wardancer,
    name: "Ultimate Skill: Eight Trigrams Chaotic Strike",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ULTIMATE_SKILL_FLASH_RAGE_BLOW: Skill = Skill {
    id: 22310,
    class_id: Class::Wardancer,
    name: "Ultimate Skill: Flash Rage Blow",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ULTIMATE_SKILL_FIST_OF_DOMINANCE: Skill = Skill {
    id: 22330,
    class_id: Class::Wardancer,
    name: "Ultimate Skill: Fist of Dominance",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEADLY_DIVE: Skill = Skill {
    id: 22380,
    class_id: Class::Wardancer,
    name: "Deadly Dive",
    cooldown: 75,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TRIPLE_FIST_22180: Skill = Skill {
    id: 22180,
    class_id: Class::Wardancer,
    name: "Triple Fist",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOON_FLASH_KICK_22190: Skill = Skill {
    id: 22190,
    class_id: Class::Wardancer,
    name: "Moon Flash Kick",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SKY_SHATTERING_BLOW_22160: Skill = Skill {
    id: 22160,
    class_id: Class::Wardancer,
    name: "Sky Shattering Blow",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWEEPING_HIDDEN_DRAGON_22070: Skill = Skill {
    id: 22070,
    class_id: Class::Wardancer,
    name: "Sweeping Hidden Dragon",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_THUNDERCLAP_KICK: Skill = Skill {
    id: 22390,
    class_id: Class::Wardancer,
    name: "Esoteric Skill: Thunderclap Kick",
    cooldown: 60,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_ORIGIN: Skill = Skill {
    id: 22400,
    class_id: Class::Wardancer,
    name: "Esoteric Origin",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_RISING_FIRE_DRAGON: Skill = Skill {
    id: 22040,
    class_id: Class::Wardancer,
    name: "Esoteric Skill: Rising Fire Dragon",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENERGY_COMBUSTION_22290: Skill = Skill {
    id: 22290,
    class_id: Class::Wardancer,
    name: "Energy Combustion",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEAPING_DRAGON: Skill = Skill {
    id: 22350,
    class_id: Class::Wardancer,
    name: "Leaping Dragon",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_AZURE_DRAGON_SUPREME_FIST: Skill = Skill {
    id: 22340,
    class_id: Class::Wardancer,
    name: "Esoteric skill: Azure Dragon Supreme Fist",
    cooldown: 21,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WINDS_WHISPER: Skill = Skill {
    id: 22120,
    class_id: Class::Wardancer,
    name: "Wind's Whisper",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ROAR_OF_COURAGE: Skill = Skill {
    id: 22080,
    class_id: Class::Wardancer,
    name: "Roar of Courage",
    cooldown: 25,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENERGY_COMBUSTION_22291: Skill = Skill {
    id: 22291,
    class_id: Class::Wardancer,
    name: "Energy Combustion",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_SPIRAL_IMPACT_22030: Skill = Skill {
    id: 22030,
    class_id: Class::Wardancer,
    name: "Esoteric Skill: Spiral Impact",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRUSHING_SMITE: Skill = Skill {
    id: 23090,
    class_id: Class::Scrapper,
    name: "Crushing Smite",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHARGING_BLOW: Skill = Skill {
    id: 23020,
    class_id: Class::Scrapper,
    name: "Charging Blow",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EARTH_REND: Skill = Skill {
    id: 23440,
    class_id: Class::Scrapper,
    name: "Earth Rend",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EXPLOSIVE_FIST_23300: Skill = Skill {
    id: 23300,
    class_id: Class::Scrapper,
    name: "Explosive Fist",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHREDDING_STRIKE: Skill = Skill {
    id: 23260,
    class_id: Class::Scrapper,
    name: "Shredding Strike",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DIVINE_DRAGON_CREATION: Skill = Skill {
    id: 23410,
    class_id: Class::Scrapper,
    name: "Divine Dragon Creation",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MYSTERIOUS_ART_BLAST_OF_RUINATION: Skill = Skill {
    id: 23270,
    class_id: Class::Scrapper,
    name: "Mysterious Art: Blast of Ruination",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ROUNDUP_SWEEP: Skill = Skill {
    id: 23160,
    class_id: Class::Scrapper,
    name: "Roundup Sweep",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUPERNOVA_23130: Skill = Skill {
    id: 23130,
    class_id: Class::Scrapper,
    name: "Supernova",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const UNDEFEATED_DRAGON_KING: Skill = Skill {
    id: 23250,
    class_id: Class::Scrapper,
    name: "Undefeated Dragon King",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CONTINUOUS_PUSH: Skill = Skill {
    id: 23200,
    class_id: Class::Scrapper,
    name: "Continuous Push",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const INSTANT_HIT: Skill = Skill {
    id: 23150,
    class_id: Class::Scrapper,
    name: "Instant Hit",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DUCK_23171: Skill = Skill {
    id: 23171,
    class_id: Class::Scrapper,
    name: "Duck",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FIST_OF_THE_WIND_GOD: Skill = Skill {
    id: 23290,
    class_id: Class::Scrapper,
    name: "Fist of the Wind God",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JUDGMENT_23060: Skill = Skill {
    id: 23060,
    class_id: Class::Scrapper,
    name: "Judgment",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FIERCE_TIGER_STRIKE: Skill = Skill {
    id: 23180,
    class_id: Class::Scrapper,
    name: "Fierce Tiger Strike",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const POTENT_RISING_FIST: Skill = Skill {
    id: 23240,
    class_id: Class::Scrapper,
    name: "Potent Rising Fist",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const IRON_CANNON_BLOW: Skill = Skill {
    id: 23230,
    class_id: Class::Scrapper,
    name: "Iron Cannon Blow",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLAZING_BOMBARDMENT: Skill = Skill {
    id: 23420,
    class_id: Class::Scrapper,
    name: "Blazing Bombardment",
    cooldown: 60,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DUCK_23172: Skill = Skill {
    id: 23172,
    class_id: Class::Scrapper,
    name: "Duck",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BATTERING_FISTS: Skill = Skill {
    id: 23220,
    class_id: Class::Scrapper,
    name: "Battering Fists",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HEAVEN_AND_EARTH_STRIKE: Skill = Skill {
    id: 23430,
    class_id: Class::Scrapper,
    name: "Heaven and Earth Strike",
    cooldown: 65,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHAIN_DESTRUCTION_FIST: Skill = Skill {
    id: 23100,
    class_id: Class::Scrapper,
    name: "Chain Destruction Fist",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUPREME_HEAVEN_SHATTERING_FIST: Skill = Skill {
    id: 23400,
    class_id: Class::Scrapper,
    name: "Supreme: Heaven-Shattering Fist",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRITICAL_BLOW: Skill = Skill {
    id: 23210,
    class_id: Class::Scrapper,
    name: "Critical Blow",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TENACITY_RELEASE: Skill = Skill {
    id: 23280,
    class_id: Class::Scrapper,
    name: "Tenacity Release",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DRAGON_ADVENT: Skill = Skill {
    id: 23050,
    class_id: Class::Scrapper,
    name: "Dragon Advent",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_RATTLE: Skill = Skill {
    id: 23110,
    class_id: Class::Scrapper,
    name: "Death Rattle",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DUCK_23170: Skill = Skill {
    id: 23170,
    class_id: Class::Scrapper,
    name: "Duck",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RULE_THE_WORLD: Skill = Skill {
    id: 24320,
    class_id: Class::Soulfist,
    name: "Rule the World",
    cooldown: 80,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOWBREAKER: Skill = Skill {
    id: 24200,
    class_id: Class::Soulfist,
    name: "Shadowbreaker",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BOLTING_CRASH: Skill = Skill {
    id: 24220,
    class_id: Class::Soulfist,
    name: "Bolting Crash",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEVEL_3_HYPE_24022: Skill = Skill {
    id: 24022,
    class_id: Class::Soulfist,
    name: "Level 3 Hype",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENERGY_RELEASE: Skill = Skill {
    id: 24250,
    class_id: Class::Soulfist,
    name: "Energy Release",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NEBULOUS_STEP: Skill = Skill {
    id: 24150,
    class_id: Class::Soulfist,
    name: "Nebulous Step",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ILLUSION_STRIKE: Skill = Skill {
    id: 24290,
    class_id: Class::Soulfist,
    name: "Illusion Strike",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_STEP_24242: Skill = Skill {
    id: 24242,
    class_id: Class::Soulfist,
    name: "Flash Step",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENERGY_BLAST: Skill = Skill {
    id: 24040,
    class_id: Class::Soulfist,
    name: "Energy Blast",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CELESTIAL_PALM: Skill = Skill {
    id: 24120,
    class_id: Class::Soulfist,
    name: "Celestial Palm",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SKY_SLASH: Skill = Skill {
    id: 24330,
    class_id: Class::Soulfist,
    name: "Sky Slash",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEVEL_3_HYPE_24025: Skill = Skill {
    id: 24025,
    class_id: Class::Soulfist,
    name: "Level 3 Hype",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HYPE: Skill = Skill {
    id: 24020,
    class_id: Class::Soulfist,
    name: "Hype",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEVEL_2_HYPE: Skill = Skill {
    id: 24021,
    class_id: Class::Soulfist,
    name: "Level 2 Hype",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FALLING_SUN: Skill = Skill {
    id: 24300,
    class_id: Class::Soulfist,
    name: "Falling Sun",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WORLD_DECIMATION: Skill = Skill {
    id: 24050,
    class_id: Class::Soulfist,
    name: "World Decimation",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MAGNETIC_PALM: Skill = Skill {
    id: 24170,
    class_id: Class::Soulfist,
    name: "Magnetic Palm",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_STEP_24241: Skill = Skill {
    id: 24241,
    class_id: Class::Soulfist,
    name: "Flash Step",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRIPPLING_BARRIER: Skill = Skill {
    id: 24140,
    class_id: Class::Soulfist,
    name: "Crippling Barrier",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FORCE_ORB: Skill = Skill {
    id: 24230,
    class_id: Class::Soulfist,
    name: "Force Orb",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUPERNOVA_PURGATION_RAY: Skill = Skill {
    id: 24310,
    class_id: Class::Soulfist,
    name: "Supernova Purgation Ray",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHTNING_PALM: Skill = Skill {
    id: 24080,
    class_id: Class::Soulfist,
    name: "Lightning Palm",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TEMPEST_BLAST: Skill = Skill {
    id: 24190,
    class_id: Class::Soulfist,
    name: "Tempest Blast",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PALM_BURST: Skill = Skill {
    id: 24110,
    class_id: Class::Soulfist,
    name: "Palm Burst",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENERGY_BULLET: Skill = Skill {
    id: 24090,
    class_id: Class::Soulfist,
    name: "Energy Bullet",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEADLY_FINGER: Skill = Skill {
    id: 24260,
    class_id: Class::Soulfist,
    name: "Deadly Finger",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PULVERIZING_PALM: Skill = Skill {
    id: 24180,
    class_id: Class::Soulfist,
    name: "Pulverizing Palm",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DECIMATION_RAY: Skill = Skill {
    id: 24270,
    class_id: Class::Soulfist,
    name: "Decimation Ray",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_STEP_24240: Skill = Skill {
    id: 24240,
    class_id: Class::Soulfist,
    name: "Flash Step",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEVEL_3_HYPE_24023: Skill = Skill {
    id: 24023,
    class_id: Class::Soulfist,
    name: "Level 3 Hype",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MERCILESS_PUMMEL: Skill = Skill {
    id: 24210,
    class_id: Class::Soulfist,
    name: "Merciless Pummel",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BREAKTHROUGH: Skill = Skill {
    id: 34520,
    class_id: Class::Glaivier,
    name: "Breakthrough",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HALF_MOON_SLASH: Skill = Skill {
    id: 34110,
    class_id: Class::Glaivier,
    name: "Half Moon Slash",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const YEON_STYLE_SPEAR_TECHNIQUE_GALAXY_FLYING_SPEAR: Skill = Skill {
    id: 34620,
    class_id: Class::Glaivier,
    name: "Yeon-style Spear Technique: Galaxy Flying Spear",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STAMPEDING_SLASH_34061: Skill = Skill {
    id: 34061,
    class_id: Class::Glaivier,
    name: "Stampeding Slash",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOCUS_STANCE_34001: Skill = Skill {
    id: 34001,
    class_id: Class::Glaivier,
    name: "Focus Stance",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RED_DRAGONS_HORN: Skill = Skill {
    id: 34590,
    class_id: Class::Glaivier,
    name: "Red Dragon's Horn",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEADLY_RED_DRAGON: Skill = Skill {
    id: 34650,
    class_id: Class::Glaivier,
    name: "Deadly Red Dragon",
    cooldown: 55,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DRAGONS_RAMPAGE: Skill = Skill {
    id: 34640,
    class_id: Class::Glaivier,
    name: "Dragon's Rampage",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_KICK: Skill = Skill {
    id: 34080,
    class_id: Class::Glaivier,
    name: "Flash Kick",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STAMPEDING_SLASH_34060: Skill = Skill {
    id: 34060,
    class_id: Class::Glaivier,
    name: "Stampeding Slash",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STARFALL_POUNCE: Skill = Skill {
    id: 34570,
    class_id: Class::Glaivier,
    name: "Starfall Pounce",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOCUS_STANCE_34000: Skill = Skill {
    id: 34000,
    class_id: Class::Glaivier,
    name: "Focus Stance",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHACKLING_BLUE_DRAGON: Skill = Skill {
    id: 34170,
    class_id: Class::Glaivier,
    name: "Shackling Blue Dragon",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WHEEL_OF_BLADES: Skill = Skill {
    id: 34070,
    class_id: Class::Glaivier,
    name: "Wheel of Blades",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THRUST_OF_DESTRUCTION: Skill = Skill {
    id: 34560,
    class_id: Class::Glaivier,
    name: "Thrust of Destruction",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THORN_JAB: Skill = Skill {
    id: 34090,
    class_id: Class::Glaivier,
    name: "Thorn Jab",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CUTTING_WIND: Skill = Skill {
    id: 34130,
    class_id: Class::Glaivier,
    name: "Cutting Wind",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const YEON_STYLE_ENCORE: Skill = Skill {
    id: 34670,
    class_id: Class::Glaivier,
    name: "Yeon-Style Encore",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRALING_SPEAR: Skill = Skill {
    id: 34540,
    class_id: Class::Glaivier,
    name: "Spiraling Spear",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const YEON_STYLE_TECHNIQUE: Skill = Skill {
    id: 34660,
    class_id: Class::Glaivier,
    name: "Yeon-Style Technique",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOUL_CUTTER: Skill = Skill {
    id: 34140,
    class_id: Class::Glaivier,
    name: "Soul Cutter",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHAIN_SLASH: Skill = Skill {
    id: 34120,
    class_id: Class::Glaivier,
    name: "Chain Slash",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MIRAGE_DASH: Skill = Skill {
    id: 34020,
    class_id: Class::Glaivier,
    name: "Mirage Dash",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DOUBLE_STRIKE: Skill = Skill {
    id: 34040,
    class_id: Class::Glaivier,
    name: "Double Strike",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DRAGONSCALE_DEFENSE: Skill = Skill {
    id: 34580,
    class_id: Class::Glaivier,
    name: "Dragonscale Defense",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WINDSPLITTER: Skill = Skill {
    id: 34050,
    class_id: Class::Glaivier,
    name: "Windsplitter",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAGING_DRAGON_SLASH: Skill = Skill {
    id: 34150,
    class_id: Class::Glaivier,
    name: "Raging Dragon Slash",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPEAR_DIVE: Skill = Skill {
    id: 34160,
    class_id: Class::Glaivier,
    name: "Spear Dive",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLURRY_STANCE_34500: Skill = Skill {
    id: 34500,
    class_id: Class::Glaivier,
    name: "Flurry Stance",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const YEON_STYLE_SPEAR_TECHNIQUE_DRAGON_CAVALRY_UNITY_SLASH: Skill = Skill {
    id: 34630,
    class_id: Class::Glaivier,
    name: "Yeon-style Spear Technique: Dragon Cavalry Unity Slash",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLUE_DRAGONS_CLAW: Skill = Skill {
    id: 34100,
    class_id: Class::Glaivier,
    name: "Blue Dragon's Claw",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLURRY_STANCE_34501: Skill = Skill {
    id: 34501,
    class_id: Class::Glaivier,
    name: "Flurry Stance",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOUR_HEADED_DRAGON: Skill = Skill {
    id: 34550,
    class_id: Class::Glaivier,
    name: "4-Headed Dragon",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const YEON_STYLE_SPEAR_TECHNIQUE_SPEAR_METEOR: Skill = Skill {
    id: 34600,
    class_id: Class::Glaivier,
    name: "Yeon-style Spear Technique: Spear Meteor",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const YEON_STYLE_SPEAR_TECHNIQUE_STORMING_RED_DRAGON: Skill = Skill {
    id: 34610,
    class_id: Class::Glaivier,
    name: "Yeon-style Spear Technique: Storming Red Dragon",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWIFT_WIND_KICK_39280: Skill = Skill {
    id: 39280,
    class_id: Class::Striker,
    name: "Swift Wind Kick",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_HEAT_FANG_39060: Skill = Skill {
    id: 39060,
    class_id: Class::Striker,
    name: "Flash Heat Fang",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_CHARGING_KICK: Skill = Skill {
    id: 39360,
    class_id: Class::Striker,
    name: "Esoteric Skill: Charging Kick",
    cooldown: 95,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SKY_SHATTERING_BLOW_39160: Skill = Skill {
    id: 39160,
    class_id: Class::Striker,
    name: "Sky Shattering Blow",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_TIGER_EMERGES: Skill = Skill {
    id: 39040,
    class_id: Class::Striker,
    name: "Esoteric Skill: Tiger Emerges",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ORBS_BLESSING: Skill = Skill {
    id: 39380,
    class_id: Class::Striker,
    name: "Orb's Blessing",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ADVANCING_STRIKE: Skill = Skill {
    id: 39370,
    class_id: Class::Striker,
    name: "Advancing Strike",
    cooldown: 45,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_LIGHTNING_TIGER_STRIKE: Skill = Skill {
    id: 39100,
    class_id: Class::Striker,
    name: "Esoteric Skill: Lightning Tiger Strike",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STORM_DRAGON_KICK_39122: Skill = Skill {
    id: 39122,
    class_id: Class::Striker,
    name: "Storm Dragon Kick",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STORM_DRAGON_KICK_39120: Skill = Skill {
    id: 39120,
    class_id: Class::Striker,
    name: "Storm Dragon Kick",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWEEPING_HIDDEN_DRAGON_39070: Skill = Skill {
    id: 39070,
    class_id: Class::Striker,
    name: "Sweeping Hidden Dragon",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWEEPING_KICK_39320: Skill = Skill {
    id: 39320,
    class_id: Class::Striker,
    name: "Sweeping Kick",
    cooldown: 25,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const VICIOUS_TIGER_DANCE: Skill = Skill {
    id: 39290,
    class_id: Class::Striker,
    name: "Vicious Tiger Dance",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHARGING_STEPS_39150: Skill = Skill {
    id: 39150,
    class_id: Class::Striker,
    name: "Charging Steps",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_CALL_OF_THE_WIND_GOD_39110: Skill = Skill {
    id: 39110,
    class_id: Class::Striker,
    name: "Esoteric Skill: Call of the Wind God",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BERSERK_CIRCLE: Skill = Skill {
    id: 39090,
    class_id: Class::Striker,
    name: "Berserk Circle",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TRIPLE_FIST_39180: Skill = Skill {
    id: 39180,
    class_id: Class::Striker,
    name: "Triple Fist",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_SPIRAL_IMPACT_39030: Skill = Skill {
    id: 39030,
    class_id: Class::Striker,
    name: "Esoteric Skill: Spiral Impact",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ULTIMATE_SKILL_NOVA_BLAST: Skill = Skill {
    id: 39310,
    class_id: Class::Striker,
    name: "Ultimate Skill: Nova Blast",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ULTIMATE_SKILL_MOUNTAIN_LORDS_EXPLOSIVE_ROAR: Skill = Skill {
    id: 39350,
    class_id: Class::Striker,
    name: "Ultimate Skill: Mountain Lord's Explosive Roar",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHTNING_WHISPER: Skill = Skill {
    id: 39080,
    class_id: Class::Striker,
    name: "Lightning Whisper",
    cooldown: 25,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHOENIX_ADVENT_39130: Skill = Skill {
    id: 39130,
    class_id: Class::Striker,
    name: "Phoenix Advent",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LIGHTNING_KICK_39170: Skill = Skill {
    id: 39170,
    class_id: Class::Striker,
    name: "Lightning Kick",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ULTIMATE_SKILL_THUNDERBOLT_KICK: Skill = Skill {
    id: 39340,
    class_id: Class::Striker,
    name: "Ultimate Skill: Thunderbolt Kick",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOON_FLASH_KICK_39190: Skill = Skill {
    id: 39190,
    class_id: Class::Striker,
    name: "Moon Flash Kick",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ULTIMATE_SKILL_POTENT_HEAVENLY_KICK: Skill = Skill {
    id: 39330,
    class_id: Class::Striker,
    name: "Ultimate Skill: Potent Heavenly Kick",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STORM_DRAGON_KICK_39121: Skill = Skill {
    id: 39121,
    class_id: Class::Striker,
    name: "Storm Dragon Kick",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ESOTERIC_SKILL_BLAST_FORMATION_39020: Skill = Skill {
    id: 39020,
    class_id: Class::Striker,
    name: "Esoteric Skill: Blast Formation",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CELESTIAL_FIST: Skill = Skill {
    id: 47310,
    class_id: Class::Breaker,
    name: "Celestial Fist",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ASURA_FEATHERWEIGHT: Skill = Skill {
    id: 47031,
    class_id: Class::Breaker,
    name: "Asura Featherweight",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EYE_OF_THE_STORM: Skill = Skill {
    id: 47270,
    class_id: Class::Breaker,
    name: "Eye of the Storm",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ASURA_DESTRUCTION_BASIC_ATTACK: Skill = Skill {
    id: 47020,
    class_id: Class::Breaker,
    name: "Asura Destruction Basic Attack",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRAWL_KINGS_ADVANCE: Skill = Skill {
    id: 47100,
    class_id: Class::Breaker,
    name: "Brawl King's Advance",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const OBLITERATION: Skill = Skill {
    id: 47190,
    class_id: Class::Breaker,
    name: "Obliteration",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASHSTEP: Skill = Skill {
    id: 47150,
    class_id: Class::Breaker,
    name: "Flashstep",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HEAVENLY_KINGS_WAR_DANCE: Skill = Skill {
    id: 47280,
    class_id: Class::Breaker,
    name: "Heavenly King's War Dance",
    cooldown: 64,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FEATHERWEIGHT: Skill = Skill {
    id: 47030,
    class_id: Class::Breaker,
    name: "Featherweight",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HAYMAKER: Skill = Skill {
    id: 47200,
    class_id: Class::Breaker,
    name: "Haymaker",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PUNISHING_WAVE: Skill = Skill {
    id: 47180,
    class_id: Class::Breaker,
    name: "Punishing Wave",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRATER_STRIKE: Skill = Skill {
    id: 47120,
    class_id: Class::Breaker,
    name: "Crater Strike",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ASURA_DESTRUCTION: Skill = Skill {
    id: 47940,
    class_id: Class::Breaker,
    name: "Asura Destruction",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HEAVENLY_PUNISHMENT: Skill = Skill {
    id: 47300,
    class_id: Class::Breaker,
    name: "Heavenly Punishment",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const YEON_GALESTORM_STRIKE: Skill = Skill {
    id: 47250,
    class_id: Class::Breaker,
    name: "Yeon Galestorm Strike",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ZAFFRE_NOVA: Skill = Skill {
    id: 47240,
    class_id: Class::Breaker,
    name: "Zaffre Nova",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ADAMANTINE_ASSAULT: Skill = Skill {
    id: 47170,
    class_id: Class::Breaker,
    name: "Adamantine Assault",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BUTTERFLY_STING: Skill = Skill {
    id: 47920,
    class_id: Class::Breaker,
    name: "Butterfly Sting",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRAWL_KING_TWELVE_FORMS_VIOLENT_WAVES: Skill = Skill {
    id: 47970,
    class_id: Class::Breaker,
    name: "Brawl King Twelve Forms: Violent Waves",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NEBULA_CRUSHER: Skill = Skill {
    id: 47290,
    class_id: Class::Breaker,
    name: "Nebula Crusher",
    cooldown: 72,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HURRICANE_CHAIN: Skill = Skill {
    id: 47110,
    class_id: Class::Breaker,
    name: "Hurricane Chain",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRAWL_KING_TWELVE_FORMS_FALLING_BLOSSOMS: Skill = Skill {
    id: 47950,
    class_id: Class::Breaker,
    name: "Brawl King Twelve Forms: Falling Blossoms",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRAWL_KING_STANCE: Skill = Skill {
    id: 47930,
    class_id: Class::Breaker,
    name: "Brawl King Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CELESTIAL_FORCE_BARRAGE: Skill = Skill {
    id: 47260,
    class_id: Class::Breaker,
    name: "Celestial Force Barrage",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FALLING_STAR: Skill = Skill {
    id: 47130,
    class_id: Class::Breaker,
    name: "Falling Star",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SMASHING_BLOW: Skill = Skill {
    id: 47140,
    class_id: Class::Breaker,
    name: "Smashing Blow",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CELESTIAL_DOOR: Skill = Skill {
    id: 47910,
    class_id: Class::Breaker,
    name: "Celestial Door",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HUNDRED_FISTS: Skill = Skill {
    id: 47160,
    class_id: Class::Breaker,
    name: "Hundred Fists",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EXPLOSIVE_FIST_47230: Skill = Skill {
    id: 47230,
    class_id: Class::Breaker,
    name: "Explosive Fist",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EARTHBREAKER: Skill = Skill {
    id: 47220,
    class_id: Class::Breaker,
    name: "Earthbreaker",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRAL_UPPERCUT: Skill = Skill {
    id: 47210,
    class_id: Class::Breaker,
    name: "Spiral Uppercut",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TURNING_SLASH: Skill = Skill {
    id: 25130,
    class_id: Class::Deathblade,
    name: "Turning Slash",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_TRANCE_25030: Skill = Skill {
    id: 25030,
    class_id: Class::Deathblade,
    name: "Death Trance",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BREAKING_MOON: Skill = Skill {
    id: 25430,
    class_id: Class::Deathblade,
    name: "Breaking Moon",
    cooldown: 180,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FALLSTAR: Skill = Skill {
    id: 25170,
    class_id: Class::Deathblade,
    name: "Fallstar",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLASH_BLINK: Skill = Skill {
    id: 25300,
    class_id: Class::Deathblade,
    name: "Flash Blink",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_SENTENCE: Skill = Skill {
    id: 25080,
    class_id: Class::Deathblade,
    name: "Death Sentence",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FATAL_WAVE: Skill = Skill {
    id: 25190,
    class_id: Class::Deathblade,
    name: "Fatal Wave",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EARTH_CLEAVER: Skill = Skill {
    id: 25070,
    class_id: Class::Deathblade,
    name: "Earth Cleaver",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPINCUTTER: Skill = Skill {
    id: 25120,
    class_id: Class::Deathblade,
    name: "Spincutter",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WIND_CUT: Skill = Skill {
    id: 25050,
    class_id: Class::Deathblade,
    name: "Wind Cut",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOUL_ABSORBER: Skill = Skill {
    id: 25110,
    class_id: Class::Deathblade,
    name: "Soul Absorber",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEVEL_3: Skill = Skill {
    id: 25037,
    class_id: Class::Deathblade,
    name: "Level 3",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SURPRISE_ATTACK: Skill = Skill {
    id: 25040,
    class_id: Class::Deathblade,
    name: "Surprise Attack",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ZERO: Skill = Skill {
    id: 25038,
    class_id: Class::Deathblade,
    name: "Zero",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MAELSTROM_25160: Skill = Skill {
    id: 25160,
    class_id: Class::Deathblade,
    name: "Maelstrom",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLADE_ASSAULT: Skill = Skill {
    id: 25350,
    class_id: Class::Deathblade,
    name: "Blade Assault",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_TRANCE_25032: Skill = Skill {
    id: 25032,
    class_id: Class::Deathblade,
    name: "Death Trance",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HIGH_SPEED_MOVE: Skill = Skill {
    id: 25020,
    class_id: Class::Deathblade,
    name: "High-Speed Move",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ETERNAL_FLASH: Skill = Skill {
    id: 25410,
    class_id: Class::Deathblade,
    name: "Eternal Flash",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOONLIGHT_SONIC: Skill = Skill {
    id: 25140,
    class_id: Class::Deathblade,
    name: "Moonlight Sonic",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DARK_AXEL: Skill = Skill {
    id: 25150,
    class_id: Class::Deathblade,
    name: "Dark Axel",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HEAD_HUNT: Skill = Skill {
    id: 25210,
    class_id: Class::Deathblade,
    name: "Head Hunt",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHAOTIC_DEATHBLADE: Skill = Skill {
    id: 25420,
    class_id: Class::Deathblade,
    name: "Chaotic Deathblade",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const UPPER_SLASH: Skill = Skill {
    id: 25060,
    class_id: Class::Deathblade,
    name: "Upper Slash",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEVEL_2: Skill = Skill {
    id: 25036,
    class_id: Class::Deathblade,
    name: "Level 2",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const VOID_STRIKE: Skill = Skill {
    id: 25180,
    class_id: Class::Deathblade,
    name: "Void Strike",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_TRANCE_25031: Skill = Skill {
    id: 25031,
    class_id: Class::Deathblade,
    name: "Death Trance",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLITZ_RUSH: Skill = Skill {
    id: 25200,
    class_id: Class::Deathblade,
    name: "Blitz Rush",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEVEL_1: Skill = Skill {
    id: 25035,
    class_id: Class::Deathblade,
    name: "Level 1",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TWIN_SHADOWS: Skill = Skill {
    id: 25090,
    class_id: Class::Deathblade,
    name: "Twin Shadows",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATHLY_SLASH: Skill = Skill {
    id: 25440,
    class_id: Class::Deathblade,
    name: "Deathly Slash",
    cooldown: 60,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLADE_DANCE: Skill = Skill {
    id: 25100,
    class_id: Class::Deathblade,
    name: "Blade Dance",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHARPENED_CUT: Skill = Skill {
    id: 27250,
    class_id: Class::Shadowhunter,
    name: "Sharpened Cut",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DESTRUCTION: Skill = Skill {
    id: 27840,
    class_id: Class::Shadowhunter,
    name: "Destruction",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RUSH: Skill = Skill {
    id: 27020,
    class_id: Class::Shadowhunter,
    name: "Rush",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOOD_MARSH: Skill = Skill {
    id: 27940,
    class_id: Class::Shadowhunter,
    name: "Blood Marsh",
    cooldown: 60,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PIERCING_THORN: Skill = Skill {
    id: 27110,
    class_id: Class::Shadowhunter,
    name: "Piercing Thorn",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WILD_SLASH: Skill = Skill {
    id: 27930,
    class_id: Class::Shadowhunter,
    name: "Wild Slash",
    cooldown: 80,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NIMBLE_CUT: Skill = Skill {
    id: 27060,
    class_id: Class::Shadowhunter,
    name: "Nimble Cut",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEMONIC_CLONE: Skill = Skill {
    id: 27210,
    class_id: Class::Shadowhunter,
    name: "Demonic Clone",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEAPING_BLOW: Skill = Skill {
    id: 27850,
    class_id: Class::Shadowhunter,
    name: "Leaping Blow",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SLASHER: Skill = Skill {
    id: 27070,
    class_id: Class::Shadowhunter,
    name: "Slasher",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOODY_PIERCING: Skill = Skill {
    id: 27960,
    class_id: Class::Shadowhunter,
    name: "Bloody Piercing",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEMONIC_SLASH_27051: Skill = Skill {
    id: 27051,
    class_id: Class::Shadowhunter,
    name: "Demonic Slash",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BRUTAL_CROSS: Skill = Skill {
    id: 27080,
    class_id: Class::Shadowhunter,
    name: "Brutal Cross",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RIFT_WALK: Skill = Skill {
    id: 27801,
    class_id: Class::Shadowhunter,
    name: "Rift Walk",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FALLEN_RUIN: Skill = Skill {
    id: 27890,
    class_id: Class::Shadowhunter,
    name: "Fallen Ruin",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THRUST_IMPACT: Skill = Skill {
    id: 27200,
    class_id: Class::Shadowhunter,
    name: "Thrust Impact",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RUINING_RUSH: Skill = Skill {
    id: 27810,
    class_id: Class::Shadowhunter,
    name: "Ruining Rush",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEMONS_GRIP: Skill = Skill {
    id: 27090,
    class_id: Class::Shadowhunter,
    name: "Demon's Grip",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOWL: Skill = Skill {
    id: 27170,
    class_id: Class::Shadowhunter,
    name: "Howl",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GATE_OF_ERUPTION: Skill = Skill {
    id: 27035,
    class_id: Class::Shadowhunter,
    name: "Gate of Eruption",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEMOLITION: Skill = Skill {
    id: 27220,
    class_id: Class::Shadowhunter,
    name: "Demolition",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPINNING_WEAPON: Skill = Skill {
    id: 27230,
    class_id: Class::Shadowhunter,
    name: "Spinning Weapon",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_CLAW: Skill = Skill {
    id: 27820,
    class_id: Class::Shadowhunter,
    name: "Death Claw",
    cooldown: 4,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GRIND_CHAIN: Skill = Skill {
    id: 27130,
    class_id: Class::Shadowhunter,
    name: "Grind Chain",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DARKNESS_BLAST: Skill = Skill {
    id: 27910,
    class_id: Class::Shadowhunter,
    name: "Darkness Blast",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DECIMATE: Skill = Skill {
    id: 27240,
    class_id: Class::Shadowhunter,
    name: "Decimate",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRUEL_CUTTER: Skill = Skill {
    id: 27180,
    class_id: Class::Shadowhunter,
    name: "Cruel Cutter",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEMONIZE: Skill = Skill {
    id: 27030,
    class_id: Class::Shadowhunter,
    name: "Demonize",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPINNING_DIVE: Skill = Skill {
    id: 27140,
    class_id: Class::Shadowhunter,
    name: "Spinning Dive",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GORE_BLEEDING: Skill = Skill {
    id: 27830,
    class_id: Class::Shadowhunter,
    name: "Gore Bleeding",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CANCEL_DEMONIZATION: Skill = Skill {
    id: 27032,
    class_id: Class::Shadowhunter,
    name: "Cancel Demonization",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLOOD_MASSACRE: Skill = Skill {
    id: 27860,
    class_id: Class::Shadowhunter,
    name: "Blood Massacre",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEMON_VISION: Skill = Skill {
    id: 27120,
    class_id: Class::Shadowhunter,
    name: "Demon Vision",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STORM_GRINDING: Skill = Skill {
    id: 27950,
    class_id: Class::Shadowhunter,
    name: "Storm Grinding",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEMONIC_SLASH_27050: Skill = Skill {
    id: 27050,
    class_id: Class::Shadowhunter,
    name: "Demonic Slash",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAY_OF_RUIN: Skill = Skill {
    id: 27920,
    class_id: Class::Shadowhunter,
    name: "Ray of Ruin",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RISING_CLAW: Skill = Skill {
    id: 27100,
    class_id: Class::Shadowhunter,
    name: "Rising Claw",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LUNAR_ECLIPSE_CADENZA: Skill = Skill {
    id: 26900,
    class_id: Class::Reaper,
    name: "Lunar Eclipse: Cadenza",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REAPERS_CALL: Skill = Skill {
    id: 26080,
    class_id: Class::Reaper,
    name: "Reaper's Call",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CADENZA_DE_LA_LUNA: Skill = Skill {
    id: 26940,
    class_id: Class::Reaper,
    name: "Cadenza de la Luna",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CALL_OF_THE_KNIFE: Skill = Skill {
    id: 26550,
    class_id: Class::Reaper,
    name: "Call of the Knife",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOW_DOUBLE_26530: Skill = Skill {
    id: 26530,
    class_id: Class::Reaper,
    name: "Shadow Double",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REQUIEM_DEL_SOL: Skill = Skill {
    id: 26950,
    class_id: Class::Reaper,
    name: "Requiem del Sol",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FINISHING_STEP: Skill = Skill {
    id: 26970,
    class_id: Class::Reaper,
    name: "Finishing Step",
    cooldown: 70,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHANTOM_DANCER_26140: Skill = Skill {
    id: 26140,
    class_id: Class::Reaper,
    name: "Phantom Dancer",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DISTORTION_26511: Skill = Skill {
    id: 26511,
    class_id: Class::Reaper,
    name: "Distortion",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FATAL_STEP: Skill = Skill {
    id: 26980,
    class_id: Class::Reaper,
    name: "Fatal Step",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATHSCYTHE: Skill = Skill {
    id: 26120,
    class_id: Class::Reaper,
    name: "Deathscythe",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NIGHTMARE_26100: Skill = Skill {
    id: 26100,
    class_id: Class::Reaper,
    name: "Nightmare",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHANTOM_DANCER_26150: Skill = Skill {
    id: 26150,
    class_id: Class::Reaper,
    name: "Phantom Dancer",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOW_KNIFE: Skill = Skill {
    id: 26960,
    class_id: Class::Reaper,
    name: "Shadow Knife",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOW_DOUBLE_26610: Skill = Skill {
    id: 26610,
    class_id: Class::Reaper,
    name: "Shadow Double",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DANCE_OF_FURY: Skill = Skill {
    id: 26830,
    class_id: Class::Reaper,
    name: "Dance of Fury",
    cooldown: 28,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SILENT_RAGE: Skill = Skill {
    id: 26820,
    class_id: Class::Reaper,
    name: "Silent Rage",
    cooldown: 28,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DISTORTION_26510: Skill = Skill {
    id: 26510,
    class_id: Class::Reaper,
    name: "Distortion",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOW_TRAP: Skill = Skill {
    id: 26570,
    class_id: Class::Reaper,
    name: "Shadow Trap",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRIT_CATCH: Skill = Skill {
    id: 26060,
    class_id: Class::Reaper,
    name: "Spirit Catch",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOW_VORTEX: Skill = Skill {
    id: 26070,
    class_id: Class::Reaper,
    name: "Shadow Vortex",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAGE_SPEAR: Skill = Skill {
    id: 26810,
    class_id: Class::Reaper,
    name: "Rage Spear",
    cooldown: 25,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOW_STORM: Skill = Skill {
    id: 26600,
    class_id: Class::Reaper,
    name: "Shadow Storm",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOLAR_ECLIPSE_REQUIEM_26920: Skill = Skill {
    id: 26920,
    class_id: Class::Reaper,
    name: "Solar Eclipse: Requiem",
    cooldown: 300,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NIGHTMARE_26090: Skill = Skill {
    id: 26090,
    class_id: Class::Reaper,
    name: "Nightmare",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PERSONA: Skill = Skill {
    id: 26040,
    class_id: Class::Reaper,
    name: "Persona",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOLAR_ECLIPSE_REQUIEM_26910: Skill = Skill {
    id: 26910,
    class_id: Class::Reaper,
    name: "Solar Eclipse: Requiem",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOW_STEP: Skill = Skill {
    id: 26020,
    class_id: Class::Reaper,
    name: "Shadow Step",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLACK_MIST: Skill = Skill {
    id: 26560,
    class_id: Class::Reaper,
    name: "Black Mist",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOW_DOUBLE_26520: Skill = Skill {
    id: 26520,
    class_id: Class::Reaper,
    name: "Shadow Double",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const _26500: Skill = Skill {
    id: 26500,
    class_id: Class::Reaper,
    name: "",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPINNING_DAGGER: Skill = Skill {
    id: 26050,
    class_id: Class::Reaper,
    name: "Spinning Dagger",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PIERCING_BLADE: Skill = Skill {
    id: 26110,
    class_id: Class::Reaper,
    name: "Piercing Blade",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOLAR_ECLIPSE_REQUIEM_26930: Skill = Skill {
    id: 26930,
    class_id: Class::Reaper,
    name: "Solar Eclipse: Requiem",
    cooldown: 300,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GLOWING_BRAND: Skill = Skill {
    id: 26800,
    class_id: Class::Reaper,
    name: "Glowing Brand",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_YARD: Skill = Skill {
    id: 46450,
    class_id: Class::Souleater,
    name: "Death Yard",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GLUTTONY: Skill = Skill {
    id: 46400,
    class_id: Class::Souleater,
    name: "Gluttony",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SCYTHE_DASH_46020: Skill = Skill {
    id: 46020,
    class_id: Class::Souleater,
    name: "Scythe Dash",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THE_END: Skill = Skill {
    id: 46610,
    class_id: Class::Souleater,
    name: "The End",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_SLASH: Skill = Skill {
    id: 46510,
    class_id: Class::Souleater,
    name: "Death Slash",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FATAL_FINALE: Skill = Skill {
    id: 46640,
    class_id: Class::Souleater,
    name: "Fatal Finale",
    cooldown: 72,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEADLY_COMBINATION: Skill = Skill {
    id: 46620,
    class_id: Class::Souleater,
    name: "Deadly Combination",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const VESTIGE: Skill = Skill {
    id: 46530,
    class_id: Class::Souleater,
    name: "Vestige",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_ORDER: Skill = Skill {
    id: 46410,
    class_id: Class::Souleater,
    name: "Death Order",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPECTRAL_COFFIN: Skill = Skill {
    id: 46420,
    class_id: Class::Souleater,
    name: "Spectral Coffin",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RUSTED_NAIL: Skill = Skill {
    id: 46220,
    class_id: Class::Souleater,
    name: "Rusted Nail",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ASTAROS: Skill = Skill {
    id: 46430,
    class_id: Class::Souleater,
    name: "Astaros",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATHBRINGER: Skill = Skill {
    id: 46600,
    class_id: Class::Souleater,
    name: "Deathbringer",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HARVEST: Skill = Skill {
    id: 46200,
    class_id: Class::Souleater,
    name: "Harvest",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REAPERS_SCYTHE: Skill = Skill {
    id: 46500,
    class_id: Class::Souleater,
    name: "Reaper's Scythe",
    cooldown: 26,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LUNATIC_EDGE: Skill = Skill {
    id: 46210,
    class_id: Class::Souleater,
    name: "Lunatic Edge",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LETHAL_SPINNING: Skill = Skill {
    id: 46250,
    class_id: Class::Souleater,
    name: "Lethal Spinning",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FRENZY_SCYTHE: Skill = Skill {
    id: 46650,
    class_id: Class::Souleater,
    name: "Frenzy Scythe",
    cooldown: 80,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THRASHING_46235: Skill = Skill {
    id: 46235,
    class_id: Class::Souleater,
    name: "Thrashing",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GUILLOTINE_SWING: Skill = Skill {
    id: 46520,
    class_id: Class::Souleater,
    name: "Guillotine Swing",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOUL_DECAPITATION: Skill = Skill {
    id: 46660,
    class_id: Class::Souleater,
    name: "Soul Decapitation",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THRASHING_46230: Skill = Skill {
    id: 46230,
    class_id: Class::Souleater,
    name: "Thrashing",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SCYTHE_DASH_46021: Skill = Skill {
    id: 46021,
    class_id: Class::Souleater,
    name: "Scythe Dash",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATHLORD_MODE: Skill = Skill {
    id: 46050,
    class_id: Class::Souleater,
    name: "Deathlord Mode",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FATALITY: Skill = Skill {
    id: 46630,
    class_id: Class::Souleater,
    name: "Fatality",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GHOST_STEP: Skill = Skill {
    id: 46240,
    class_id: Class::Souleater,
    name: "Ghost Step",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOUL_DRAIN: Skill = Skill {
    id: 46440,
    class_id: Class::Souleater,
    name: "Soul Drain",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LOCK_ON_28291: Skill = Skill {
    id: 28291,
    class_id: Class::Sharpshooter,
    name: "Lock-On",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SILVERHAWK_BASIC_ATTACK: Skill = Skill {
    id: 28159,
    class_id: Class::Sharpshooter,
    name: "Silverhawk Basic Attack",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SMOKESCREEN_ARROW: Skill = Skill {
    id: 28060,
    class_id: Class::Sharpshooter,
    name: "Smokescreen Arrow",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHARGED_SHOT: Skill = Skill {
    id: 28090,
    class_id: Class::Sharpshooter,
    name: "Charged Shot",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SNIPE: Skill = Skill {
    id: 28220,
    class_id: Class::Sharpshooter,
    name: "Snipe",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SILVERHAWK_ASSAULT: Skill = Skill {
    id: 28300,
    class_id: Class::Sharpshooter,
    name: "Silverhawk Assault",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHADOW_ARROW: Skill = Skill {
    id: 28200,
    class_id: Class::Sharpshooter,
    name: "Shadow Arrow",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAPID_SHOT: Skill = Skill {
    id: 28020,
    class_id: Class::Sharpshooter,
    name: "Rapid Shot",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ATOMIC_ARROW: Skill = Skill {
    id: 28040,
    class_id: Class::Sharpshooter,
    name: "Atomic Arrow",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WINGS_OF_STORM_28160: Skill = Skill {
    id: 28160,
    class_id: Class::Sharpshooter,
    name: "Wings of Storm",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EVASIVE_FIRE: Skill = Skill {
    id: 28100,
    class_id: Class::Sharpshooter,
    name: "Evasive Fire",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CLAYMORE_MINE: Skill = Skill {
    id: 28190,
    class_id: Class::Sharpshooter,
    name: "Claymore Mine",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUMMON_SILVERHAWK_28157: Skill = Skill {
    id: 28157,
    class_id: Class::Sharpshooter,
    name: "Summon Silverhawk",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AAGA_DEADEYE: Skill = Skill {
    id: 28270,
    class_id: Class::Sharpshooter,
    name: "A.A.G.A.: Deadeye",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRAL_ARROW: Skill = Skill {
    id: 28280,
    class_id: Class::Sharpshooter,
    name: "Spiral Arrow",
    cooldown: 65,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUMMON_SILVERHAWK_28156: Skill = Skill {
    id: 28156,
    class_id: Class::Sharpshooter,
    name: "Summon Silverhawk",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ARROW_WAVE: Skill = Skill {
    id: 28070,
    class_id: Class::Sharpshooter,
    name: "Arrow Wave",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GOLDEN_EYE: Skill = Skill {
    id: 28240,
    class_id: Class::Sharpshooter,
    name: "Golden Eye",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLADE_STORM: Skill = Skill {
    id: 28150,
    class_id: Class::Sharpshooter,
    name: "Blade Storm",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WINGS_OF_STORM_28320: Skill = Skill {
    id: 28320,
    class_id: Class::Sharpshooter,
    name: "Wings of Storm",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ARROW_SHOWER: Skill = Skill {
    id: 28080,
    class_id: Class::Sharpshooter,
    name: "Arrow Shower",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOVING_SLASH: Skill = Skill {
    id: 28140,
    class_id: Class::Sharpshooter,
    name: "Moving Slash",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HAWK_SHOT: Skill = Skill {
    id: 28250,
    class_id: Class::Sharpshooter,
    name: "Hawk Shot",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FENRIRS_MESSENGER: Skill = Skill {
    id: 28230,
    class_id: Class::Sharpshooter,
    name: "Fenrir's Messenger",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SALVO: Skill = Skill {
    id: 28030,
    class_id: Class::Sharpshooter,
    name: "Salvo",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DM_FOUR2_28120: Skill = Skill {
    id: 28120,
    class_id: Class::Sharpshooter,
    name: "DM-42",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEADLY_SLASH: Skill = Skill {
    id: 28130,
    class_id: Class::Sharpshooter,
    name: "Deadly Slash",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SLIDE: Skill = Skill {
    id: 28015,
    class_id: Class::Sharpshooter,
    name: "Slide",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LAST_RUSH: Skill = Skill {
    id: 28170,
    class_id: Class::Sharpshooter,
    name: "Last Rush",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DM_FOUR2_28125: Skill = Skill {
    id: 28125,
    class_id: Class::Sharpshooter,
    name: "DM-42",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GIGANTAR_BOW_FENRIR: Skill = Skill {
    id: 28260,
    class_id: Class::Sharpshooter,
    name: "Gigantar Bow: Fenrir",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LOCK_ON_28290: Skill = Skill {
    id: 28290,
    class_id: Class::Sharpshooter,
    name: "Lock-On",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUMMON_SILVERHAWK_28158: Skill = Skill {
    id: 28158,
    class_id: Class::Sharpshooter,
    name: "Summon Silverhawk",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHARPSHOOTER_28110: Skill = Skill {
    id: 28110,
    class_id: Class::Sharpshooter,
    name: "Sharpshooter",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STALKER: Skill = Skill {
    id: 28210,
    class_id: Class::Sharpshooter,
    name: "Stalker",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CLAY_BOMBARDMENT: Skill = Skill {
    id: 29250,
    class_id: Class::Deadeye,
    name: "Clay Bombardment",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DESPERADO: Skill = Skill {
    id: 29290,
    class_id: Class::Deadeye,
    name: "Desperado",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEADLY_CAGE: Skill = Skill {
    id: 29360,
    class_id: Class::Deadeye,
    name: "Deadly Cage",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SECRET_WEAPON: Skill = Skill {
    id: 29380,
    class_id: Class::Deadeye,
    name: "Secret Weapon",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const METEOR_STREAM_29100: Skill = Skill {
    id: 29100,
    class_id: Class::Deadeye,
    name: "Meteor Stream",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEXTEROUS_SHOT_29200: Skill = Skill {
    id: 29200,
    class_id: Class::Deadeye,
    name: "Dexterous Shot",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PERFECT_SHOT_29270: Skill = Skill {
    id: 29270,
    class_id: Class::Deadeye,
    name: "Perfect Shot",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHOTGUN_DOMINATOR: Skill = Skill {
    id: 29110,
    class_id: Class::Deadeye,
    name: "Shotgun Dominator",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DOOM_SHOT: Skill = Skill {
    id: 29340,
    class_id: Class::Deadeye,
    name: "Doom Shot",
    cooldown: 70,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BLAUER_BLITZ: Skill = Skill {
    id: 29370,
    class_id: Class::Deadeye,
    name: "Blauer Blitz",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_FIRE_29090: Skill = Skill {
    id: 29090,
    class_id: Class::Deadeye,
    name: "Death Fire",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANGE_STANCE_29162: Skill = Skill {
    id: 29162,
    class_id: Class::Deadeye,
    name: "Change Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENFORCE_EXECUTION: Skill = Skill {
    id: 29020,
    class_id: Class::Deadeye,
    name: "Enforce Execution",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRAL_FLAME_29060: Skill = Skill {
    id: 29060,
    class_id: Class::Deadeye,
    name: "Spiral Flame",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRAL_TRACKER_29180: Skill = Skill {
    id: 29180,
    class_id: Class::Deadeye,
    name: "Spiral Tracker",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRUEL_TRACKER: Skill = Skill {
    id: 29190,
    class_id: Class::Deadeye,
    name: "Cruel Tracker",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANGE_STANCE_29163: Skill = Skill {
    id: 29163,
    class_id: Class::Deadeye,
    name: "Change Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const QUICK_FIRE: Skill = Skill {
    id: 29330,
    class_id: Class::Deadeye,
    name: "Quick Fire",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EQUILIBRIUM_29220: Skill = Skill {
    id: 29220,
    class_id: Class::Deadeye,
    name: "Equilibrium",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LAST_REQUEST_29230: Skill = Skill {
    id: 29230,
    class_id: Class::Deadeye,
    name: "Last Request",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BURSTING_FLARE: Skill = Skill {
    id: 29280,
    class_id: Class::Deadeye,
    name: "Bursting Flare",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AT02_GRENADE_29120: Skill = Skill {
    id: 29120,
    class_id: Class::Deadeye,
    name: "AT02 Grenade",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHOTGUN_RAPID_FIRE_29170: Skill = Skill {
    id: 29170,
    class_id: Class::Deadeye,
    name: "Shotgun Rapid Fire",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RELENTLESS_ASSAULT: Skill = Skill {
    id: 29320,
    class_id: Class::Deadeye,
    name: "Relentless Assault",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MARK_OF_DEATH: Skill = Skill {
    id: 29350,
    class_id: Class::Deadeye,
    name: "Mark of Death",
    cooldown: 55,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SIGN_OF_APOCALYPSE_29040: Skill = Skill {
    id: 29040,
    class_id: Class::Deadeye,
    name: "Sign of Apocalypse",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AIMED_SHOT: Skill = Skill {
    id: 29240,
    class_id: Class::Deadeye,
    name: "Aimed Shot",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANGE_STANCE_29161: Skill = Skill {
    id: 29161,
    class_id: Class::Deadeye,
    name: "Change Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SIGN_OF_APOCALYPSE_29041: Skill = Skill {
    id: 29041,
    class_id: Class::Deadeye,
    name: "Sign of Apocalypse",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JUDGMENT_DAY: Skill = Skill {
    id: 29300,
    class_id: Class::Deadeye,
    name: "Judgment Day",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANGE_STANCE_29160: Skill = Skill {
    id: 29160,
    class_id: Class::Deadeye,
    name: "Change Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ONE_SHOT_ONE_KILL: Skill = Skill {
    id: 29310,
    class_id: Class::Deadeye,
    name: "One Shot One Kill",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TUMBLING_29150: Skill = Skill {
    id: 29150,
    class_id: Class::Deadeye,
    name: "Tumbling",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOMERSAULT_SHOT_29210: Skill = Skill {
    id: 29210,
    class_id: Class::Deadeye,
    name: "Somersault Shot",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const QUICK_SHOT_29261: Skill = Skill {
    id: 29261,
    class_id: Class::Deadeye,
    name: "Quick Shot",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CATASTROPHE_29080: Skill = Skill {
    id: 29080,
    class_id: Class::Deadeye,
    name: "Catastrophe",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANGE_STANCE_29164: Skill = Skill {
    id: 29164,
    class_id: Class::Deadeye,
    name: "Change Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const QUICK_SHOT_29262: Skill = Skill {
    id: 29262,
    class_id: Class::Deadeye,
    name: "Quick Shot",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const QUICK_SHOT_29260: Skill = Skill {
    id: 29260,
    class_id: Class::Deadeye,
    name: "Quick Shot",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PLASMA_BULLET_29140: Skill = Skill {
    id: 29140,
    class_id: Class::Deadeye,
    name: "Plasma Bullet",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const JUMP_BARRAGE: Skill = Skill {
    id: 30150,
    class_id: Class::Artillerist,
    name: "Jump Barrage",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FORWARD_BARRAGE: Skill = Skill {
    id: 30190,
    class_id: Class::Artillerist,
    name: "Forward Barrage",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOMING_BARRAGE: Skill = Skill {
    id: 30220,
    class_id: Class::Artillerist,
    name: "Homing Barrage",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const KINDLE: Skill = Skill {
    id: 30380,
    class_id: Class::Artillerist,
    name: "Kindle",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BARRAGE_FOCUS_FIRE: Skill = Skill {
    id: 30260,
    class_id: Class::Artillerist,
    name: "Barrage: Focus Fire",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NAPALM_SHOT: Skill = Skill {
    id: 30180,
    class_id: Class::Artillerist,
    name: "Napalm Shot",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENERGY_FIELD: Skill = Skill {
    id: 30160,
    class_id: Class::Artillerist,
    name: "Energy Field",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MISSILE_BARRAGE: Skill = Skill {
    id: 30230,
    class_id: Class::Artillerist,
    name: "Missile Barrage",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GRAVITY_EXPLOSION: Skill = Skill {
    id: 30200,
    class_id: Class::Artillerist,
    name: "Gravity Explosion",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOWITZER: Skill = Skill {
    id: 30090,
    class_id: Class::Artillerist,
    name: "Howitzer",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUMMON_TURRET: Skill = Skill {
    id: 30100,
    class_id: Class::Artillerist,
    name: "Summon Turret",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BUCKSHOT: Skill = Skill {
    id: 30070,
    class_id: Class::Artillerist,
    name: "Buckshot",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWING_30140: Skill = Skill {
    id: 30140,
    class_id: Class::Artillerist,
    name: "Swing",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HEAVY_TURRET: Skill = Skill {
    id: 30280,
    class_id: Class::Artillerist,
    name: "Heavy Turret",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHOTOELECTRONIC_CANNON: Skill = Skill {
    id: 30285,
    class_id: Class::Artillerist,
    name: "Photoelectronic Cannon",
    cooldown: 300,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BARRAGE_STEEL_RAIN: Skill = Skill {
    id: 30340,
    class_id: Class::Artillerist,
    name: "Barrage: Steel Rain",
    cooldown: 70,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EXIT_BARRAGE_MODE: Skill = Skill {
    id: 30021,
    class_id: Class::Artillerist,
    name: "Exit Barrage Mode",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MULTI_ROCKET_LAUNCHER: Skill = Skill {
    id: 30080,
    class_id: Class::Artillerist,
    name: "Multi-Rocket Launcher",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BARRAGE_HOWITZER: Skill = Skill {
    id: 30250,
    class_id: Class::Artillerist,
    name: "Barrage: Howitzer",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BARRAGE_ENERGY_CANNON: Skill = Skill {
    id: 30270,
    class_id: Class::Artillerist,
    name: "Barrage: Energy Cannon",
    cooldown: 40,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BARRAGE_ATTACK: Skill = Skill {
    id: 30370,
    class_id: Class::Artillerist,
    name: "Barrage Attack",
    cooldown: 60,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BARRAGE_BASIC_ATTACK: Skill = Skill {
    id: 30240,
    class_id: Class::Artillerist,
    name: "Barrage: Basic Attack",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AIR_RAID: Skill = Skill {
    id: 30110,
    class_id: Class::Artillerist,
    name: "Air Raid",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ROLL_30015: Skill = Skill {
    id: 30015,
    class_id: Class::Artillerist,
    name: "Roll",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENTER_BARRAGE_MODE: Skill = Skill {
    id: 30020,
    class_id: Class::Artillerist,
    name: "Enter Barrage Mode",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ACOM_ATTACK: Skill = Skill {
    id: 30330,
    class_id: Class::Artillerist,
    name: "A.C.O.M. Attack",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLAMETHROWER: Skill = Skill {
    id: 30120,
    class_id: Class::Artillerist,
    name: "Flamethrower",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BARRAGE_IMPREGNABILITY: Skill = Skill {
    id: 30290,
    class_id: Class::Artillerist,
    name: "Barrage: Impregnability",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ACOM_BOMBARDMENT_SUPPORT: Skill = Skill {
    id: 30320,
    class_id: Class::Artillerist,
    name: "A.C.O.M. Bombardment Support",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENHANCED_SHELL: Skill = Skill {
    id: 30050,
    class_id: Class::Artillerist,
    name: "Enhanced Shell",
    cooldown: 5,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BARRAGE_ACT: Skill = Skill {
    id: 30392,
    class_id: Class::Artillerist,
    name: "Barrage: A.C.T",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PLASMA_STORM: Skill = Skill {
    id: 30210,
    class_id: Class::Artillerist,
    name: "Plasma Storm",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GATLING_GUN: Skill = Skill {
    id: 30170,
    class_id: Class::Artillerist,
    name: "Gatling Gun",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MISSILE_LAUNCHER: Skill = Skill {
    id: 30350,
    class_id: Class::Artillerist,
    name: "Missile Launcher",
    cooldown: 70,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PRESSURIZED_HEATBOMB: Skill = Skill {
    id: 30060,
    class_id: Class::Artillerist,
    name: "Pressurized Heatbomb",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COMMAND_BABY_DRONES: Skill = Skill {
    id: 35100,
    class_id: Class::Machinist,
    name: "Command: Baby Drones",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THRUSTER_MOVE: Skill = Skill {
    id: 35730,
    class_id: Class::Machinist,
    name: "Thruster Move",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COMMAND_RAID_MISSILE: Skill = Skill {
    id: 35110,
    class_id: Class::Machinist,
    name: "Command: Raid Missile",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const POINT_EXCLUSION: Skill = Skill {
    id: 35840,
    class_id: Class::Machinist,
    name: "Point Exclusion",
    cooldown: 90,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FIERY_ESCAPE: Skill = Skill {
    id: 35120,
    class_id: Class::Machinist,
    name: "Fiery Escape",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ANNIHILATION_MODE: Skill = Skill {
    id: 35130,
    class_id: Class::Machinist,
    name: "Annihilation Mode",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AIR_STRIKE: Skill = Skill {
    id: 35930,
    class_id: Class::Machinist,
    name: "Air Strike",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COMMAND_BLOCKADE: Skill = Skill {
    id: 35070,
    class_id: Class::Machinist,
    name: "Command: Blockade",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COMET_STRIKE: Skill = Skill {
    id: 35750,
    class_id: Class::Machinist,
    name: "Comet Strike",
    cooldown: 4,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ECHELON_BEAM: Skill = Skill {
    id: 35780,
    class_id: Class::Machinist,
    name: "Echelon Beam",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ENERGY_BUSTER: Skill = Skill {
    id: 35210,
    class_id: Class::Machinist,
    name: "Energy Buster",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SLUGSHOT: Skill = Skill {
    id: 35760,
    class_id: Class::Machinist,
    name: "Slugshot",
    cooldown: 3,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PROJECT_TITAN: Skill = Skill {
    id: 35890,
    class_id: Class::Machinist,
    name: "Project Titan",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const OVERCHARGED_BATTERY: Skill = Skill {
    id: 35050,
    class_id: Class::Machinist,
    name: "Overcharged Battery",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOBILE_SHOT_35181: Skill = Skill {
    id: 35181,
    class_id: Class::Machinist,
    name: "Mobile Shot",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DISABLE_HYPERSYNC: Skill = Skill {
    id: 35701,
    class_id: Class::Machinist,
    name: "Disable Hypersync",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BACKFLIP_STRIKE: Skill = Skill {
    id: 35150,
    class_id: Class::Machinist,
    name: "Backflip Strike",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOBILE_SHOT_35182: Skill = Skill {
    id: 35182,
    class_id: Class::Machinist,
    name: "Mobile Shot",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HYPERSYNC: Skill = Skill {
    id: 35700,
    class_id: Class::Machinist,
    name: "Hypersync",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BATTLESHIP_OPERATION: Skill = Skill {
    id: 35810,
    class_id: Class::Machinist,
    name: "Battleship Operation",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SYNC_PROTOCOL: Skill = Skill {
    id: 35880,
    class_id: Class::Machinist,
    name: "Sync Protocol",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COMMAND_CALL_BACK: Skill = Skill {
    id: 35711,
    class_id: Class::Machinist,
    name: "Command: Call Back",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LASER_BLADE: Skill = Skill {
    id: 35770,
    class_id: Class::Machinist,
    name: "Laser Blade",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PULSE_FIRE: Skill = Skill {
    id: 35200,
    class_id: Class::Machinist,
    name: "Pulse Fire",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TREMORS: Skill = Skill {
    id: 35020,
    class_id: Class::Machinist,
    name: "Tremors",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COMMAND_CARPET: Skill = Skill {
    id: 35060,
    class_id: Class::Machinist,
    name: "Command: Carpet",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COMMAND_ACTIVE_PULSE: Skill = Skill {
    id: 35090,
    class_id: Class::Machinist,
    name: "Command: Active Pulse",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STRATEGIC_FIRE: Skill = Skill {
    id: 35170,
    class_id: Class::Machinist,
    name: "Strategic Fire",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COMMAND_FLARE_BEAM: Skill = Skill {
    id: 35080,
    class_id: Class::Machinist,
    name: "Command: Flare Beam",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NEO_FIRE: Skill = Skill {
    id: 35870,
    class_id: Class::Machinist,
    name: "Neo Fire",
    cooldown: 65,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SURGE_BLOW: Skill = Skill {
    id: 35790,
    class_id: Class::Machinist,
    name: "Surge Blow",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BULLET_HAIL: Skill = Skill {
    id: 35040,
    class_id: Class::Machinist,
    name: "Bullet Hail",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HIGH_VOLTAGE_BULLET: Skill = Skill {
    id: 35160,
    class_id: Class::Machinist,
    name: "High Voltage Bullet",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const COMMAND_M1FOUR3_MACHINE_GUN: Skill = Skill {
    id: 35140,
    class_id: Class::Machinist,
    name: "Command: M143 Machine Gun",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AVALANCHE: Skill = Skill {
    id: 35190,
    class_id: Class::Machinist,
    name: "Avalanche",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CRIMSON_BREAKER: Skill = Skill {
    id: 35800,
    class_id: Class::Machinist,
    name: "Crimson Breaker",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EX_ZERO_POINT: Skill = Skill {
    id: 35850,
    class_id: Class::Machinist,
    name: "EX - Zero Point",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOBILE_SHOT_35180: Skill = Skill {
    id: 35180,
    class_id: Class::Machinist,
    name: "Mobile Shot",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TARGET_DOWN: Skill = Skill {
    id: 38070,
    class_id: Class::Gunslinger,
    name: "Target Down",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HIGH_CALIBER_HE_BULLET: Skill = Skill {
    id: 38280,
    class_id: Class::Gunslinger,
    name: "High-Caliber HE Bullet",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRAL_TRACKER_38180: Skill = Skill {
    id: 38180,
    class_id: Class::Gunslinger,
    name: "Spiral Tracker",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEXTEROUS_SHOT_38200: Skill = Skill {
    id: 38200,
    class_id: Class::Gunslinger,
    name: "Dexterous Shot",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DUAL_BUCKSHOT: Skill = Skill {
    id: 38040,
    class_id: Class::Gunslinger,
    name: "Dual Buckshot",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOCUSED_SHOT: Skill = Skill {
    id: 38240,
    class_id: Class::Gunslinger,
    name: "Focused Shot",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BULLET_RAIN: Skill = Skill {
    id: 38190,
    class_id: Class::Gunslinger,
    name: "Bullet Rain",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPIRAL_FLAME_38060: Skill = Skill {
    id: 38060,
    class_id: Class::Gunslinger,
    name: "Spiral Flame",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOUR_OF_JUDGMENT: Skill = Skill {
    id: 38050,
    class_id: Class::Gunslinger,
    name: "Hour of Judgment",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AT02_GRENADE_38120: Skill = Skill {
    id: 38120,
    class_id: Class::Gunslinger,
    name: "AT02 Grenade",
    cooldown: 6,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANGE_STANCE_38162: Skill = Skill {
    id: 38162,
    class_id: Class::Gunslinger,
    name: "Change Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PEACEKEEPER: Skill = Skill {
    id: 38260,
    class_id: Class::Gunslinger,
    name: "Peacekeeper",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EYE_OF_TWILIGHT: Skill = Skill {
    id: 38250,
    class_id: Class::Gunslinger,
    name: "Eye of Twilight",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SOMERSAULT_SHOT_38210: Skill = Skill {
    id: 38210,
    class_id: Class::Gunslinger,
    name: "Somersault Shot",
    cooldown: 9,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REVERSE_STANCE_SWITCH: Skill = Skill {
    id: 38161,
    class_id: Class::Gunslinger,
    name: "Reverse Stance Switch",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STAMPEDE_38150: Skill = Skill {
    id: 38150,
    class_id: Class::Gunslinger,
    name: "Stampede",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PLASMA_BULLET_38140: Skill = Skill {
    id: 38140,
    class_id: Class::Gunslinger,
    name: "Plasma Bullet",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LAST_REQUEST_38230: Skill = Skill {
    id: 38230,
    class_id: Class::Gunslinger,
    name: "Last Request",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ATOMIC_EXPLOSION: Skill = Skill {
    id: 38330,
    class_id: Class::Gunslinger,
    name: "Atomic Explosion",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANGE_STANCE_38163: Skill = Skill {
    id: 38163,
    class_id: Class::Gunslinger,
    name: "Change Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEATH_FIRE_38090: Skill = Skill {
    id: 38090,
    class_id: Class::Gunslinger,
    name: "Death Fire",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHARPSHOOTER_38110: Skill = Skill {
    id: 38110,
    class_id: Class::Gunslinger,
    name: "Sharpshooter",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CATASTROPHE_38080: Skill = Skill {
    id: 38080,
    class_id: Class::Gunslinger,
    name: "Catastrophe",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANGE_STANCE_38164: Skill = Skill {
    id: 38164,
    class_id: Class::Gunslinger,
    name: "Change Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EQUILIBRIUM_38220: Skill = Skill {
    id: 38220,
    class_id: Class::Gunslinger,
    name: "Equilibrium",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const QUICK_STEP: Skill = Skill {
    id: 38020,
    class_id: Class::Gunslinger,
    name: "Quick Step",
    cooldown: 10,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BULLET_PRISON: Skill = Skill {
    id: 38290,
    class_id: Class::Gunslinger,
    name: "Bullet Prison",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const METEOR_STREAM_38100: Skill = Skill {
    id: 38100,
    class_id: Class::Gunslinger,
    name: "Meteor Stream",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BULLSEYE: Skill = Skill {
    id: 38310,
    class_id: Class::Gunslinger,
    name: "Bullseye",
    cooldown: 70,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ROSE_BLOSSOM: Skill = Skill {
    id: 38340,
    class_id: Class::Gunslinger,
    name: "Rose Blossom",
    cooldown: 60,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PERFECT_SHOT_38270: Skill = Skill {
    id: 38270,
    class_id: Class::Gunslinger,
    name: "Perfect Shot",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DEAD_END: Skill = Skill {
    id: 38320,
    class_id: Class::Gunslinger,
    name: "Dead End",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SHOTGUN_RAPID_FIRE_38170: Skill = Skill {
    id: 38170,
    class_id: Class::Gunslinger,
    name: "Shotgun Rapid Fire",
    cooldown: 36,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHANGE_STANCE_38160: Skill = Skill {
    id: 38160,
    class_id: Class::Gunslinger,
    name: "Change Stance",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SEVEN_SHOTGUNS: Skill = Skill {
    id: 38300,
    class_id: Class::Gunslinger,
    name: "Seven Shotguns",
    cooldown: 60,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_SPILL_15020: Skill = Skill {
    id: 15020,
    class_id: Class::Specialist,
    name: "Stroke: Spill",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_HERE_AND_THERE_15030: Skill = Skill {
    id: 15030,
    class_id: Class::Specialist,
    name: "Stroke: Here and There",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_HERE_AND_THERE_15040: Skill = Skill {
    id: 15040,
    class_id: Class::Specialist,
    name: "Stroke: Here and There",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_HOPPER: Skill = Skill {
    id: 31210,
    class_id: Class::Artist,
    name: "Stroke: Hopper",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUNRISE_31141: Skill = Skill {
    id: 31141,
    class_id: Class::Artist,
    name: "Sunrise",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOONFALL_31050: Skill = Skill {
    id: 31050,
    class_id: Class::Artist,
    name: "Moonfall",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_DRAGON_ENGRAVING: Skill = Skill {
    id: 31950,
    class_id: Class::Artist,
    name: "Paint: Dragon Engraving",
    cooldown: 72,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MASTERWORK_EFFLORESCENCE: Skill = Skill {
    id: 31910,
    class_id: Class::Artist,
    name: "Masterwork: Efflorescence",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_MOONSKETCH: Skill = Skill {
    id: 31500,
    class_id: Class::Artist,
    name: "Paint: Moonsketch",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUNRISE_31140: Skill = Skill {
    id: 31140,
    class_id: Class::Artist,
    name: "Sunrise",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_INK_SHOWER: Skill = Skill {
    id: 31200,
    class_id: Class::Artist,
    name: "Stroke: Ink Shower",
    cooldown: 8,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_SPRINKLE: Skill = Skill {
    id: 31430,
    class_id: Class::Artist,
    name: "Stroke: Sprinkle",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUNRISE_31131: Skill = Skill {
    id: 31131,
    class_id: Class::Artist,
    name: "Sunrise",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_DRAWING_ORCHIDS: Skill = Skill {
    id: 31420,
    class_id: Class::Artist,
    name: "Paint: Drawing Orchids",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_HERE_AND_THERE_31030: Skill = Skill {
    id: 31030,
    class_id: Class::Artist,
    name: "Stroke: Here and There",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RISING_MOON: Skill = Skill {
    id: 31145,
    class_id: Class::Artist,
    name: "Rising Moon",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_STARRY_NIGHT: Skill = Skill {
    id: 31450,
    class_id: Class::Artist,
    name: "Paint: Starry Night",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUNRISE_31130: Skill = Skill {
    id: 31130,
    class_id: Class::Artist,
    name: "Sunrise",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_INK_WELL: Skill = Skill {
    id: 31230,
    class_id: Class::Artist,
    name: "Paint: Ink Well",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_POUNCING_TIGER: Skill = Skill {
    id: 31490,
    class_id: Class::Artist,
    name: "Paint: Pouncing Tiger",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_UPWARD_STROKE: Skill = Skill {
    id: 31510,
    class_id: Class::Artist,
    name: "Stroke: Upward Stroke",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MASTERWORK_SPECTACLE: Skill = Skill {
    id: 31900,
    class_id: Class::Artist,
    name: "Masterwork: Spectacle",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const MOONFALL_31051: Skill = Skill {
    id: 31051,
    class_id: Class::Artist,
    name: "Moonfall",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_SUNSKETCH: Skill = Skill {
    id: 31400,
    class_id: Class::Artist,
    name: "Paint: Sunsketch",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: Some(SUNSKETCH_314010),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const HOLY_BEAST_SUMMON_PHOENIX: Skill = Skill {
    id: 31920,
    class_id: Class::Artist,
    name: "Holy Beast Summon: Phoenix",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUNRISE_31120: Skill = Skill {
    id: 31120,
    class_id: Class::Artist,
    name: "Sunrise",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_SUN_WELL: Skill = Skill {
    id: 31410,
    class_id: Class::Artist,
    name: "Paint: Sun Well",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: Some(SUN_WELL_314184),
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_SHATTERING_STRIKE: Skill = Skill {
    id: 31060,
    class_id: Class::Artist,
    name: "Paint: Shattering Strike",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUNRISE_31121: Skill = Skill {
    id: 31121,
    class_id: Class::Artist,
    name: "Sunrise",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_SPILL_31020: Skill = Skill {
    id: 31020,
    class_id: Class::Artist,
    name: "Stroke: Spill",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_ILLUSION_DOOR: Skill = Skill {
    id: 31220,
    class_id: Class::Artist,
    name: "Paint: Illusion Door",
    cooldown: 45,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUNRISE_31110: Skill = Skill {
    id: 31110,
    class_id: Class::Artist,
    name: "Sunrise",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_CRANE_WING: Skill = Skill {
    id: 31480,
    class_id: Class::Artist,
    name: "Paint: Crane Wing",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_CATTLE_DRIVE: Skill = Skill {
    id: 31940,
    class_id: Class::Artist,
    name: "Paint: Cattle Drive",
    cooldown: 45,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_BUTTERFLY_DREAM: Skill = Skill {
    id: 31460,
    class_id: Class::Artist,
    name: "Paint: Butterfly Dream",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_ONE_STROKE: Skill = Skill {
    id: 31470,
    class_id: Class::Artist,
    name: "Stroke: One Stroke",
    cooldown: 25,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PAINT_INKRISE: Skill = Skill {
    id: 31440,
    class_id: Class::Artist,
    name: "Paint: Inkrise",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STROKE_HERE_AND_THERE_31040: Skill = Skill {
    id: 31040,
    class_id: Class::Artist,
    name: "Stroke: Here and There",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DREAM_BLOSSOM_GARDEN: Skill = Skill {
    id: 31930,
    class_id: Class::Artist,
    name: "Dream Blossom Garden",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WIND_GIMLET: Skill = Skill {
    id: 32250,
    class_id: Class::Aeromancer,
    name: "Wind Gimlet",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPRING_BREEZE: Skill = Skill {
    id: 32220,
    class_id: Class::Aeromancer,
    name: "Spring Breeze",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PIERCING_WIND: Skill = Skill {
    id: 32260,
    class_id: Class::Aeromancer,
    name: "Piercing Wind",
    cooldown: 27,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THICK_FOG: Skill = Skill {
    id: 32140,
    class_id: Class::Aeromancer,
    name: "Thick Fog",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const LEAP_32030: Skill = Skill {
    id: 32030,
    class_id: Class::Aeromancer,
    name: "Leap!",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WHIRLPOOL: Skill = Skill {
    id: 32190,
    class_id: Class::Aeromancer,
    name: "Whirlpool",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPRINTING: Skill = Skill {
    id: 32020,
    class_id: Class::Aeromancer,
    name: "Sprinting",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const KAHNS_TERRITORY: Skill = Skill {
    id: 32300,
    class_id: Class::Aeromancer,
    name: "Kahn's Territory",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUN_SHOWER_32040: Skill = Skill {
    id: 32040,
    class_id: Class::Aeromancer,
    name: "Sun Shower",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STRONG_WIND: Skill = Skill {
    id: 32160,
    class_id: Class::Aeromancer,
    name: "Strong Wind",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TORNADO: Skill = Skill {
    id: 32170,
    class_id: Class::Aeromancer,
    name: "Tornado",
    cooldown: 14,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FACE_TO_FACE: Skill = Skill {
    id: 32230,
    class_id: Class::Aeromancer,
    name: "Face to Face",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUMMER_SUNLIGHT: Skill = Skill {
    id: 32320,
    class_id: Class::Aeromancer,
    name: "Summer Sunlight",
    cooldown: 70,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SUN_SHOWER_32041: Skill = Skill {
    id: 32041,
    class_id: Class::Aeromancer,
    name: "Sun Shower",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAGE: Skill = Skill {
    id: 32120,
    class_id: Class::Aeromancer,
    name: "Rage",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DAZZLING_DAYS: Skill = Skill {
    id: 32340,
    class_id: Class::Aeromancer,
    name: "Dazzling Days",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TORNADO_DANCE: Skill = Skill {
    id: 32180,
    class_id: Class::Aeromancer,
    name: "Tornado Dance",
    cooldown: 20,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const NARRS_BLADE: Skill = Skill {
    id: 32280,
    class_id: Class::Aeromancer,
    name: "Narr's Blade",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPREAD: Skill = Skill {
    id: 32110,
    class_id: Class::Aeromancer,
    name: "Spread",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SPACE_CLEAVE: Skill = Skill {
    id: 32330,
    class_id: Class::Aeromancer,
    name: "Space Cleave",
    cooldown: 22,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DOWNWARD_STRIKE_32130: Skill = Skill {
    id: 32130,
    class_id: Class::Aeromancer,
    name: "Downward Strike",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RAINSTORM: Skill = Skill {
    id: 32150,
    class_id: Class::Aeromancer,
    name: "Rainstorm",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const STORMS_APPROACH: Skill = Skill {
    id: 32270,
    class_id: Class::Aeromancer,
    name: "Storm's Approach",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WIPING_WIND: Skill = Skill {
    id: 32210,
    class_id: Class::Aeromancer,
    name: "Wiping Wind",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THUNDERWIND: Skill = Skill {
    id: 32310,
    class_id: Class::Aeromancer,
    name: "Thunderwind",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLY: Skill = Skill {
    id: 32200,
    class_id: Class::Aeromancer,
    name: "Fly",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SCORCHING_SUN: Skill = Skill {
    id: 32240,
    class_id: Class::Aeromancer,
    name: "Scorching Sun",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const AKASHAS_WAVE: Skill = Skill {
    id: 32290,
    class_id: Class::Aeromancer,
    name: "Akasha's Wave",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLOAT: Skill = Skill {
    id: 32021,
    class_id: Class::Aeromancer,
    name: "Float",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DIGGER_BEAR: Skill = Skill {
    id: 33210,
    class_id: Class::Wildsoul,
    name: "Digger Bear",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const THUMP_AND_THUD: Skill = Skill {
    id: 33021,
    class_id: Class::Wildsoul,
    name: "Thump and Thud",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOX_ORB: Skill = Skill {
    id: 33300,
    class_id: Class::Wildsoul,
    name: "Fox Orb",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BEAR_FORM_BASIC_ATTACK: Skill = Skill {
    id: 33001,
    class_id: Class::Wildsoul,
    name: "Bear Form Basic Attack",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWISH_BEAR_33201: Skill = Skill {
    id: 33201,
    class_id: Class::Wildsoul,
    name: "Swish Bear",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOX_FLAME_33321: Skill = Skill {
    id: 33321,
    class_id: Class::Wildsoul,
    name: "Fox Flame",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BOING: Skill = Skill {
    id: 33032,
    class_id: Class::Wildsoul,
    name: "Boing!",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REMOVE_SHAPESHIFT_33041: Skill = Skill {
    id: 33041,
    class_id: Class::Wildsoul,
    name: "Remove Shapeshift",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CROW_PARADE: Skill = Skill {
    id: 33120,
    class_id: Class::Wildsoul,
    name: "Crow Parade",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOX_FLAME_33320: Skill = Skill {
    id: 33320,
    class_id: Class::Wildsoul,
    name: "Fox Flame",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOX_LEAP_33331: Skill = Skill {
    id: 33331,
    class_id: Class::Wildsoul,
    name: "Fox Leap",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CROW_COMMOTION: Skill = Skill {
    id: 33150,
    class_id: Class::Wildsoul,
    name: "Crow Commotion",
    cooldown: 18,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BOULDER_BEAR_33231: Skill = Skill {
    id: 33231,
    class_id: Class::Wildsoul,
    name: "Boulder Bear",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ZOOM_33020: Skill = Skill {
    id: 33020,
    class_id: Class::Wildsoul,
    name: "Zoom!",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FORBIDDEN_SORCERY_RIPPING_BEAR: Skill = Skill {
    id: 33400,
    class_id: Class::Wildsoul,
    name: "Forbidden Sorcery: Ripping Bear",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TAILWIND: Skill = Skill {
    id: 33130,
    class_id: Class::Wildsoul,
    name: "Tailwind",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const VOILA: Skill = Skill {
    id: 33022,
    class_id: Class::Wildsoul,
    name: "Voila!",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SMACK_SMITE: Skill = Skill {
    id: 33520,
    class_id: Class::Wildsoul,
    name: "Smack Smite",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ONE_HIT_BEAR: Skill = Skill {
    id: 33450,
    class_id: Class::Wildsoul,
    name: "One-Hit Bear",
    cooldown: 50,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const ROLLING_WHEEL: Skill = Skill {
    id: 33160,
    class_id: Class::Wildsoul,
    name: "Rolling Wheel",
    cooldown: 15,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHANTOM_BEAST_AWAKENING_33050: Skill = Skill {
    id: 33050,
    class_id: Class::Wildsoul,
    name: "Phantom Beast Awakening",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const WHOOSH_33030: Skill = Skill {
    id: 33030,
    class_id: Class::Wildsoul,
    name: "Whoosh",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const TUMBLE: Skill = Skill {
    id: 33031,
    class_id: Class::Wildsoul,
    name: "Tumble",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const BOULDER_BEAR_33230: Skill = Skill {
    id: 33230,
    class_id: Class::Wildsoul,
    name: "Boulder Bear",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const REMOVE_SHAPESHIFT_33040: Skill = Skill {
    id: 33040,
    class_id: Class::Wildsoul,
    name: "Remove Shapeshift",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOX_FORM_BASIC_ATTACK: Skill = Skill {
    id: 33002,
    class_id: Class::Wildsoul,
    name: "Fox Form Basic Attack",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FLOATING_FOXBEAR: Skill = Skill {
    id: 33460,
    class_id: Class::Wildsoul,
    name: "Floating Foxbear",
    cooldown: 75,
    kind: SkillType::HyperAwakeningTechnique,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const URSINE_WINDUP: Skill = Skill {
    id: 33170,
    class_id: Class::Wildsoul,
    name: "Ursine Windup",
    cooldown: 30,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const GROWLING_BEAR: Skill = Skill {
    id: 33220,
    class_id: Class::Wildsoul,
    name: "Growling Bear",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOX_ILLUSION_33310: Skill = Skill {
    id: 33310,
    class_id: Class::Wildsoul,
    name: "Fox Illusion",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const VULPINE_VELOCITY: Skill = Skill {
    id: 33100,
    class_id: Class::Wildsoul,
    name: "Vulpine Velocity",
    cooldown: 7,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const EARTHQUAKE_POUND: Skill = Skill {
    id: 33500,
    class_id: Class::Wildsoul,
    name: "Earthquake Pound",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOX_FIRE_DANCE: Skill = Skill {
    id: 33530,
    class_id: Class::Wildsoul,
    name: "Fox Fire Dance",
    cooldown: 300,
    kind: SkillType::HyperAwakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const RUTHLESS_PECKING: Skill = Skill {
    id: 33140,
    class_id: Class::Wildsoul,
    name: "Ruthless Pecking",
    cooldown: 16,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FORBIDDEN_SORCERY_FOX_STAR_RAINSTORM: Skill = Skill {
    id: 33410,
    class_id: Class::Wildsoul,
    name: "Forbidden Sorcery: Fox Star Rainstorm",
    cooldown: 2,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const PHANTOM_BEAST_AWAKENING_33060: Skill = Skill {
    id: 33060,
    class_id: Class::Wildsoul,
    name: "Phantom Beast Awakening",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CLAW: Skill = Skill {
    id: 33110,
    class_id: Class::Wildsoul,
    name: "Claw",
    cooldown: 12,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOX_LEAP_33330: Skill = Skill {
    id: 33330,
    class_id: Class::Wildsoul,
    name: "Fox Leap",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const FOX_ILLUSION_33311: Skill = Skill {
    id: 33311,
    class_id: Class::Wildsoul,
    name: "Fox Illusion",
    cooldown: 24,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const CHAOS_BRAWL: Skill = Skill {
    id: 33510,
    class_id: Class::Wildsoul,
    name: "Chaos Brawl",
    cooldown: 300,
    kind: SkillType::Awakening,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const SWISH_BEAR_33200: Skill = Skill {
    id: 33200,
    class_id: Class::Wildsoul,
    name: "Swish Bear",
    cooldown: 1,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};

pub const DART: Skill = Skill {
    id: 33023,
    class_id: Class::Wildsoul,
    name: "Dart",
    cooldown: 0,
    kind: SkillType::Normal,
    skill_buff: None,
    is_counter: false,
	is_projectile: false,
    is_trap: false,
    summon: None
};


pub const ALL_CLASS_SKILLS: &'static [Skill] = &[
	TUMBLING_10060,
	SHOULDER_CHARGE_16061,
	CHAIN_OF_VENGEANCE,
	HELL_BLADE_TEMP_16181,
	DIVING_SLASH,
	FINISH_STRIKE,
	HELL_BLADE_TEMP_16180,
	BLOOD_SLASH,
	BLOODY_RUSH_16145,
	MAELSTROM_16620,
	BRAVE_SLASH,
	RAGE_DEATHBLADE,
	WIND_BLADE,
	THEATRIC_COMBAT_SWITCH,
	CRIME_HAZARD,
	BLOODY_RUSH_16147,
	CHARGE_STRIKE,
	RED_DUST,
	HELL_BLADE_TEMP_16170,
	DARK_RUSH,
	CHAIN_SWORD,
	BLOODY_RUSH_16140,
	TUMBLING_16040,
	FURY_METHOD,
	WHIRLWIND,
	POWER_BREAK,
	HELL_BLADE,
	OVERDRIVE,
	BLOODY_SURGE,
	SWORD_STORM,
	SHOULDER_CHARGE_16060,
	ASSAULT_BLADE,
	BERSERK_FURY,
	FIST_ATTACK,
	BURST_16130,
	BLOODY_RUSH_16146,
	MOUNTAIN_CRASH,
	STAMPEDE_16041,
	DOUBLE_SLASH,
	TEMPEST_SLASH,
	BASIC_3_CHAIN_HITS,
	GALAXY_BREAK,
	CHAIN_STRIKE,
	FULL_SWING,
	HEAVY_CRUSH,
	ACTIVATE_HYPERGRAVITY,
	SEISMIC_HAMMER,
	POWER_SHOULDER,
	EARTH_EATER,
	JUMPING_SMASH,
	ENDURE_PAIN,
	GRAVITATIONAL_ENERGY_18200,
	PERFECT_SWING,
	HYPER_BIG_BANG,
	VORTEX_GRAVITY,
	BIG_BANG,
	GRAVITATIONAL_ENERGY_18201,
	SUPERNOVA_18270,
	NEUTRALIZER,
	DREADNAUGHT,
	RUN_18020,
	TERRA_BREAK,
	EARTH_WAVE,
	EARTH_SMASHER,
	GRAVITY_IMPACT,
	POWER_STRIKE,
	GRAVITY_COMPRESSION,
	HYPERGRAVITY_BASIC_ATTACK,
	RUNNING_CRASH,
	DEFENSIVE_STANCE_DEACTIVATION,
	SHIELD_SHOCK_17101,
	RISING_GUNLANCE,
	DASH_UPPER_FIRE,
	SURGE_CANNON,
	GUNLANCE_SHOT,
	SHIELD_CHARGE,
	CHARGED_STINGER,
	LEAP_ATTACK,
	BASIC_ATTACK_3_COMBO_HITS,
	FIST_ATTACK_3_CHAIN_HITS,
	JUSTICE_SERVED,
	BATTLEFIELD_SHIELD,
	GUARDIANS_THUNDERCRACK,
	SHIELD_BASH,
	FIRE_BULLET,
	CHARGE_17020,
	BASH,
	SHIELD_DASH,
	LANCE_OF_JUDGMENT,
	SHIELD_SHOCK_17100,
	SHARP_GUNLANCE,
	DEFENSIVE_STANCE,
	SPEAR_DASH,
	SHOUT_OF_HATRED,
	HOOK_CHAIN,
	GUARDIANS_PROTECTION,
	NELASIAS_ENERGY,
	COUNTER_GUNLANCE,
	GUARDIANS_OATH,
	ALITHANESS_JUDGMENT,
	SWORD_OF_JUSTICE,
	ALITHANESS_RAGE,
	FLASH_THRUST,
	PVP_FORCED_WAKEUP,
	SPRINT_36020,
	HOLY_PROTECTION,
	WRATH_OF_GOD,
	FLASH_SLASH,
	ALITHANESS_DEVOTION,
	HOLY_EXPLOSION,
	HOLY_SWORD,
	HEAVENLY_GUARDIAN_BASIC_ATTACK,
	PUNISHMENT,
	LIGHT_OF_JUDGMENT,
	CHARGE_36070,
	HOLY_AURA,
	SACRED_EXECUTIONER_36910,
	DIVINE_WAVE,
	GODS_DECREE,
	DIVINE_JUSTICE,
	ALITHANESS_LIGHT,
	DASH_SLASH,
	SPIN_SLASH,
	LIGHT_SHOCK,
	UNARMED_BASIC_ATTACK,
	EXECUTION_OF_JUSTICE,
	HOLY_AREA,
	EXECUTION_OF_JUDGMENT,
	HEAVENLY_BLESSINGS,
	JUDGMENT_BLADE,
	EXECUTIONERS_SWORD,
	SACRED_EXECUTIONER_36900,
	MOUNTAIN_CLEAVE,
	VOLCANIC_ERUPTION,
	HURRICANE_SWORD,
	RAGE_SLASHER,
	FLASH_BLADE,
	BURST_45003,
	RAGNA_DEATHBLADE,
	CRUEL_PIERCE,
	CROSS_BLADE,
	FINAL_BLOW,
	FLYING_STRIKE,
	FATAL_SWORD,
	DASH_45010,
	SPIRAL_DEATHBLADE,
	PUNISHING_DRAW,
	WILD_RUSH,
	WILD_STOMP,
	GUILLOTINE,
	GROUND_SMASH,
	BLOODLUST,
	FLAME_DEATHBLADE,
	FURY_BLADE,
	BRUTAL_IMPACT,
	SPINNING_SWORD,
	RAGNA_BREAK,
	WILD_DASH,
	FURIOUS_CLAW,
	EXECUTE,
	REQUIEM_RAIN,
	SERAPHIC_OATH,
	FORESIGHT_SLASH,
	RELEASE_LIGHT_48041,
	LIGHT_OF_THE_FAITHFUL_48052,
	SALVATION_SITE,
	LIGHT_OF_THE_FAITHFUL_48051,
	METEOR_STRIKE,
	FINAL_SPLENDOR,
	TRUTHS_DECREE,
	SWORD_OF_REVELATION,
	LIGHT_OF_THE_FAITHFUL_48053,
	APPROACH_OF_REVELATION,
	CIRCLE_OF_TRUTH,
	ENTER,
	WHISPER_OF_JUDGMENT,
	RELEASE_LIGHT_48042,
	REQUIEM_ASH,
	LIGHT_OF_THE_FAITHFUL_48050,
	LIGHT_OF_THE_FAITHFUL_48054,
	LUNGING_STAB,
	SERAPHIC_LEAP,
	BRUNDIAS_EPIPHANY,
	EXECUTION_OF_REVELATION,
	CATACLYSM,
	BLESSING_OF_SALVATION,
	JUDGMENT_STIGMATA,
	REQUIEM_STORM,
	RELEASE_LIGHT_48040,
	SHINING_KNIGHT,
	BRUNDIAS_RITUAL,
	DIVINE_CONFIRMATION,
	BRUNDIAS_SANCTUARY,
	BRUNDIAS_INCARNATION,
	CRUSHING_CONDEMNATION,
	STREAM_OF_EDGE,
	UNLIMITED_SHUFFLE,
	CULL,
	FOUR_OF_A_KIND,
	THE_TOWER,
	SPIRAL_EDGE_19190,
	BALANCE,
	KNIGHT_OF_THE_EMPRESS,
	SPIRAL_EDGE_19195,
	DARK_RESURRECTION,
	JUDGMENT_19098,
	JOY,
	CLOWN,
	CHECKMATE,
	SECRET_GARDEN,
	EVOKE_19290,
	CHANCELLOR,
	MYSTERIOUS_STAMPEDE_19210,
	DEATH,
	PRISMATIC_MIRROR,
	MOON,
	ROYAL,
	DANCING_OF_SPINEFLOWER,
	CORROSION,
	SCRATCH_DEALER,
	MAYHEM,
	EVOKE_19030,
	TWISTED_FATE,
	VANISH,
	DEALERS_FLIP,
	CALL_OF_DESTINY,
	THE_DEVIL,
	THREE_HEADED_SNAKE,
	EMPRESS,
	DEATHBOUND,
	WHEEL_OF_FORTUNE,
	SOVEREIGN,
	STAR,
	RETURN,
	EVOKE_19300,
	CELESTIAL_RAIN,
	THE_SUN,
	SERENDIPITY,
	GHOST,
	QUADRA_ACCELERATE,
	EMPEROR,
	MYSTERIOUS_STAMPEDE_19215,
	MARIRIN_CHARGE_20138,
	MARIRIN,
	KELSION_20291,
	WINGED_SPIRIT_20220,
	JAHIA_LIGHEAS_20081,
	EARTH_COLLAPSE,
	FLEETING_GALE_BIRD,
	REINES_PROTECTION,
	ELECTRICITY_RELEASE_20231,
	ELECTRICITY_RELEASE_20230,
	FLASH_EXPLOSION_20050,
	CRYSTALLINE_MAGICK,
	MARIRIN_TAUNT_20347,
	PAURU,
	STEED_CHARGE,
	RELEASED_WILL,
	PAURU_FLAME_BREATH_20120,
	WATER_ELEMENTAL,
	ELEMENTAL_WINGS,
	ELCID,
	MARIRIN_STAGGER_20349,
	ALIMAJI_20071,
	AKIR_20310,
	IGNA,
	KELSION_THUNDERCRACK,
	PHOENIX_20281,
	ELECTRIC_STORM,
	PAURU_SELF_DESTRUCT_20125,
	WINGED_SPIRIT_20222,
	JUDGE_KELSION,
	WINGED_SPIRIT_20221,
	FLASH_EXPLOSION_20051,
	KELSION_20293,
	PAURU_SELF_DESTRUCT_20335,
	PHOENIX_20280,
	BAGRONS_WRATH,
	ALIMAJI_20070,
	KELSION_20290,
	PAURU_FLAME_BREATH_20330,
	OSH_20181,
	KELSION_20292,
	STICKY_MOSS_SWAMP,
	AKIR_20311,
	AKIR_BURST,
	SHURDI,
	IGNA_BREATH,
	ANCIENT_SPEAR,
	JAHIA_LIGHEAS_20080,
	MARIRIN_TAUNT_20137,
	MARIRIN_STAGGER_20139,
	MARIPOSA,
	MARIRIN_CHARGE_20348,
	WINGED_SPIRIT_20223,
	BAGRONS_FRENZY,
	OSH_20170,
	HARP_OF_RHYTHM,
	MARCH,
	SONIC_VIBRATION,
	SERENADE_OF_COURAGE_21141,
	SERENADE_OF_SALVATION_21132,
	MAJOR_CHORD,
	CONCERTO,
	TEMPEST_21149,
	RHAPSODY_OF_LIGHT,
	SOUND_WAVE,
	SERENADE_OF_SALVATION_21131,
	SERENADE_OF_COURAGE_21140,
	SYMPHONIA,
	DISSONANCE,
	SYMPHONY_MELODY,
	PRELUDE_OF_STORM,
	NOTE_BUNDLE,
	MINOR_CHORD,
	PRELUDE_OF_DEATH,
	VIVACE,
	SOUND_SHOCK,
	SERENADE_OF_COURAGE_21142,
	SERENADE_OF_COURAGE_21143,
	RHYTHM_BUCKSHOT,
	SONATINA,
	TEMPEST_21147,
	ARIA,
	TEMPEST_21148,
	SOUNDHOLIC,
	GUARDIAN_TUNE,
	HEAVENLY_TUNE,
	SOUND_ILLUSION,
	SERENADE_OF_SALVATION_21130,
	SERENADE_OF_SALVATION_21133,
	STIGMA,
	ORATORIO,
	WIND_OF_MUSIC_21070,
	WIND_OF_MUSIC_21079,
	REVERSE_GRAVITY,
	APOCALYPSE,
	ARCANE_RUPTURE_37101,
	ICE_SHOWER,
	DOOMSDAY,
	APOCALYPSE_CALL,
	ESOTERIC_REACTION,
	RUSHING_GLACIER,
	BLINK,
	SQUALL,
	FROSTS_CALL_37341,
	EXPLOSION,
	ENERGY_DISCHARGE,
	LIGHTNING_VORTEX,
	ENVISKAS_MIGHT,
	ARCANE_RUPTURE_37100,
	TRANSCENDENT_JUDGMENT,
	PUNISHING_STRIKE_37270,
	PHASE_LEAP,
	LAVA_BLAST,
	BLAZE,
	SERAPHIC_HAIL,
	ELEGIANS_TOUCH_37290,
	INFERNO,
	ELEGIANS_TOUCH_37291,
	FROSTS_CALL_37340,
	LIGHTNING_BOLT,
	RIME_ARROW,
	ULTIMATE_SKILL_GREAT_RAGING_DEMON_KICK,
	FLASH_HEAT_FANG_22060,
	ESOTERIC_SKILL_CALL_OF_THE_WIND_GOD_22110,
	LIGHTNING_KICK_22170,
	SWIFT_WIND_KICK_22280,
	SWEEPING_KICK_22320,
	ESOTERIC_SKILL_BLAST_FORMATION_22020,
	CHARGING_STEPS_22150,
	PHOENIX_ADVENT_22130,
	ULTIMATE_SKILL_EIGHT_TRIGRAMS_CHAOTIC_STRIKE,
	ULTIMATE_SKILL_FLASH_RAGE_BLOW,
	ULTIMATE_SKILL_FIST_OF_DOMINANCE,
	DEADLY_DIVE,
	TRIPLE_FIST_22180,
	MOON_FLASH_KICK_22190,
	SKY_SHATTERING_BLOW_22160,
	SWEEPING_HIDDEN_DRAGON_22070,
	ESOTERIC_SKILL_THUNDERCLAP_KICK,
	ESOTERIC_ORIGIN,
	ESOTERIC_SKILL_RISING_FIRE_DRAGON,
	ENERGY_COMBUSTION_22290,
	LEAPING_DRAGON,
	ESOTERIC_SKILL_AZURE_DRAGON_SUPREME_FIST,
	WINDS_WHISPER,
	ROAR_OF_COURAGE,
	ENERGY_COMBUSTION_22291,
	ESOTERIC_SKILL_SPIRAL_IMPACT_22030,
	CRUSHING_SMITE,
	CHARGING_BLOW,
	EARTH_REND,
	EXPLOSIVE_FIST_23300,
	SHREDDING_STRIKE,
	DIVINE_DRAGON_CREATION,
	MYSTERIOUS_ART_BLAST_OF_RUINATION,
	ROUNDUP_SWEEP,
	SUPERNOVA_23130,
	UNDEFEATED_DRAGON_KING,
	CONTINUOUS_PUSH,
	INSTANT_HIT,
	DUCK_23171,
	FIST_OF_THE_WIND_GOD,
	JUDGMENT_23060,
	FIERCE_TIGER_STRIKE,
	POTENT_RISING_FIST,
	IRON_CANNON_BLOW,
	BLAZING_BOMBARDMENT,
	DUCK_23172,
	BATTERING_FISTS,
	HEAVEN_AND_EARTH_STRIKE,
	CHAIN_DESTRUCTION_FIST,
	SUPREME_HEAVEN_SHATTERING_FIST,
	CRITICAL_BLOW,
	TENACITY_RELEASE,
	DRAGON_ADVENT,
	DEATH_RATTLE,
	DUCK_23170,
	RULE_THE_WORLD,
	SHADOWBREAKER,
	BOLTING_CRASH,
	LEVEL_3_HYPE_24022,
	ENERGY_RELEASE,
	NEBULOUS_STEP,
	ILLUSION_STRIKE,
	FLASH_STEP_24242,
	ENERGY_BLAST,
	CELESTIAL_PALM,
	SKY_SLASH,
	LEVEL_3_HYPE_24025,
	HYPE,
	LEVEL_2_HYPE,
	FALLING_SUN,
	WORLD_DECIMATION,
	MAGNETIC_PALM,
	FLASH_STEP_24241,
	CRIPPLING_BARRIER,
	FORCE_ORB,
	SUPERNOVA_PURGATION_RAY,
	LIGHTNING_PALM,
	TEMPEST_BLAST,
	PALM_BURST,
	ENERGY_BULLET,
	DEADLY_FINGER,
	PULVERIZING_PALM,
	DECIMATION_RAY,
	FLASH_STEP_24240,
	LEVEL_3_HYPE_24023,
	MERCILESS_PUMMEL,
	BREAKTHROUGH,
	HALF_MOON_SLASH,
	YEON_STYLE_SPEAR_TECHNIQUE_GALAXY_FLYING_SPEAR,
	STAMPEDING_SLASH_34061,
	FOCUS_STANCE_34001,
	RED_DRAGONS_HORN,
	DEADLY_RED_DRAGON,
	DRAGONS_RAMPAGE,
	FLASH_KICK,
	STAMPEDING_SLASH_34060,
	STARFALL_POUNCE,
	FOCUS_STANCE_34000,
	SHACKLING_BLUE_DRAGON,
	WHEEL_OF_BLADES,
	THRUST_OF_DESTRUCTION,
	THORN_JAB,
	CUTTING_WIND,
	YEON_STYLE_ENCORE,
	SPIRALING_SPEAR,
	YEON_STYLE_TECHNIQUE,
	SOUL_CUTTER,
	CHAIN_SLASH,
	MIRAGE_DASH,
	DOUBLE_STRIKE,
	DRAGONSCALE_DEFENSE,
	WINDSPLITTER,
	RAGING_DRAGON_SLASH,
	SPEAR_DIVE,
	FLURRY_STANCE_34500,
	YEON_STYLE_SPEAR_TECHNIQUE_DRAGON_CAVALRY_UNITY_SLASH,
	BLUE_DRAGONS_CLAW,
	FLURRY_STANCE_34501,
	FOUR_HEADED_DRAGON,
	YEON_STYLE_SPEAR_TECHNIQUE_SPEAR_METEOR,
	YEON_STYLE_SPEAR_TECHNIQUE_STORMING_RED_DRAGON,
	SWIFT_WIND_KICK_39280,
	FLASH_HEAT_FANG_39060,
	ESOTERIC_SKILL_CHARGING_KICK,
	SKY_SHATTERING_BLOW_39160,
	ESOTERIC_SKILL_TIGER_EMERGES,
	ORBS_BLESSING,
	ADVANCING_STRIKE,
	ESOTERIC_SKILL_LIGHTNING_TIGER_STRIKE,
	STORM_DRAGON_KICK_39122,
	STORM_DRAGON_KICK_39120,
	SWEEPING_HIDDEN_DRAGON_39070,
	SWEEPING_KICK_39320,
	VICIOUS_TIGER_DANCE,
	CHARGING_STEPS_39150,
	ESOTERIC_SKILL_CALL_OF_THE_WIND_GOD_39110,
	BERSERK_CIRCLE,
	TRIPLE_FIST_39180,
	ESOTERIC_SKILL_SPIRAL_IMPACT_39030,
	ULTIMATE_SKILL_NOVA_BLAST,
	ULTIMATE_SKILL_MOUNTAIN_LORDS_EXPLOSIVE_ROAR,
	LIGHTNING_WHISPER,
	PHOENIX_ADVENT_39130,
	LIGHTNING_KICK_39170,
	ULTIMATE_SKILL_THUNDERBOLT_KICK,
	MOON_FLASH_KICK_39190,
	ULTIMATE_SKILL_POTENT_HEAVENLY_KICK,
	STORM_DRAGON_KICK_39121,
	ESOTERIC_SKILL_BLAST_FORMATION_39020,
	CELESTIAL_FIST,
	ASURA_FEATHERWEIGHT,
	EYE_OF_THE_STORM,
	ASURA_DESTRUCTION_BASIC_ATTACK,
	BRAWL_KINGS_ADVANCE,
	OBLITERATION,
	FLASHSTEP,
	HEAVENLY_KINGS_WAR_DANCE,
	FEATHERWEIGHT,
	HAYMAKER,
	PUNISHING_WAVE,
	CRATER_STRIKE,
	ASURA_DESTRUCTION,
	HEAVENLY_PUNISHMENT,
	YEON_GALESTORM_STRIKE,
	ZAFFRE_NOVA,
	ADAMANTINE_ASSAULT,
	BUTTERFLY_STING,
	BRAWL_KING_TWELVE_FORMS_VIOLENT_WAVES,
	NEBULA_CRUSHER,
	HURRICANE_CHAIN,
	BRAWL_KING_TWELVE_FORMS_FALLING_BLOSSOMS,
	BRAWL_KING_STANCE,
	CELESTIAL_FORCE_BARRAGE,
	FALLING_STAR,
	SMASHING_BLOW,
	CELESTIAL_DOOR,
	HUNDRED_FISTS,
	EXPLOSIVE_FIST_47230,
	EARTHBREAKER,
	SPIRAL_UPPERCUT,
	TURNING_SLASH,
	DEATH_TRANCE_25030,
	BREAKING_MOON,
	FALLSTAR,
	FLASH_BLINK,
	DEATH_SENTENCE,
	FATAL_WAVE,
	EARTH_CLEAVER,
	SPINCUTTER,
	WIND_CUT,
	SOUL_ABSORBER,
	LEVEL_3,
	SURPRISE_ATTACK,
	ZERO,
	MAELSTROM_25160,
	BLADE_ASSAULT,
	DEATH_TRANCE_25032,
	HIGH_SPEED_MOVE,
	ETERNAL_FLASH,
	MOONLIGHT_SONIC,
	DARK_AXEL,
	HEAD_HUNT,
	CHAOTIC_DEATHBLADE,
	UPPER_SLASH,
	LEVEL_2,
	VOID_STRIKE,
	DEATH_TRANCE_25031,
	BLITZ_RUSH,
	LEVEL_1,
	TWIN_SHADOWS,
	DEATHLY_SLASH,
	BLADE_DANCE,
	SHARPENED_CUT,
	DESTRUCTION,
	RUSH,
	BLOOD_MARSH,
	PIERCING_THORN,
	WILD_SLASH,
	NIMBLE_CUT,
	DEMONIC_CLONE,
	LEAPING_BLOW,
	SLASHER,
	BLOODY_PIERCING,
	DEMONIC_SLASH_27051,
	BRUTAL_CROSS,
	RIFT_WALK,
	FALLEN_RUIN,
	THRUST_IMPACT,
	RUINING_RUSH,
	DEMONS_GRIP,
	HOWL,
	GATE_OF_ERUPTION,
	DEMOLITION,
	SPINNING_WEAPON,
	DEATH_CLAW,
	GRIND_CHAIN,
	DARKNESS_BLAST,
	DECIMATE,
	CRUEL_CUTTER,
	DEMONIZE,
	SPINNING_DIVE,
	GORE_BLEEDING,
	CANCEL_DEMONIZATION,
	BLOOD_MASSACRE,
	DEMON_VISION,
	STORM_GRINDING,
	DEMONIC_SLASH_27050,
	RAY_OF_RUIN,
	RISING_CLAW,
	LUNAR_ECLIPSE_CADENZA,
	REAPERS_CALL,
	CADENZA_DE_LA_LUNA,
	CALL_OF_THE_KNIFE,
	SHADOW_DOUBLE_26530,
	REQUIEM_DEL_SOL,
	FINISHING_STEP,
	PHANTOM_DANCER_26140,
	DISTORTION_26511,
	FATAL_STEP,
	DEATHSCYTHE,
	NIGHTMARE_26100,
	PHANTOM_DANCER_26150,
	SHADOW_KNIFE,
	SHADOW_DOUBLE_26610,
	DANCE_OF_FURY,
	SILENT_RAGE,
	DISTORTION_26510,
	SHADOW_TRAP,
	SPIRIT_CATCH,
	SHADOW_VORTEX,
	RAGE_SPEAR,
	SHADOW_STORM,
	SOLAR_ECLIPSE_REQUIEM_26920,
	NIGHTMARE_26090,
	PERSONA,
	SOLAR_ECLIPSE_REQUIEM_26910,
	SHADOW_STEP,
	BLACK_MIST,
	SHADOW_DOUBLE_26520,
	_26500,
	SPINNING_DAGGER,
	PIERCING_BLADE,
	SOLAR_ECLIPSE_REQUIEM_26930,
	GLOWING_BRAND,
	DEATH_YARD,
	GLUTTONY,
	SCYTHE_DASH_46020,
	THE_END,
	DEATH_SLASH,
	FATAL_FINALE,
	DEADLY_COMBINATION,
	VESTIGE,
	DEATH_ORDER,
	SPECTRAL_COFFIN,
	RUSTED_NAIL,
	ASTAROS,
	DEATHBRINGER,
	HARVEST,
	REAPERS_SCYTHE,
	LUNATIC_EDGE,
	LETHAL_SPINNING,
	FRENZY_SCYTHE,
	THRASHING_46235,
	GUILLOTINE_SWING,
	SOUL_DECAPITATION,
	THRASHING_46230,
	SCYTHE_DASH_46021,
	DEATHLORD_MODE,
	FATALITY,
	GHOST_STEP,
	SOUL_DRAIN,
	LOCK_ON_28291,
	SILVERHAWK_BASIC_ATTACK,
	SMOKESCREEN_ARROW,
	CHARGED_SHOT,
	SNIPE,
	SILVERHAWK_ASSAULT,
	SHADOW_ARROW,
	RAPID_SHOT,
	ATOMIC_ARROW,
	WINGS_OF_STORM_28160,
	EVASIVE_FIRE,
	CLAYMORE_MINE,
	SUMMON_SILVERHAWK_28157,
	AAGA_DEADEYE,
	SPIRAL_ARROW,
	SUMMON_SILVERHAWK_28156,
	ARROW_WAVE,
	GOLDEN_EYE,
	BLADE_STORM,
	WINGS_OF_STORM_28320,
	ARROW_SHOWER,
	MOVING_SLASH,
	HAWK_SHOT,
	FENRIRS_MESSENGER,
	SALVO,
	DM_FOUR2_28120,
	DEADLY_SLASH,
	SLIDE,
	LAST_RUSH,
	DM_FOUR2_28125,
	GIGANTAR_BOW_FENRIR,
	LOCK_ON_28290,
	SUMMON_SILVERHAWK_28158,
	SHARPSHOOTER_28110,
	STALKER,
	CLAY_BOMBARDMENT,
	DESPERADO,
	DEADLY_CAGE,
	SECRET_WEAPON,
	METEOR_STREAM_29100,
	DEXTEROUS_SHOT_29200,
	PERFECT_SHOT_29270,
	SHOTGUN_DOMINATOR,
	DOOM_SHOT,
	BLAUER_BLITZ,
	DEATH_FIRE_29090,
	CHANGE_STANCE_29162,
	ENFORCE_EXECUTION,
	SPIRAL_FLAME_29060,
	SPIRAL_TRACKER_29180,
	CRUEL_TRACKER,
	CHANGE_STANCE_29163,
	QUICK_FIRE,
	EQUILIBRIUM_29220,
	LAST_REQUEST_29230,
	BURSTING_FLARE,
	AT02_GRENADE_29120,
	SHOTGUN_RAPID_FIRE_29170,
	RELENTLESS_ASSAULT,
	MARK_OF_DEATH,
	SIGN_OF_APOCALYPSE_29040,
	AIMED_SHOT,
	CHANGE_STANCE_29161,
	SIGN_OF_APOCALYPSE_29041,
	JUDGMENT_DAY,
	CHANGE_STANCE_29160,
	ONE_SHOT_ONE_KILL,
	TUMBLING_29150,
	SOMERSAULT_SHOT_29210,
	QUICK_SHOT_29261,
	CATASTROPHE_29080,
	CHANGE_STANCE_29164,
	QUICK_SHOT_29262,
	QUICK_SHOT_29260,
	PLASMA_BULLET_29140,
	JUMP_BARRAGE,
	FORWARD_BARRAGE,
	HOMING_BARRAGE,
	KINDLE,
	BARRAGE_FOCUS_FIRE,
	NAPALM_SHOT,
	ENERGY_FIELD,
	MISSILE_BARRAGE,
	GRAVITY_EXPLOSION,
	HOWITZER,
	SUMMON_TURRET,
	BUCKSHOT,
	SWING_30140,
	HEAVY_TURRET,
	PHOTOELECTRONIC_CANNON,
	BARRAGE_STEEL_RAIN,
	EXIT_BARRAGE_MODE,
	MULTI_ROCKET_LAUNCHER,
	BARRAGE_HOWITZER,
	BARRAGE_ENERGY_CANNON,
	BARRAGE_ATTACK,
	BARRAGE_BASIC_ATTACK,
	AIR_RAID,
	ROLL_30015,
	ENTER_BARRAGE_MODE,
	ACOM_ATTACK,
	FLAMETHROWER,
	BARRAGE_IMPREGNABILITY,
	ACOM_BOMBARDMENT_SUPPORT,
	ENHANCED_SHELL,
	BARRAGE_ACT,
	PLASMA_STORM,
	GATLING_GUN,
	MISSILE_LAUNCHER,
	PRESSURIZED_HEATBOMB,
	COMMAND_BABY_DRONES,
	THRUSTER_MOVE,
	COMMAND_RAID_MISSILE,
	POINT_EXCLUSION,
	FIERY_ESCAPE,
	ANNIHILATION_MODE,
	AIR_STRIKE,
	COMMAND_BLOCKADE,
	COMET_STRIKE,
	ECHELON_BEAM,
	ENERGY_BUSTER,
	SLUGSHOT,
	PROJECT_TITAN,
	OVERCHARGED_BATTERY,
	MOBILE_SHOT_35181,
	DISABLE_HYPERSYNC,
	BACKFLIP_STRIKE,
	MOBILE_SHOT_35182,
	HYPERSYNC,
	BATTLESHIP_OPERATION,
	SYNC_PROTOCOL,
	COMMAND_CALL_BACK,
	LASER_BLADE,
	PULSE_FIRE,
	TREMORS,
	COMMAND_CARPET,
	COMMAND_ACTIVE_PULSE,
	STRATEGIC_FIRE,
	COMMAND_FLARE_BEAM,
	NEO_FIRE,
	SURGE_BLOW,
	BULLET_HAIL,
	HIGH_VOLTAGE_BULLET,
	COMMAND_M1FOUR3_MACHINE_GUN,
	AVALANCHE,
	CRIMSON_BREAKER,
	EX_ZERO_POINT,
	MOBILE_SHOT_35180,
	TARGET_DOWN,
	HIGH_CALIBER_HE_BULLET,
	SPIRAL_TRACKER_38180,
	DEXTEROUS_SHOT_38200,
	DUAL_BUCKSHOT,
	FOCUSED_SHOT,
	BULLET_RAIN,
	SPIRAL_FLAME_38060,
	HOUR_OF_JUDGMENT,
	AT02_GRENADE_38120,
	CHANGE_STANCE_38162,
	PEACEKEEPER,
	EYE_OF_TWILIGHT,
	SOMERSAULT_SHOT_38210,
	REVERSE_STANCE_SWITCH,
	STAMPEDE_38150,
	PLASMA_BULLET_38140,
	LAST_REQUEST_38230,
	ATOMIC_EXPLOSION,
	CHANGE_STANCE_38163,
	DEATH_FIRE_38090,
	SHARPSHOOTER_38110,
	CATASTROPHE_38080,
	CHANGE_STANCE_38164,
	EQUILIBRIUM_38220,
	QUICK_STEP,
	BULLET_PRISON,
	METEOR_STREAM_38100,
	BULLSEYE,
	ROSE_BLOSSOM,
	PERFECT_SHOT_38270,
	DEAD_END,
	SHOTGUN_RAPID_FIRE_38170,
	CHANGE_STANCE_38160,
	SEVEN_SHOTGUNS,
	STROKE_SPILL_15020,
	STROKE_HERE_AND_THERE_15030,
	STROKE_HERE_AND_THERE_15040,
	STROKE_HOPPER,
	SUNRISE_31141,
	MOONFALL_31050,
	PAINT_DRAGON_ENGRAVING,
	MASTERWORK_EFFLORESCENCE,
	PAINT_MOONSKETCH,
	SUNRISE_31140,
	STROKE_INK_SHOWER,
	STROKE_SPRINKLE,
	SUNRISE_31131,
	PAINT_DRAWING_ORCHIDS,
	STROKE_HERE_AND_THERE_31030,
	RISING_MOON,
	PAINT_STARRY_NIGHT,
	SUNRISE_31130,
	PAINT_INK_WELL,
	PAINT_POUNCING_TIGER,
	STROKE_UPWARD_STROKE,
	MASTERWORK_SPECTACLE,
	MOONFALL_31051,
	PAINT_SUNSKETCH,
	HOLY_BEAST_SUMMON_PHOENIX,
	SUNRISE_31120,
	PAINT_SUN_WELL,
	PAINT_SHATTERING_STRIKE,
	SUNRISE_31121,
	STROKE_SPILL_31020,
	PAINT_ILLUSION_DOOR,
	SUNRISE_31110,
	PAINT_CRANE_WING,
	PAINT_CATTLE_DRIVE,
	PAINT_BUTTERFLY_DREAM,
	STROKE_ONE_STROKE,
	PAINT_INKRISE,
	STROKE_HERE_AND_THERE_31040,
	DREAM_BLOSSOM_GARDEN,
	WIND_GIMLET,
	SPRING_BREEZE,
	PIERCING_WIND,
	THICK_FOG,
	LEAP_32030,
	WHIRLPOOL,
	SPRINTING,
	KAHNS_TERRITORY,
	SUN_SHOWER_32040,
	STRONG_WIND,
	TORNADO,
	FACE_TO_FACE,
	SUMMER_SUNLIGHT,
	SUN_SHOWER_32041,
	RAGE,
	DAZZLING_DAYS,
	TORNADO_DANCE,
	NARRS_BLADE,
	SPREAD,
	SPACE_CLEAVE,
	DOWNWARD_STRIKE_32130,
	RAINSTORM,
	STORMS_APPROACH,
	WIPING_WIND,
	THUNDERWIND,
	FLY,
	SCORCHING_SUN,
	AKASHAS_WAVE,
	FLOAT,
	DIGGER_BEAR,
	THUMP_AND_THUD,
	FOX_ORB,
	BEAR_FORM_BASIC_ATTACK,
	SWISH_BEAR_33201,
	FOX_FLAME_33321,
	BOING,
	REMOVE_SHAPESHIFT_33041,
	CROW_PARADE,
	FOX_FLAME_33320,
	FOX_LEAP_33331,
	CROW_COMMOTION,
	BOULDER_BEAR_33231,
	ZOOM_33020,
	FORBIDDEN_SORCERY_RIPPING_BEAR,
	TAILWIND,
	VOILA,
	SMACK_SMITE,
	ONE_HIT_BEAR,
	ROLLING_WHEEL,
	PHANTOM_BEAST_AWAKENING_33050,
	WHOOSH_33030,
	TUMBLE,
	BOULDER_BEAR_33230,
	REMOVE_SHAPESHIFT_33040,
	FOX_FORM_BASIC_ATTACK,
	FLOATING_FOXBEAR,
	URSINE_WINDUP,
	GROWLING_BEAR,
	FOX_ILLUSION_33310,
	VULPINE_VELOCITY,
	EARTHQUAKE_POUND,
	FOX_FIRE_DANCE,
	RUTHLESS_PECKING,
	FORBIDDEN_SORCERY_FOX_STAR_RAINSTORM,
	PHANTOM_BEAST_AWAKENING_33060,
	CLAW,
	FOX_LEAP_33330,
	FOX_ILLUSION_33311,
	CHAOS_BRAWL,
	SWISH_BEAR_33200,
	DART,
];

pub const ALL_COUNTER_SKILL_IDS: &'static [u32] = &[
    CHAIN_SWORD.id,
    DREADNAUGHT.id,
    POWER_STRIKE.id,
    BASH.id,
    DASH_UPPER_FIRE.id,
    HOLY_SWORD.id,
    EXECUTIONERS_SWORD.id,
    SERENDIPITY.id,
    PRELUDE_OF_STORM.id,
    RHYTHM_BUCKSHOT.id,
    SQUALL.id,
    ICE_SHOWER.id,
    ESOTERIC_SKILL_SPIRAL_IMPACT_22030.id,
    ESOTERIC_SKILL_SPIRAL_IMPACT_39030.id,
    INSTANT_HIT.id,
    ROUNDUP_SWEEP.id,
    BOLTING_CRASH.id,
    LIGHTNING_PALM.id,
    EARTH_CLEAVER.id,
    HEAD_HUNT.id,
    RISING_CLAW.id,
    DEATH_CLAW.id,
    SHADOW_TRAP.id,
    LAST_REQUEST_29230.id,
    LAST_REQUEST_29230.id,
    NAPALM_SHOT.id,
    SWING_30140.id,
    EVASIVE_FIRE.id,
    DEADLY_SLASH.id,
    BACKFLIP_STRIKE.id,
    ECHELON_BEAM.id,
    PEACEKEEPER.id,
];