#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use meter_core_fake::packets::{definitions::{TripodIndex, TripodLevel}, structures::StatusEffectData};
use rand::Rng;

use crate::{simulator::{identity::IdentityGenerator, utils::make_hp_stat_pairs}, templates::{mordum_g3, skill::ALL_COUNTER_SKILL_IDS, RaidTemplate, RaidTemplateNpc}, types::*};

#[derive(Debug, Clone)]
pub struct EncounterBuilderArgs {
    pub template: RaidTemplate,
    
    pub sidereal_frequency: u64,
    pub use_vehicle_packet_instead_of_spawn: bool,
    pub supports_raid_begin_packet: bool,
    pub supports_zone_member_load_packet: bool,
    pub invoke_zone_member_load: Option<(u32, u32)>,
    pub supports_npc_death_packet: bool,
    pub invoke_boss_kill_packet: bool,
    pub invoke_raid_result_packet: bool,
    pub invoke_zone_change_packet: bool,
    pub invoke_trigger_start_packet: Option<u32>,
    pub boss_attack_chance: f64,
    pub boss_knock_chance: f64,
    pub boss_min_damage: i64,
    pub boss_max_damage: i64,
    pub boss_skill_ids: Vec<u32>,
    pub local_player_index: u8,
    pub min_attack_power: i64,
    pub max_attack_power: i64,
    pub dps_multiplier: f64,
    pub sup_multiplier: f64,
    pub dps_skill_damage_ratio: f64,
    pub sup_skill_damage_ratio: f64,
    pub initial_hyper_awakening_cooldown: u64,
    pub default_player_hp: i64,
    pub default_crit_damage: f64,
    pub max_ticks: u64,
    pub allowed_skill_cast_count: u32,
    pub counter_skill_ids: HashSet<u32>,
    pub new_transit_packet: Option<u32>,
    pub player_configuration_map: HashMap<u8, PlayerConfiguration>,
}

#[derive(Debug, Clone)]
pub struct PlayerConfiguration {
    pub dies_at: u64,
    pub class_with_specialisation: ClassWithSpecialisation,
    pub hp: i64,
    pub skill_tripod_map: HashMap<u32, (TripodIndex, TripodLevel)>
}

impl Default for EncounterBuilderArgs {
    fn default() -> Self {
        Self {
            template: mordum_g3(),
            dps_multiplier: 1.2,
            sup_multiplier: 1.0,
            dps_skill_damage_ratio: 10.0,
            sup_skill_damage_ratio: 1.0,
            min_attack_power: 100_000,
            max_attack_power: 130_000,
            default_crit_damage: 1.5,
            sidereal_frequency: 100,
            use_vehicle_packet_instead_of_spawn: false,
            supports_raid_begin_packet: true,
            supports_zone_member_load_packet: true,
            invoke_zone_member_load: None,
            supports_npc_death_packet: false,
            invoke_boss_kill_packet: true,
            invoke_raid_result_packet: true,
            invoke_zone_change_packet: false,
            invoke_trigger_start_packet: None,
            boss_attack_chance: 0.1,
            boss_knock_chance: 0.25,
            boss_min_damage: 10_00,
            boss_max_damage: 50_000,
            boss_skill_ids: vec![10000],
            default_player_hp: 300_000,
            local_player_index: 0,
            max_ticks: 10000,
            allowed_skill_cast_count: 4,
            initial_hyper_awakening_cooldown: 300,
            counter_skill_ids: ALL_COUNTER_SKILL_IDS.iter().cloned().collect(),
            new_transit_packet: None,
            player_configuration_map: HashMap::new(),
        }
    }
}

pub struct EncounterBuilder {
    args: EncounterBuilderArgs,
    generator: IdentityGenerator,
    parties: Vec<Party>,
    bosses: Vec<Boss>,
    esther: Vec<Esther>,
}

impl EncounterBuilder {
    pub fn new(args: EncounterBuilderArgs) -> Self {
        Self {
            args,
            parties: vec![],
            bosses: vec![],
            esther: vec![],
            generator: IdentityGenerator::new(),
        }
    }

