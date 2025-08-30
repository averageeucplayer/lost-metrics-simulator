use std::hash::Hash;

use serde::{Deserialize, Serialize};
use strum_macros::VariantArray;
use strum_macros::{AsRefStr, EnumString};

use crate::templates::skill::ALL_CLASS_SKILLS;
use crate::templates::skill_buff::{DPS_CRIT_SWIFT_PET_BUFFS, IMPROVED_CRIT, VITALITY_TRAINING};
use crate::types::{Class, Skill, SkillBuff, SkillType};

#[derive(Debug, Clone)]
pub struct Preset {
    pub is_support: bool,
    pub crit_rate: f64,
    pub cooldown_reduction: f64,
    pub status_effects: Vec<SkillBuff>,
    pub skills: Vec<Skill>,
    pub identity_skills: Vec<Skill>,
    pub transformation_skills: Vec<Skill>,
    pub hyper_awakening_technique_skill: Skill,
    pub awakening_skill: Skill,
    pub hyper_awakening_skill: Skill,
}

impl Preset {
    pub fn default_for_class(class: Class) -> Self {

        let skills: Vec<Skill> = ALL_CLASS_SKILLS.iter()
            .filter(|pr| pr.class_id == class)
            .cloned()
            .collect();

        let identity_skills: Vec<Skill> = skills.iter()
            .filter(|pr| pr.kind == SkillType::Identity)
            .cloned()
            .collect();

        let normal_skills: Vec<Skill> = skills.iter()
            .filter(|pr| pr.kind == SkillType::Normal)
            .cloned()
            .collect();

        let hyper_awakening_technique_skill = skills.iter()
            .filter(|pr| pr.kind == SkillType::HyperAwakeningTechnique)
            .next()
            .cloned()
            .unwrap();

        let hyper_awakening_skill = skills.iter()
            .filter(|pr| pr.kind == SkillType::HyperAwakening)
            .next()
            .cloned()
            .unwrap();

        let awakening_skill = skills.iter()
            .filter(|pr| pr.kind == SkillType::Awakening)
            .next()
            .cloned()
            .unwrap();

        Self {
            is_support: false,
            cooldown_reduction: 0.18,
            crit_rate: 0.0,
            hyper_awakening_technique_skill,
            hyper_awakening_skill,
            awakening_skill,
            skills: normal_skills,
            transformation_skills: vec![],
            identity_skills,
            status_effects: DPS_CRIT_SWIFT_PET_BUFFS.to_owned()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ClassWithSpecialisation {
    Berserker(BerserkerSpecialisation),
    Destroyer(DestroyerSpecialisation),
    Gunlancer(GunlancerSpecialisation),
    Paladin(PaladinSpecialisation),
    Slayer(SlayerSpecialisation),
    Valkyrie(ValkyrieSpecialisation),
    Arcanist(ArcanaSpecialisation),
    Summoner(SummonerSpecialisation),
    Bard(BardSpecialisation),
    Sorceress(SorceressSpecialisation),
    Wardancer(WardancerSpecialisation),
    Scrapper(ScrapperSpecialisation),
    Soulfist(SoulfistSpecialisation),
    Glaivier(GlaivierSpecialisation),
    Striker(StrikerSpecialisation),
    Breaker(BreakerSpecialisation),
    Deathblade(DeatbladeSpecialisation),
    Shadowhunter(ShadowhunterSpecialisation),
    Reaper(ReaperSpecialisation),
    Souleater(SouleaterSpecialisation),
    Sharpshooter(SharpshooterSpecialisation),
    Deadeye(DeadeyeSpecialisation),
    Artillerist(ArtilleristSpecialisation),
    Machinist(ScouterSpecialisation),
    Gunslinger(GunslingerSpecialisation),
    Artist(ArtistSpecialisation),
    Aeromancer(AeromancerSpecialisation),
    Wildsoul(WildsoulSpecialisation)
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum BerserkerSpecialisation {
    #[strum(serialize = "Berserker Technique")]
    BerserkerTechnique = 2160000,
    #[strum(serialize = "Mayhem")]
    Mayhem = 2160010,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum GunlancerSpecialisation {
    #[strum(serialize = "Lone Knight")]
    LoneKnight = 2170000,
    #[strum(serialize = "Combat Readiness")]
    CombatReadiness = 2170010,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum DestroyerSpecialisation {
    #[strum(serialize = "Rage Hammer")]
    RageHammer = 2180000,
    #[strum(serialize = "Gravity Training")]
    GravityTraining = 2180010,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum PaladinSpecialisation {
    #[strum(serialize = "Judgement")]
    Judgement = 2360000,
    #[strum(serialize = "Blessed Aura")]
    BlessedAura = 2360010,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum SlayerSpecialisation {
    #[strum(serialize = "Punisher")]
    Punisher = 2450000,
    #[strum(serialize = "Predator")]
    Predator = 2450010,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum ScrapperSpecialisation {
    #[strum(serialize = "Ultimate Skill: Taijutsu")]
    UltimateSkillTaijutsu = 2230000,
    #[strum(serialize = "Shock Training")]
    ShockTraining = 2230100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum WardancerSpecialisation {
    #[strum(serialize = "First Intention")]
    FirstIntention = 2220000,
    #[strum(serialize = "Esoteric Skill Enhancement")]
    EsotericSkillEnhancement = 2220100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum SoulfistSpecialisation {
    #[strum(serialize = "Energy Overflow")]
    EnergyOverflow = 2240000,
    #[strum(serialize = "Robust Spirit")]
    RobustSpirit = 2240100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum GlaivierSpecialisation {
    #[strum(serialize = "Control")]
    Control = 2340000,
    #[strum(serialize = "Pinnacle")]
    Pinnacle = 2340100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum BreakerSpecialisation {
    #[strum(serialize = "Brawl King Storm")]
    BrawlKingStorm = 2470000,
    #[strum(serialize = "Asura's Path")]
    AsurasPath = 2470100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum StrikerSpecialisation {
    #[strum(serialize = "Esoteric Flurry")]
    EsotericFlurry = 2390000,
    #[strum(serialize = "Deathblow")]
    Deathblow = 2390010,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum ArtilleristSpecialisation {
    #[strum(serialize = "Barrage Enhancement")]
    BarrageEnhancement = 2300000,
    #[strum(serialize = "Firepower Enhancement")]
    FirepowerEnhancement = 2300100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum DeadeyeSpecialisation {
    #[strum(serialize = "Enhanced Weapon")]
    EnhancedWeapon = 2290000,
    #[strum(serialize = "Pistoleer")]
    Pistoleer = 2290100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum SharpshooterSpecialisation {
    #[strum(serialize = "Death Strike")]
    DeathStrike = 2280000,
    #[strum(serialize = "Loyal Companion")]
    LoyalCompanion = 2280100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum ScouterSpecialisation {
    #[strum(serialize = "Evolutionary Legacy")]
    EvolutionaryLegacy = 2350000,
    #[strum(serialize = "Arthetinean Skill")]
    ArthetineanSkill = 2350100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum GunslingerSpecialisation {
    #[strum(serialize = "Peacemaker")]
    Peacemaker = 2380000,
    #[strum(serialize = "Time to Hunt")]
    TimeToHunt = 2380100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum SorceressSpecialisation {
    #[strum(serialize = "Igniter")]
    Igniter = 2370000,
    #[strum(serialize = "Reflux")]
    Reflux = 2370100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum ArcanaSpecialisation {
    #[strum(serialize = "Grace of the Empress")]
    GraceOfTheEmpress = 2190000,
    #[strum(serialize = "Order of the Emperor")]
    OrderOfTheEmperor = 2190100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum SummonerSpecialisation {
    #[strum(serialize = "Communication Overflow")]
    CommunicationOverflow = 2200000,
    #[strum(serialize = "Master Summoner")]
    MasterSummoner = 2200100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum BardSpecialisation {
    #[strum(serialize = "Desperate Salvation")]
    DesperateSalvation = 2210000,
    #[strum(serialize = "True Courage")]
    TrueCourage = 2210100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum ShadowhunterSpecialisation {
    #[strum(serialize = "Demonic Impulse")]
    DemonicImpulse = 2270000,
    #[strum(serialize = "Perfect Suppression")]
    PerfectSuppression = 2270600,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum DeatbladeSpecialisation {
    #[strum(serialize = "Surge")]
    Surge = 2250000,
    #[strum(serialize = "Remaining Energy")]
    RemainingEnergy = 2250600,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum ReaperSpecialisation {
    #[strum(serialize = "Lunar Voice")]
    LunarVoice = 2260000,
    #[strum(serialize = "Hunger")]
    Hunger = 2260600,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum SouleaterSpecialisation {
    #[strum(serialize = "Full Moon Harvester")]
    FullMoonHarvester = 2460000,
    #[strum(serialize = "Night's Edge")]
    NightsEdge = 2460600,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum AeromancerSpecialisation {
    #[strum(serialize = "Wind Fury")]
    WindFury = 2320000,
    #[strum(serialize = "Drizzle")]
    Drizzle = 2320600,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum ArtistSpecialisation {
    #[strum(serialize = "Full Bloom")]
    FullBloom = 2310000,
    #[strum(serialize = "Recurrence")]
    Recurrence = 2310600,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum ValkyrieSpecialisation {
    #[strum(serialize = "Shining Knight")]
    ShiningKnight = 2480000,
    #[strum(serialize = "Liberator")]
    Liberator = 2480100,
}

#[derive(Debug, Clone, Copy, EnumString, AsRefStr)]
#[repr(u32)]
pub enum WildsoulSpecialisation {
    #[strum(serialize = "Phantom Beast Awakening")]
    PhantomBeastAwakening = 2330100,
    Ferality = 2330000,
}

#[derive(Default, Debug, Serialize, Deserialize, Copy, Hash, Clone, PartialEq, Eq, AsRefStr, EnumString, VariantArray)]
#[repr(u32)]
pub enum Specialisation {
    #[default]
    Unknown = 0,
    #[strum(serialize = "Berserker Technique")]
    BerserkerTechnique = 2160000,
    Mayhem = 2160010,
    #[strum(serialize = "Lone Knight")]
    LoneKnight = 2170000,
    #[strum(serialize = "Combat Readiness")]
    CombatReadiness = 2170010,
    #[strum(serialize = "Rage Hammer")]
    RageHammer = 2180000,
    #[strum(serialize = "Gravity Training")]
    GravityTraining = 2180010,
    #[strum(serialize = "Judgement")]
    Judgement = 2360000,
    #[strum(serialize = "Blessed Aura")]
    BlessedAura = 2360010,
    #[strum(serialize = "Punisher")]
    Punisher = 2450000,
    #[strum(serialize = "Predator")]
    Predator = 2450010,
    #[strum(serialize = "Ultimate Skill: Taijutsu")]
    UltimateSkillTaijutsu = 2230000,
    #[strum(serialize = "Shock Training")]
    ShockTraining = 2230100,
    #[strum(serialize = "First Intention")]
    FirstIntention = 2220000,
    #[strum(serialize = "Esoteric Skill Enhancement")]
    EsotericSkillEnhancement = 2220100,
    #[strum(serialize = "Energy Overflow")]
    EnergyOverflow = 2240000,
    #[strum(serialize = "Robust Spirit")]
    RobustSpirit = 2240100,
    #[strum(serialize = "Control")]
    Control = 2340000,
    #[strum(serialize = "Pinnacle")]
    Pinnacle = 2340100,
    #[strum(serialize = "Brawl King Storm")]
    BrawlKingStorm = 2470000,
    #[strum(serialize = "Asura's Path")]
    AsurasPath = 2470100,
    #[strum(serialize = "Esoteric Flurry")]
    EsotericFlurry = 2390000,
    #[strum(serialize = "Deathblow")]
    Deathblow = 2390010,
    #[strum(serialize = "Barrage Enhancement")]
    BarrageEnhancement = 2300000,
    #[strum(serialize = "Firepower Enhancement")]
    FirepowerEnhancement = 2300100,
    #[strum(serialize = "Enhanced Weapon")]
    EnhancedWeapon = 2290000,
    #[strum(serialize = "Pistoleer")]
    Pistoleer = 2290100,
    #[strum(serialize = "Death Strike")]
    DeathStrike = 2280000,
    #[strum(serialize = "Loyal Companion")]
    LoyalCompanion = 2280100,
    #[strum(serialize = "Evolutionary Legacy")]
    EvolutionaryLegacy = 2350000,
    #[strum(serialize = "Arthetinean Skill")]
    ArthetineanSkill = 2350100,
    #[strum(serialize = "Peacemaker")]
    Peacemaker = 2380000,
    #[strum(serialize = "Time to Hunt")]
    TimeToHunt = 2380100,
    #[strum(serialize = "Igniter")]
    Igniter = 2370000,
    #[strum(serialize = "Reflux")]
    Reflux = 2370100,
    #[strum(serialize = "Grace of the Empress")]
    GraceOfTheEmpress = 2190000,
    #[strum(serialize = "Order of the Emperor")]
    OrderOfTheEmperor = 2190100,
    #[strum(serialize = "Communication Overflow")]
    CommunicationOverflow = 2200000,
    #[strum(serialize = "Master Summoner")]
    MasterSummoner = 2200100,
    #[strum(serialize = "Desperate Salvation")]
    DesperateSalvation = 2210000,
    #[strum(serialize = "True Courage")]
    TrueCourage = 2210100,
    #[strum(serialize = "Demonic Impulse")]
    DemonicImpulse = 2270000,
    #[strum(serialize = "Perfect Suppression")]
    PerfectSuppression = 2270600,
    #[strum(serialize = "Surge")]
    Surge = 2250000,
    #[strum(serialize = "Remaining Energy")]
    RemainingEnergy = 2250600,
    #[strum(serialize = "Lunar Voice")]
    LunarVoice = 2260000,
    #[strum(serialize = "Hunger")]
    Hunger = 2260600,
    #[strum(serialize = "Full Moon Harvester")]
    FullMoonHarvester = 2460000,
    #[strum(serialize = "Night's Edge")]
    NightsEdge = 2460600,
    #[strum(serialize = "Wind Fury")]
    WindFury = 2320000,
    #[strum(serialize = "Drizzle")]
    Drizzle = 2320600,
    #[strum(serialize = "Full Bloom")]
    FullBloom = 2310000,
    #[strum(serialize = "Recurrence")]
    Recurrence = 2310600,
}
