use anyhow::Result;
use meter_core_fake::packets::{common::SkillMoveOptionData, definitions::*, opcodes::Pkt, structures::*};
use crate::{simulator::utils::{encode_modifier, make_hp_stat_pairs}, types::{EncounterEvent, Player}};

impl EncounterEvent {
    pub fn to_packet(self) -> Result<(Pkt, Vec<u8>)> {
        match self {
            EncounterEvent::NewTransit(zone_instance_id) => {
                let packet = PKTNewTransit {
                    zone_instance_id
                };
                Ok((Pkt::NewTransit, packet.encode()?))
            }
            EncounterEvent::Death(target_id) => {
                let packet = PKTDeathNotify {
                    target_id
                };
                Ok((Pkt::DeathNotify, packet.encode()?))
            }
            EncounterEvent::NewVehicle(player) => {
                let packet = PKTNewVehicle {
                    vehicle_struct: PKTNewVehicleInner { sub_p_k_t_new_vehicle_2_2_397: PKTNewVehicleInnerInner { 
                        p_c_struct: Some(PCStruct { 
                            player_id: player.id,
                            name: player.name,
                            character_id: player.id,
                            class_id: player.class.into(),
                            gear_level: player.gear_level,
                            stat_pairs: player.stats,
                            max_item_level: player.gear_level,
                            status_effect_datas: player.status_effects
                        })
                     } },
                };
                Ok((Pkt::NewVehicle, packet.encode()?))
            }
            EncounterEvent::RaidResult => {
                Ok((Pkt::RaidResult, vec![]))
            }
            EncounterEvent::RaidBossKill => {
                Ok((Pkt::RaidBossKillNotify, vec![]))
            }
            EncounterEvent::Trigger(signal) => {
                let packet = PKTTriggerStartNotify {
                    signal
                };
                Ok((Pkt::TriggerStartNotify, packet.encode()?))
            }
            EncounterEvent::RaidBegin(raid_id) => {
                let packet = PKTRaidBegin {
                    raid_id
                };
                Ok((Pkt::InitEnv, packet.encode()?))
            }
            EncounterEvent::ZoneMemberLoad { zone_id, zone_level } => {
                let packet = PKTZoneMemberLoadStatusNotify {
                    zone_id,
                    zone_level
                };
                Ok((Pkt::InitEnv, packet.encode()?))
            }
            EncounterEvent::ZoneChange(player_id) => {
                let packet = PKTInitEnv {
                    player_id
                };
                Ok((Pkt::InitEnv, packet.encode()?))
            }
            EncounterEvent::Npc {
                id,
                npc_id,
                hp
            } => {
                let packet = PKTNewNpc {
                    npc_struct: NpcStruct { 
                        object_id: id,
                        type_id: npc_id,
                        level: 60,
                        balance_level: NpcStructBalance { value: None },
                        stat_pairs: make_hp_stat_pairs(hp),
                        status_effect_datas: vec![]
                    }
                };
                Ok((Pkt::NewNpc, packet.encode()?))
            }
            EncounterEvent::AbnormalDamage {
                cur_hp,
                damage,
                hit_flag,
                hit_option,
                max_hp,
                skill_id,
                source_id,
                target_id,
                down_time,
                move_time,
                stand_up_time
            } => {
                let skill_damage_event = SkillDamageEvent {
                    target_id: target_id,
                    damage: damage,
                    cur_hp: cur_hp,
                    max_hp: max_hp,
                    modifier: encode_modifier(hit_flag, hit_option),
                    damage_attr: None,
                    damage_type: 0,
                    shield_damage: SkillDamageEventShield { p64_0: Some(0) },
                };

                let skill_move_option_data = SkillMoveOptionData {
                    down_time,
                    stand_up_time,
                    move_time
                };

                let skill_packet = PKTSkillDamageAbnormalMoveNotify {
                    source_id: source_id,
                    skill_damage_abnormal_move_events: vec![
                        PKTSkillDamageAbnormalMoveNotifyInner {
                            skill_damage_event,
                            skill_move_option_data
                        }
                    ],
                    skill_id,
                    skill_effect_id: 0,
                };

                Ok((Pkt::SkillDamageNotify, skill_packet.encode()?))
            },
            EncounterEvent::Damage { 
                hit_flag,
                hit_option,
                source_id,
                target_id,
                skill_id,
                damage,
                cur_hp,
                max_hp } => {
                let skill_damage_event = SkillDamageEvent {
                    target_id: target_id,
                    damage: damage,
                    cur_hp: cur_hp,
                    max_hp: max_hp,
                    modifier: encode_modifier(hit_flag, hit_option),
                    damage_attr: None,
                    damage_type: 0,
                    shield_damage: SkillDamageEventShield { p64_0: Some(0) },
                };

                let skill_packet = PKTSkillDamageNotify {
                    source_id: source_id,
                    skill_damage_events: vec![skill_damage_event],
                    skill_id,
                    skill_effect_id: None,
                };

                Ok((Pkt::SkillDamageNotify, skill_packet.encode()?))
            }
            EncounterEvent::SpawnLocalPlayer(player) => {
                let pkt = PKTInitPC { 
                    player_id: player.id,
                    name: player.name,
                    character_id: player.id,
                    class_id: player.class as u32,
                    gear_level: player.gear_level,
                    stat_pairs: player.stats,
                    status_effect_datas: player.status_effects,
                };
                Ok((Pkt::InitPC, pkt.encode()?))
            }
            EncounterEvent::SpawnPlayer(player) => {
                let pc = PCStruct {
                    player_id: player.id,
                    name: player.name,
                    character_id: player.id,
                    class_id: player.class as u32,
                    gear_level: player.gear_level,
                    stat_pairs: player.stats,
                    max_item_level: player.gear_level,
                    status_effect_datas: player.status_effects,
                };
                let pkt = PKTNewPC { pc_struct: pc };
                Ok((Pkt::NewPC, pkt.encode()?))
            }
            EncounterEvent::PartyInfo {
                party_instance_id,
                raid_instance_id,
                members
            } => {
                let party_member_datas = members
                    .into_iter()
                    .map(|pr| PKTPartyInfoInner {
                        character_id: pr.id,
                        class_id: pr.class as u32,
                        gear_level: pr.gear_level,
                        name: pr.name
                    }).collect();
                let pkt = PKTPartyInfo {
                    party_instance_id,
                    raid_instance_id,
                    party_member_datas
                };
                Ok((Pkt::PartyInfo, pkt.encode()?))
            }
            EncounterEvent::SpawnBoss(boss) => {
                let npc = NpcStruct {
                    object_id: boss.id,
                    type_id: boss.type_id,
                    level: 0,
                    balance_level: NpcStructBalance { value: None },
                    stat_pairs: boss.stats,
                    status_effect_datas: vec![],
                };
                let pkt = PKTNewNpc { npc_struct: npc };
                Ok((Pkt::NewNpc, pkt.encode()?))
            }
            EncounterEvent::SkillCast { source_id, skill_id, .. } => {
                let pkt = PKTSkillCastNotify {
                    source_id: source_id,
                    skill_id: skill_id,
                };
                Ok((Pkt::SkillCastNotify, pkt.encode()?))
            }
            EncounterEvent::Counter(source_id) => {
                let pkt = PKTCounterAttackNotify {
                    source_id: source_id,
                };
                Ok((Pkt::CounterAttackNotify, pkt.encode()?))
            }
            EncounterEvent::SkillStart { source_id,
                skill_id,
                tripod_index,
                tripod_level } => {
                let pkt = PKTSkillStartNotify {
                    source_id: source_id,
                    skill_id: skill_id,
                    skill_option_data: PKTSkillStartNotifyInner {
                        tripod_index,
                        tripod_level,
                    },
                };
                Ok((Pkt::SkillStartNotify, pkt.encode()?))
            }
            EncounterEvent::AddBuff { id, effect_id, duration, source_id, target_id } => {
                let pkt = PKTStatusEffectAddNotify {
                    object_id: source_id,
                    status_effect_data: StatusEffectData { 
                        source_id,
                        status_effect_id: effect_id,
                        status_effect_instance_id: id,
                        value: StatusEffectDataValue { bytearray_0: None },
                        total_time: duration as f32 * 1000.0,
                        stack_count: 0,
                        end_tick: 0
                    }
                };
                Ok((Pkt::StatusEffectAddNotify, pkt.encode()?))
            }
            EncounterEvent::AddPartyBuff { id, effect_id, duration, source_id, target_id } => {
                let pkt = PKTPartyStatusEffectAddNotify {
                    character_id: target_id,
                    status_effect_datas: vec![
                        StatusEffectData {
                            source_id,
                            status_effect_id: effect_id,
                            status_effect_instance_id: id,
                            value: StatusEffectDataValue { bytearray_0: None },
                            total_time: duration as f32 * 1000.0,
                            end_tick: 0,
                            stack_count: 0,
                        }
                    ]
                };
                Ok((Pkt::PartyStatusEffectAddNotify, pkt.encode()?))
            }
            EncounterEvent::RemovePartyBuff { id, target_id } => {
                let pkt = PKTStatusEffectRemoveNotify {
                    object_id: target_id,
                    reason: 0,
                    status_effect_instance_ids: vec![id]
                };
                Ok((Pkt::PartyStatusEffectRemoveNotify, pkt.encode()?))
            }
            EncounterEvent::NpcSummon { id, owner_id, npc_id, hp } => {
                let pkt = PKTNewNpcSummon {
                    npc_struct: NpcStruct { 
                        object_id: id,
                        type_id: npc_id,
                        level: 0,
                        balance_level: NpcStructBalance { value: None },
                        stat_pairs: make_hp_stat_pairs(hp),
                        status_effect_datas: vec![]
                    },
                    owner_id
                };
                Ok((Pkt::NewNpcSummon, pkt.encode()?))
            }
            EncounterEvent::Projectile { id, skill_id, skill_effect_id, owner_id } => {
                let pkt = PKTNewProjectile {
                    projectile_info: PKTNewProjectileInner { 
                        projectile_id: id,
                        owner_id,
                        skill_id,
                        skill_effect: skill_effect_id.unwrap_or_default()
                    }
                };
                Ok((Pkt::NewTrap, pkt.encode()?))
            }
            EncounterEvent::RemoveBuff { id, target_id } => {
                let pkt = PKTStatusEffectRemoveNotify {
                    object_id: target_id,
                    status_effect_instance_ids: vec![id],
                    reason: 0
                };
                Ok((Pkt::NewTrap, pkt.encode()?))
            }
            EncounterEvent::RemoveObject(object_id) => {
                let pkt = PKTRemoveObject {
                    unpublished_objects: vec![PKTRemoveObjectInner { object_id }]
                };
                Ok((Pkt::RemoveObject, pkt.encode()?))
            }
            EncounterEvent::Trap { id, owner_id, skill_id } => {
                let pkt = PKTNewTrap {
                    trap_struct: PKTNewTrapInner { 
                        object_id: id,
                        owner_id: owner_id,
                        skill_id,
                        skill_effect: 0
                    }
                };
                Ok((Pkt::NewTrap, pkt.encode()?))
            }
            _ => todo!("Other events not implemented yet"),
        }
    }
}