    pub fn build_skill(
        args: &EncounterBuilderArgs,
        skill: Skill,
        is_support: bool,
        skill_tripod_map: &HashMap<u32, (TripodIndex, TripodLevel)>
    ) -> SkillInstance {
        let initial_cooldown = match skill.kind {
            SkillType::HyperAwakening | SkillType::Awakening => args.initial_hyper_awakening_cooldown,
            _ => 0,
        };
        
        let mut skill_instance = SkillInstance {
            id: skill.id,
            is_counter: skill.is_counter,
            cooldown: skill.cooldown,
            initial_cooldown: 0,
            cooldown_ends_at: 0,
            hit_option: PreferredHitOption::Any,
            ..Default::default()
        };

        if matches!(skill_instance.cooldown, 0 | 1 | 2) {
            skill_instance.cooldown = 10;
        }

        skill_instance.cooldown_ends_at = initial_cooldown;

        if args.counter_skill_ids.contains(&skill_instance.id) {
            skill_instance.is_counter = true;
        }

        if let Some((index, level)) = skill_tripod_map.get(&skill_instance.id) {
            skill_instance.tripod_index = Some(index.clone());
            skill_instance.tripod_level = Some(level.clone());
        }

        let damage_ratio = if is_support { 
            args.sup_skill_damage_ratio
        } else {
            args.dps_skill_damage_ratio
        };

        skill_instance.effects.push(SkillEffect::Damage { damage_ratio });

        if let Some(buff) = skill.skill_buff {
            skill_instance.effects.push(SkillEffect::Buff { 
                effect_id: buff.id,
                unique_group: buff.unique_group,
                is_party: buff.is_party,
                duration: buff.duration,
                buff_type: buff.buff_type
            });
        }

        if let Some(npc_id) = skill.summon {
            skill_instance.effects.push(SkillEffect::Summon {
                duration: 5,
                skill_id: skill.id,
                npc_id: npc_id,
                damage_ratio: 1.0
            });
        }

        if skill.is_trap {
            skill_instance.effects.push(SkillEffect::Trap { duration: 5, skill_id: skill.id, damage_ratio: 1.0 });
        }

        if skill.is_projectile {
            skill_instance.effects.push(SkillEffect::Projectile { damage_ratio: 1.0 });
        }

        skill_instance
    }

    pub fn build_player(&mut self, configuration: PlayerConfiguration) -> Player {
        let PlayerConfiguration {
            class_with_specialisation,
            dies_at,
            hp,
            skill_tripod_map,
            ..
        } = configuration;

        let preset = class_with_specialisation.get_preset();
        let is_support = preset.is_support;
        let (class, specialisation) = class_with_specialisation.to_tuple();

        let attack_power = self.generator.random_range(self.args.min_attack_power..self.args.max_attack_power);

        let multiplier = if is_support {
            self.args.sup_multiplier
        } else {
            self.args.dps_multiplier
        };

        let mut skill_instances: Vec<SkillInstance> = vec![];
        let mut skills: Vec<Skill> = preset.skills.iter().take(8).cloned().collect();
        
        if let Some(identity_skill) = preset.identity_skills.first() {
            skills.push(identity_skill.clone());
        }
        
        skills.push(preset.hyper_awakening_technique_skill);
        skills.push(preset.hyper_awakening_skill);

        for skill in skills {
            let skill_instance = Self::build_skill(
                &self.args,
                skill,
                is_support,
                &skill_tripod_map);
            skill_instances.push(skill_instance);
        }

        let id = self.generator.new_u64();

        let status_effects = preset.status_effects.into_iter().map(|pr| {
            let status_effect_instance_id = self.generator.new_u32();

            StatusEffectData {
                status_effect_id: pr.id,
                total_time: pr.duration as f32,
                source_id: id,
                status_effect_instance_id,
                ..Default::default()
            }
        }).collect();

        Player {
            id,
            is_support,
            is_dead: false,
            dies_at,
            is_local: false,
            incapacitate_time: 0,
            name: self.generator.random_nickname(),
            class,
            specialisation,
            gear_level: 1700.0,
            stats: make_hp_stat_pairs(hp),
            cur_hp: hp,
            max_hp: hp,
            status_effects,
            attack_power,
            cooldown_reduction: preset.cooldown_reduction,
            crit_rate: preset.crit_rate,
            crit_damage: self.args.default_crit_damage,
            multiplier,
            skills: skill_instances,
            buffs: HashMap::new(),
            summons: HashMap::new(),
            traps: HashMap::new(),
        }
    }

