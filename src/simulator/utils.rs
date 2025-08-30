use std::collections::HashMap;

use meter_core_fake::packets::structures::*;
use rand::{rngs::ThreadRng, seq::IndexedRandom, Rng};

use crate::{simulator::identity::IdentityGenerator, types::*};

pub fn make_hp_stat_pairs(hp: i64) -> Vec<StatPair> {
    let mut result = Vec::new();

    result.push(StatPair {
        stat_type: 1, // current HP
        value: hp,
    });

    result.push(StatPair {
        stat_type: 27, // max HP
        value: hp,
    });

    result
}

fn status_effect(id: u32, instance_id: u32) -> StatusEffectData {
    StatusEffectData {
        source_id: 2,
        end_tick: 0,
        stack_count: 0,
        status_effect_id: id,
        status_effect_instance_id: instance_id,
        total_time: 10_000.0,
        value: StatusEffectDataValue { bytearray_0: None },
    }
}

pub fn make_status_effects(generator: &mut IdentityGenerator, effect_ids: &[u32]) -> Vec<StatusEffectData> {
    effect_ids
        .iter()
        .map(|&id| status_effect(id, generator.new_u32()))
        .collect()
}

pub fn make_default_dps_crit_status_effects(generator: &mut IdentityGenerator) -> Vec<StatusEffectData> {
    make_status_effects(generator, &[20004, 21004])
}

pub fn make_default_dps_spec_status_effects(generator: &mut IdentityGenerator) -> Vec<StatusEffectData> {
    make_status_effects(generator, &[20004, 21104])
}

pub fn make_default_sup_status_effects(generator: &mut IdentityGenerator) -> Vec<StatusEffectData> {
    make_status_effects(generator, &[20004, 21304])
}

pub fn encode_modifier(hit_flag: HitFlag, hit_option: HitOption) -> i32 {
    let flag_bits = if hit_flag == HitFlag::Unknown {
        15u8
    } else {
        hit_flag as u8
    };

    let option_bits = if (hit_option as u8) >= 4 {
        0u8
    } else {
        hit_option as u8
    };

    let value = (option_bits << 4) | flag_bits;

    value as i32
}

pub fn random_hit_option(rng: &mut ThreadRng) -> HitOption {
    let options = vec![
        HitOption::Max,
        HitOption::FlankAttack,
        HitOption::FrontalAttack,
        HitOption::BackAttack,
        HitOption::None,
    ];

    *options.choose(rng).unwrap()
}

pub fn calculate_damage(
    rng: &mut ThreadRng,
    player: &Player,
    damage_ratio: f64,
    boss_debuffs: &HashMap<u32, Buff>) -> (i64, HitFlag, HitOption) {
    let damage_ratio = damage_ratio;
    let mut multiplier = player.multiplier;
    let mut attack_power = player.attack_power as f64;
    let is_crit = rng.random_bool(player.crit_rate);
    let mut hit_flag = HitFlag::Normal;
    let hit_option = random_hit_option(rng);

    for (id, debuff) in boss_debuffs {
        match debuff.buff_type {
            BuffType::Brand(brand_multiplier) => {
                    multiplier += brand_multiplier;
            },
            _ => unimplemented!()
        }
    }

    for (id, buff) in &player.buffs {
        match buff.buff_type {
            BuffType::AttackPower(buff_attack_power) => {
                attack_power += buff_attack_power;
            },
            BuffType::Damage(buff_multiplier) => {
                multiplier += buff_multiplier;
            },
            BuffType::Brand(_) => unimplemented!(),
            _ => {}
        }
    }

    let mut damage = damage_ratio * attack_power * multiplier;

    if is_crit {
        damage *= player.crit_damage;
        hit_flag = HitFlag::Critical;
    }

    (damage as i64, hit_flag, hit_option)
}

pub fn calculate_player_damage(
    rng: &mut ThreadRng,
    player: &Player,
    damage_ratio: f64,
    preferred_hit_option: PreferredHitOption,
    boss_debuffs: &HashMap<u32, Buff>,
) -> (i64, HitFlag, HitOption) {

    let damage_ratio = damage_ratio;
    let mut multiplier = player.multiplier;
    let mut attack_power = player.attack_power as f64;
    let is_crit = rng.random_bool(player.crit_rate);
    let mut hit_flag = HitFlag::Normal;
    let hit_option = random_hit_option(rng);

    if hit_option as u8 != PreferredHitOption::Any as u8 {
        if preferred_hit_option as u8 == hit_option as u8 {
            multiplier += 0.1;
        }
    }

    for (id, debuff) in boss_debuffs {
        match debuff.buff_type {
            BuffType::Brand(brand_multiplier) => {
                    multiplier += brand_multiplier;
            },
            _ => unimplemented!()
        }
    }

    for (id, buff) in &player.buffs {
        match buff.buff_type {
            BuffType::AttackPower(buff_attack_power) => {
                attack_power += buff_attack_power;
            },
            BuffType::Damage(buff_multiplier) => {
                multiplier += buff_multiplier;
            },
            BuffType::Brand(_) => unimplemented!(),
            _ => {}
        }
    }

    let mut damage = damage_ratio * attack_power * multiplier;

    if is_crit {
        damage *= player.crit_damage;
        hit_flag = HitFlag::Critical;
    }

    (damage as i64, hit_flag, hit_option)
}

