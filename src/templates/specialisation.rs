use crate::{templates::{skill::*, skill_buff::{DPS_CRIT_SWIFT_PET_BUFFS, DRAGON_ENGRAVING_319504, SUP_PET_BUFFS}}, types::*};

impl ClassWithSpecialisation {
    pub fn SUPPORT() -> &'static [ClassWithSpecialisation] {
        &[
            ClassWithSpecialisation::Bard(BardSpecialisation::DesperateSalvation),
            ClassWithSpecialisation::Paladin(PaladinSpecialisation::BlessedAura),
            ClassWithSpecialisation::Artist(ArtistSpecialisation::FullBloom),
            ClassWithSpecialisation::Valkyrie(ValkyrieSpecialisation::Liberator),
        ]
    }

    pub fn DPS() -> &'static [ClassWithSpecialisation] {
        &[
            ClassWithSpecialisation::Bard(BardSpecialisation::TrueCourage),
            ClassWithSpecialisation::Paladin(PaladinSpecialisation::Judgement),
            ClassWithSpecialisation::Artist(ArtistSpecialisation::Recurrence),
            ClassWithSpecialisation::Valkyrie(ValkyrieSpecialisation::ShiningKnight),
            ClassWithSpecialisation::Berserker(BerserkerSpecialisation::BerserkerTechnique),
            ClassWithSpecialisation::Berserker(BerserkerSpecialisation::Mayhem),
            ClassWithSpecialisation::Destroyer(DestroyerSpecialisation::RageHammer),
            ClassWithSpecialisation::Destroyer(DestroyerSpecialisation::GravityTraining),
            ClassWithSpecialisation::Gunlancer(GunlancerSpecialisation::LoneKnight),
            ClassWithSpecialisation::Gunlancer(GunlancerSpecialisation::CombatReadiness),
            ClassWithSpecialisation::Slayer(SlayerSpecialisation::Punisher),
            ClassWithSpecialisation::Slayer(SlayerSpecialisation::Predator),
            ClassWithSpecialisation::Arcanist(ArcanaSpecialisation::GraceOfTheEmpress),
            ClassWithSpecialisation::Arcanist(ArcanaSpecialisation::OrderOfTheEmperor),
            ClassWithSpecialisation::Summoner(SummonerSpecialisation::CommunicationOverflow),
            ClassWithSpecialisation::Summoner(SummonerSpecialisation::MasterSummoner),
            ClassWithSpecialisation::Sorceress(SorceressSpecialisation::Igniter),
            ClassWithSpecialisation::Sorceress(SorceressSpecialisation::Reflux),
            ClassWithSpecialisation::Wardancer(WardancerSpecialisation::FirstIntention),
            ClassWithSpecialisation::Wardancer(WardancerSpecialisation::EsotericSkillEnhancement),
            ClassWithSpecialisation::Scrapper(ScrapperSpecialisation::UltimateSkillTaijutsu),
            ClassWithSpecialisation::Scrapper(ScrapperSpecialisation::ShockTraining),
            ClassWithSpecialisation::Soulfist(SoulfistSpecialisation::EnergyOverflow),
            ClassWithSpecialisation::Soulfist(SoulfistSpecialisation::RobustSpirit),
            ClassWithSpecialisation::Glaivier(GlaivierSpecialisation::Control),
            ClassWithSpecialisation::Glaivier(GlaivierSpecialisation::Pinnacle),
            ClassWithSpecialisation::Striker(StrikerSpecialisation::EsotericFlurry),
            ClassWithSpecialisation::Striker(StrikerSpecialisation::Deathblow),
            ClassWithSpecialisation::Breaker(BreakerSpecialisation::BrawlKingStorm),
            ClassWithSpecialisation::Breaker(BreakerSpecialisation::AsurasPath),
            ClassWithSpecialisation::Deathblade(DeatbladeSpecialisation::Surge),
            ClassWithSpecialisation::Deathblade(DeatbladeSpecialisation::RemainingEnergy),
            ClassWithSpecialisation::Shadowhunter(ShadowhunterSpecialisation::DemonicImpulse),
            ClassWithSpecialisation::Shadowhunter(ShadowhunterSpecialisation::PerfectSuppression),
            ClassWithSpecialisation::Reaper(ReaperSpecialisation::LunarVoice),
            ClassWithSpecialisation::Reaper(ReaperSpecialisation::Hunger),
            ClassWithSpecialisation::Souleater(SouleaterSpecialisation::FullMoonHarvester),
            ClassWithSpecialisation::Souleater(SouleaterSpecialisation::NightsEdge),
            ClassWithSpecialisation::Sharpshooter(SharpshooterSpecialisation::DeathStrike),
            ClassWithSpecialisation::Sharpshooter(SharpshooterSpecialisation::LoyalCompanion),
            ClassWithSpecialisation::Deadeye(DeadeyeSpecialisation::EnhancedWeapon),
            ClassWithSpecialisation::Deadeye(DeadeyeSpecialisation::Pistoleer),
            ClassWithSpecialisation::Artillerist(ArtilleristSpecialisation::BarrageEnhancement),
            ClassWithSpecialisation::Artillerist(ArtilleristSpecialisation::FirepowerEnhancement),
            ClassWithSpecialisation::Machinist(ScouterSpecialisation::EvolutionaryLegacy),
            ClassWithSpecialisation::Machinist(ScouterSpecialisation::ArthetineanSkill),
            ClassWithSpecialisation::Gunslinger(GunslingerSpecialisation::Peacemaker),
            ClassWithSpecialisation::Gunslinger(GunslingerSpecialisation::TimeToHunt),
            ClassWithSpecialisation::Aeromancer(AeromancerSpecialisation::WindFury),
            ClassWithSpecialisation::Aeromancer(AeromancerSpecialisation::Drizzle),
            ClassWithSpecialisation::Wildsoul(WildsoulSpecialisation::PhantomBeastAwakening),
            ClassWithSpecialisation::Wildsoul(WildsoulSpecialisation::Ferality),
        ]
    }

    pub fn to_tuple(&self) -> (Class, Specialisation) {
        unsafe {
            match self {
                ClassWithSpecialisation::Berserker(spec) => {
                    (Class::Berserker, std::mem::transmute::<BerserkerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Destroyer(spec) => {
                    (Class::Destroyer, std::mem::transmute::<DestroyerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Gunlancer(spec) => {
                    (Class::Gunlancer, std::mem::transmute::<GunlancerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Paladin(spec) => {
                    (Class::Paladin, std::mem::transmute::<PaladinSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Slayer(spec) => {
                    (Class::Slayer, std::mem::transmute::<SlayerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Valkyrie(spec) => {
                    (Class::Valkyrie, std::mem::transmute::<ValkyrieSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Arcanist(spec) => {
                    (Class::Arcanist, std::mem::transmute::<ArcanaSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Summoner(spec) => {
                    (Class::Summoner, std::mem::transmute::<SummonerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Bard(spec) => {
                    (Class::Bard, std::mem::transmute::<BardSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Sorceress(spec) => {
                    (Class::Sorceress, std::mem::transmute::<SorceressSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Wardancer(spec) => {
                    (Class::Wardancer, std::mem::transmute::<WardancerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Scrapper(spec) => {
                    (Class::Scrapper, std::mem::transmute::<ScrapperSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Soulfist(spec) => {
                    (Class::Soulfist, std::mem::transmute::<SoulfistSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Glaivier(spec) => {
                    (Class::Glaivier, std::mem::transmute::<GlaivierSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Striker(spec) => {
                    (Class::Striker, std::mem::transmute::<StrikerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Breaker(spec) => {
                    (Class::Breaker, std::mem::transmute::<BreakerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Deathblade(spec) => {
                    (Class::Deathblade, std::mem::transmute::<DeatbladeSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Shadowhunter(spec) => {
                    (Class::Shadowhunter, std::mem::transmute::<ShadowhunterSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Reaper(spec) => {
                    (Class::Reaper, std::mem::transmute::<ReaperSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Souleater(spec) => {
                    (Class::Souleater, std::mem::transmute::<SouleaterSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Sharpshooter(spec) => {
                    (Class::Sharpshooter, std::mem::transmute::<SharpshooterSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Deadeye(spec) => {
                    (Class::Deadeye, std::mem::transmute::<DeadeyeSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Artillerist(spec) => {
                    (Class::Artillerist, std::mem::transmute::<ArtilleristSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Machinist(spec) => {
                    (Class::Machinist, std::mem::transmute::<ScouterSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Gunslinger(spec) => {
                    (Class::Gunslinger, std::mem::transmute::<GunslingerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Artist(spec) => {
                    (Class::Artist, std::mem::transmute::<ArtistSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Aeromancer(spec) => {
                    (Class::Aeromancer, std::mem::transmute::<AeromancerSpecialisation, Specialisation>(*spec))
                }
                ClassWithSpecialisation::Wildsoul(spec) => {
                    (Class::Wildsoul, std::mem::transmute::<WildsoulSpecialisation, Specialisation>(*spec))
                }
            }
        }
    }

    pub fn get_preset(&self) -> Preset {
        match self {
            ClassWithSpecialisation::Berserker(specialisation) => match specialisation {
                BerserkerSpecialisation::BerserkerTechnique => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Berserker)
                },
                BerserkerSpecialisation::Mayhem => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Berserker)
                }
            },
            ClassWithSpecialisation::Destroyer(specialisation) => match specialisation {
                DestroyerSpecialisation::GravityTraining => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Destroyer)
                },
                DestroyerSpecialisation::RageHammer => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Destroyer)
                },
            },
            ClassWithSpecialisation::Gunlancer(specialisation) => match specialisation {
                GunlancerSpecialisation::LoneKnight => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    awakening_skill: LANCE_OF_JUDGMENT,
                    hyper_awakening_skill: LANCE_OF_JUDGMENT,
                    hyper_awakening_technique_skill: SPEAR_DASH,
                    is_support: false,
                    skills: vec![
                        SHOUT_OF_HATRED,
                        CHARGED_STINGER,
                        SURGE_CANNON,
                        GUNLANCE_SHOT,
                        DASH_UPPER_FIRE,
                        BASH,
                        SHIELD_BASH,
                        COUNTER_GUNLANCE,
                        FIRE_BULLET,
                        RISING_GUNLANCE
                    ],
                    identity_skills: vec![],
                    status_effects: DPS_CRIT_SWIFT_PET_BUFFS.to_owned(),
                    transformation_skills: vec![]
                },
                GunlancerSpecialisation::CombatReadiness => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Gunlancer)
                },
            },
            ClassWithSpecialisation::Paladin(specialisation) => match specialisation {
                PaladinSpecialisation::Judgement => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Paladin)
                },
                PaladinSpecialisation::BlessedAura => Preset {
                    cooldown_reduction: 0.50,
                    crit_rate: 0.10,
                    is_support: true,
                    skills: vec![
                        HEAVENLY_BLESSINGS,
                        WRATH_OF_GOD,
                        HOLY_PROTECTION,
                        GODS_DECREE,
                        LIGHT_OF_JUDGMENT,
                        SWORD_OF_JUSTICE,
                        HOLY_AREA,
                    ],
                    hyper_awakening_technique_skill: DIVINE_JUSTICE,
                    awakening_skill: ALITHANESS_JUDGMENT,
                    hyper_awakening_skill: ALITHANESS_DEVOTION,
                    status_effects: SUP_PET_BUFFS.to_owned(),
                    identity_skills: vec![HOLY_AURA],
                    transformation_skills: vec![]
                },
            },
            ClassWithSpecialisation::Slayer(specialisation) => match specialisation {
                SlayerSpecialisation::Punisher => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Slayer)
                },
                SlayerSpecialisation::Predator => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Slayer)
                },
            },
            ClassWithSpecialisation::Valkyrie(specialisation) => match specialisation {
                ValkyrieSpecialisation::ShiningKnight => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Valkyrie)
                },
                ValkyrieSpecialisation::Liberator => Preset {
                    cooldown_reduction: 0.50,
                    crit_rate: 0.10,
                    is_support: true,
                    skills: vec![
                        SERAPHIC_LEAP,
                        SERAPHIC_OATH,
                        SWORD_OF_REVELATION,
                        REQUIEM_ASH,
                        CIRCLE_OF_TRUTH,
                        SALVATION_SITE,
                        EXECUTION_OF_REVELATION
                    ],
                    identity_skills: vec![],
                    hyper_awakening_technique_skill: DIVINE_CONFIRMATION,
                    awakening_skill: BRUNDIAS_RITUAL,
                    hyper_awakening_skill: BRUNDIAS_SANCTUARY,
                    ..Preset::default_for_class(Class::Valkyrie)
                },
            },
            ClassWithSpecialisation::Arcanist(specialisation) => match specialisation {
                ArcanaSpecialisation::GraceOfTheEmpress => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Arcanist)
                },
                ArcanaSpecialisation::OrderOfTheEmperor => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Arcanist)
                },
            },
            ClassWithSpecialisation::Summoner(specialisation) => match specialisation {
                SummonerSpecialisation::CommunicationOverflow => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Summoner)
                },
                SummonerSpecialisation::MasterSummoner => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Summoner)
                },
            },
            ClassWithSpecialisation::Bard(specialisation) => match specialisation {
                BardSpecialisation::DesperateSalvation => Preset {
                    cooldown_reduction: 0.50,
                    crit_rate: 0.10,
                    is_support: true,
                    skills: vec![
                        HEAVENLY_TUNE,
                        SONIC_VIBRATION,
                        GUARDIAN_TUNE,
                        WIND_OF_MUSIC_21079,
                        RHYTHM_BUCKSHOT,
                        SONATINA,
                        HARP_OF_RHYTHM,
                        PRELUDE_OF_STORM
                    ],
                    identity_skills: vec![
                        SERENADE_OF_COURAGE_21141,
                        SERENADE_OF_COURAGE_21142,
                        SERENADE_OF_COURAGE_21143,
                    ],
                    hyper_awakening_technique_skill: ARIA,
                    awakening_skill: SYMPHONIA,
                    hyper_awakening_skill: SYMPHONY_MELODY,
                    status_effects: SUP_PET_BUFFS.to_owned(),
                    transformation_skills: vec![]
                },
                BardSpecialisation::TrueCourage => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Bard)
                },
            },
            ClassWithSpecialisation::Sorceress(specialisation) => match specialisation {
                SorceressSpecialisation::Igniter => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Sorceress)
                },
                SorceressSpecialisation::Reflux => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Sorceress)
                },
            },
            ClassWithSpecialisation::Wardancer(specialisation) => match specialisation {
                WardancerSpecialisation::FirstIntention => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Wardancer)
                },
                WardancerSpecialisation::EsotericSkillEnhancement => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Wardancer)
                },
            }
            ClassWithSpecialisation::Scrapper(specialisation) => match specialisation {
                ScrapperSpecialisation::UltimateSkillTaijutsu => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Scrapper)
                },
                ScrapperSpecialisation::ShockTraining => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Scrapper)
                },
            },
            ClassWithSpecialisation::Soulfist(specialisation) => match specialisation {
                SoulfistSpecialisation::EnergyOverflow => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Soulfist)
                },
                SoulfistSpecialisation::RobustSpirit => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Soulfist)
                },
            },
            ClassWithSpecialisation::Glaivier(specialisation) => match specialisation {
                GlaivierSpecialisation::Control => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Glaivier)
                },
                GlaivierSpecialisation::Pinnacle => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Glaivier)
                },
            },
            ClassWithSpecialisation::Striker(specialisation) => match specialisation {
                StrikerSpecialisation::EsotericFlurry => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Striker)
                },
                StrikerSpecialisation::Deathblow => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Striker)
                },
            },
            ClassWithSpecialisation::Breaker(specialisation) => match specialisation {
                BreakerSpecialisation::BrawlKingStorm => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Breaker)
                },
                BreakerSpecialisation::AsurasPath => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Breaker)
                },
            },
            ClassWithSpecialisation::Deathblade(specialisation) => match specialisation {
                DeatbladeSpecialisation::Surge | DeatbladeSpecialisation::RemainingEnergy => {
                    Preset {
                        cooldown_reduction: 0.18,
                        crit_rate: 0.75,
                        ..Preset::default_for_class(Class::Deathblade)
                    }
                }
            },
            ClassWithSpecialisation::Shadowhunter(specialisation) => match specialisation {
                ShadowhunterSpecialisation::DemonicImpulse => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Shadowhunter)
                },
                ShadowhunterSpecialisation::PerfectSuppression => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Shadowhunter)
                },
            },
            ClassWithSpecialisation::Reaper(specialisation) => match specialisation {
                ReaperSpecialisation::LunarVoice => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Reaper)
                },
                ReaperSpecialisation::Hunger => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Reaper)
                },
            },
            ClassWithSpecialisation::Souleater(specialisation) => match specialisation {
                SouleaterSpecialisation::FullMoonHarvester => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Souleater)
                },
                SouleaterSpecialisation::NightsEdge => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Souleater)
                },
            },
            ClassWithSpecialisation::Sharpshooter(specialisation) => match specialisation {
                SharpshooterSpecialisation::DeathStrike => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Sharpshooter)
                },
                SharpshooterSpecialisation::LoyalCompanion => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Sharpshooter)
                },
            },
            ClassWithSpecialisation::Deadeye(specialisation) => match specialisation {
                DeadeyeSpecialisation::EnhancedWeapon => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Deadeye)
                },
                DeadeyeSpecialisation::Pistoleer => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Deadeye)
                },
            },
            ClassWithSpecialisation::Artillerist(specialisation) => match specialisation {
                ArtilleristSpecialisation::BarrageEnhancement => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Artillerist)
                },
                ArtilleristSpecialisation::FirepowerEnhancement => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Artillerist)
                },
            },
            ClassWithSpecialisation::Machinist(specialisation) => match specialisation {
                ScouterSpecialisation::EvolutionaryLegacy => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Machinist)
                },
                ScouterSpecialisation::ArthetineanSkill => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Machinist)
                },
            },
            ClassWithSpecialisation::Gunslinger(specialisation) => match specialisation {
                GunslingerSpecialisation::Peacemaker => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Gunslinger)
                },
                GunslingerSpecialisation::TimeToHunt => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Gunslinger)
                },
            },
            ClassWithSpecialisation::Artist(specialisation) => match specialisation {
                ArtistSpecialisation::FullBloom => Preset {
                    cooldown_reduction: 0.50,
                    crit_rate: 0.10,
                    is_support: true,
                    skills: vec![
                        PAINT_SUNSKETCH,
                        PAINT_SUN_WELL,
                        PAINT_DRAWING_ORCHIDS,
                        STROKE_HOPPER,
                        PAINT_ILLUSION_DOOR,
                        PAINT_INK_WELL,
                        PAINT_STARRY_NIGHT,
                        STROKE_UPWARD_STROKE
                    ],
                    hyper_awakening_technique_skill: PAINT_DRAGON_ENGRAVING,
                    awakening_skill: MASTERWORK_EFFLORESCENCE,
                    hyper_awakening_skill: DREAM_BLOSSOM_GARDEN,
                    identity_skills: vec![MOONFALL_31051],
                    status_effects: SUP_PET_BUFFS.to_owned(),
                    transformation_skills: vec![]
                },
                ArtistSpecialisation::Recurrence => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Artist)
                },
            },
            ClassWithSpecialisation::Aeromancer(specialisation) => match specialisation {
                AeromancerSpecialisation::WindFury => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Aeromancer)
                },
                AeromancerSpecialisation::Drizzle => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Aeromancer)
                }
            },
            ClassWithSpecialisation::Wildsoul(specialisation) => match specialisation {
                WildsoulSpecialisation::PhantomBeastAwakening => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    ..Preset::default_for_class(Class::Wildsoul)
                },
                WildsoulSpecialisation::Ferality => Preset {
                    cooldown_reduction: 0.18,
                    crit_rate: 0.75,
                    skills: vec![
                        BOULDER_BEAR_33231,
                        FOX_ILLUSION_33310,
                        FOX_FLAME_33320,
                        FOX_ORB,
                        TAILWIND,
                        VULPINE_VELOCITY,
                        CLAW,
                        SWISH_BEAR_33200
                    ],
                    hyper_awakening_technique_skill: ONE_HIT_BEAR,
                    awakening_skill: EARTHQUAKE_POUND,
                    hyper_awakening_skill: SMACK_SMITE,
                    ..Preset::default_for_class(Class::Wildsoul)
                }
            },
        }
    }

}