    pub fn build_boss(&mut self, template: RaidTemplateNpc) -> Boss {
        Boss {
            id: self.generator.new_u64(),
            type_id: template.npc_id,
            stats: make_hp_stat_pairs(template.hp),
            cur_hp: template.hp,
            max_hp: template.hp,
            min_damage: self.args.boss_min_damage,
            max_damage: self.args.boss_max_damage,
            debuffs: HashMap::new(),
            skill_ids: self.args.boss_skill_ids.clone()
        }
    }

    pub fn build_default_party(&mut self, party_index: u8) -> Party {
        let mut members = Vec::with_capacity(4);

        for member_index in 0..4 {
            let index = (party_index * 4 + member_index) as u8; 
            if let Some(configuration) = self.args.player_configuration_map.get(&index) {
                let player = self.build_player(configuration.clone());
                members.push(player);
            }
            else {
                let class_with_specialisation = if member_index < 3 {
                    self.generator.random_dps()
                } else {
                    self.generator.random_sup()
                };
                let configuration = PlayerConfiguration { 
                    dies_at: 0,
                    class_with_specialisation,
                    hp: self.args.default_player_hp,
                    skill_tripod_map: HashMap::new()
                };
                let player = self.build_player(configuration);
                members.push(player);
            }
        } 

        let id = self.generator.new_u32();

        Party { id, members }
    }

    pub fn build(mut self) -> Encounter {

        if self.parties.is_empty() {
            for index in 0..self.args.template.party_count {
                let party = self.build_default_party(index);
                self.parties.push(party);
            }
        }

        let local_player_id = self
            .parties
            .iter_mut()
            .flat_map(|p| p.members.iter_mut())
            .nth(self.args.local_player_index as usize)
            .map(|p| {
                p.is_local = true;
                p.id
            }).unwrap();

        self.bosses = self
            .args
            .template
            .npcs
            .clone()
            .into_iter()
            .map(|template| self.build_boss(template))
            .collect();

        Encounter {
            raid_instance_id: self.generator.new_u32(),
            max_ticks: self.args.max_ticks,
            local_player_id,
            zone_id: self.args.template.zone_id,
            zone_level: self.args.template.zone_level,
            parties: self.parties,
            bosses: self.bosses,
            esther: self.esther,
            use_vehicle_packet_instead_of_spawn: self.args.use_vehicle_packet_instead_of_spawn,
            sidereal_frequency: self.args.sidereal_frequency,
            supports_raid_begin_packet: self.args.supports_raid_begin_packet,
            supports_zone_member_load_packet: self.args.supports_zone_member_load_packet,
            invoke_zone_member_load: self.args.invoke_zone_member_load,
            supports_npc_death_packet: self.args.supports_npc_death_packet,
            invoke_boss_kill_packet: self.args.invoke_boss_kill_packet,
            invoke_raid_result_packet: self.args.invoke_raid_result_packet,
            invoke_zone_change_packet: self.args.invoke_zone_change_packet,
            invoke_trigger_start_packet: self.args.invoke_trigger_start_packet,
            boss_attack_chance: self.args.boss_attack_chance,
            boss_knock_chance: self.args.boss_knock_chance,
            new_transit_packet: self.args.new_transit_packet,
            allowed_skill_cast_count: self.args.allowed_skill_cast_count
        }
    }
}