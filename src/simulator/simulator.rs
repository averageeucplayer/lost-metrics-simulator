
use log::debug;
use log::info;
use rand::seq::IndexedMutRandom;
use rand::seq::IndexedRandom;
use rand::Rng;

use crate::simulator::identity::IdentityGenerator;
use crate::simulator::utils::calculate_damage;
use crate::simulator::utils::calculate_player_damage;
use crate::templates::skill_buff::*;
use crate::types::*;

pub struct EncounterSimulator {
    pub generator: IdentityGenerator,
    pub tick: u64,
    pub esther_instance: Option<EstherInstance>,
    pub leader_last_sidereal: u64,
    pub encounter: Encounter
}

impl EncounterSimulator {
    pub fn from(encounter: Encounter) -> Self {
        Self {
            generator: IdentityGenerator::new(),
            esther_instance: None,
            tick: 0,
            leader_last_sidereal: 0,
            encounter,
        }
    }

    fn get_current_boss(&mut self) -> Option<&mut Boss> {
        for boss in self.encounter.bosses.iter_mut() {
            if boss.cur_hp == 0 {
                continue;
            }

            return Some(boss)
        }

        None
    }

    fn check_special_actions(&mut self) -> Vec<EncounterEvent> {
        let mut events = vec![];

        let self_ptr: *mut Self = self;

        if let Some(esther) = self.esther_instance.as_mut() {

            if self.tick < esther.expires_at {

                let damage = self.generator.random_range(esther.min_damage..esther.max_damage);

                let boss = unsafe { match (*self_ptr).get_current_boss() {
                    Some(boss) => boss,
                    None => return events,
                } };

                boss.cur_hp = boss.cur_hp.saturating_sub(damage);

                let event = EncounterEvent::Damage {
                    source_id: esther.id,
                    target_id: boss.id,
                    damage: 1000,
                    skill_id: 1,
                    hit_option: HitOption::None,
                    hit_flag: HitFlag::Normal,
                    cur_hp: boss.cur_hp,
                    max_hp: boss.max_hp
                };
                events.push(event);
            }
            else {
                let event = EncounterEvent::RemoveObject(esther.id);
                events.push(event);
                self.esther_instance = None;
            }
        }

        if self.tick - self.leader_last_sidereal >= self.encounter.sidereal_frequency {
            let id = self.generator.new_u64();
            let esther = self.encounter.esther.choose(&mut self.generator).unwrap();

            events.push(EncounterEvent::Npc {
                id,
                npc_id: esther.npc_type_id,
                hp: esther.hp
            });

            let boss = unsafe { match (*self_ptr).get_current_boss() {
                Some(boss) => boss,
                None => return events,
            } };

            boss.cur_hp = boss.cur_hp.saturating_sub(esther.initial_damage);

            let event = EncounterEvent::Damage {
                source_id: id,
                target_id: boss.id,
                damage: 1000,
                skill_id: 1,
                hit_option: HitOption::None,
                hit_flag: HitFlag::Normal,
                cur_hp: boss.cur_hp,
                max_hp: boss.max_hp
            };

            events.push(event);

            if boss.cur_hp == 0 {
                return events
            }

            self.leader_last_sidereal = self.tick;
        }

        events
    }

    pub fn finalize(self) -> Vec<EncounterEvent> {
        let mut events = vec![];

        if self.encounter.invoke_boss_kill_packet {
            let event = EncounterEvent::RaidBossKill;
            events.push(event);
        }

        if self.encounter.invoke_raid_result_packet {
            let event = EncounterEvent::RaidResult;
            events.push(event);
        }

        if let Some(id) = self.encounter.invoke_trigger_start_packet {
            let event = EncounterEvent::Trigger(id);
            events.push(event);
        }

        if self.encounter.invoke_zone_change_packet {
            let event = EncounterEvent::ZoneChange(self.encounter.local_player_id);
            events.push(event);
        }

        events
    }

    pub fn is_finished(&self) -> bool {
        self.tick > self.encounter.max_ticks || self.encounter.bosses.iter().all(|b| b.cur_hp <= 0)
    }

    pub fn spawn_phase(&self) -> Vec<EncounterEvent> {
        let mut events = vec![];

        for party in &self.encounter.parties {
            for player in &party.members {
                if player.is_local {
                    events.push(EncounterEvent::SpawnLocalPlayer(player.clone()));
                    break;
                }
            }
        }

        for party in &self.encounter.parties {
            for player in &party.members {
                if !player.is_local {

                    if self.encounter.use_vehicle_packet_instead_of_spawn {
                        events.push(EncounterEvent::NewVehicle(player.clone()));
                    }
                    else {
                        events.push(EncounterEvent::SpawnPlayer(player.clone()));
                    }

                }
            }

            events.push(EncounterEvent::PartyInfo {
                raid_instance_id: self.encounter.raid_instance_id,
                party_instance_id: party.id,
                members: party.members.clone()
            });
        }

        for boss in &self.encounter.bosses {
            events.push(EncounterEvent::SpawnBoss(boss.clone()));
        }

        if self.encounter.supports_raid_begin_packet {
            events.push(EncounterEvent::RaidBegin(self.encounter.zone_id));
        }

        if self.encounter.supports_zone_member_load_packet {
            events.push(EncounterEvent::ZoneMemberLoad {
                zone_id: self.encounter.zone_id,
                zone_level: self.encounter.zone_level
            });
        }

        if let Some(id) = self.encounter.new_transit_packet {
            events.push(EncounterEvent::NewTransit(id));
        }

        events
    }

    pub fn next_events(&mut self) -> Vec<EncounterEvent> {
        let mut events = vec![];

        if self.tick == 0 {
            self.tick += 1;
            return self.spawn_phase()
        }

        let tick = self.tick;

        if let Some(boss) = self.get_current_boss() {
            Self::process_boss_debuffs(boss, tick, &mut events);
        }

        let self_ptr: *mut Self = self;

        for party in self.encounter.parties.iter_mut() {
            let ptr: *mut Party = party;
            let party_ref: &mut Party = unsafe { &mut *ptr };

            for player in party.members.iter_mut() {

                if player.is_dead {
                    continue;
                }

                if player.dies_at > self.tick {
                    events.push(EncounterEvent::Death(player.id));
                    player.is_dead = true;
                    continue;
                }

                Self::process_buffs(self.tick, player, &mut events);

                if player.incapacitate_time > 0 {
                    player.incapacitate_time -= 1;
                    continue;
                }

                let boss = unsafe { match (*self_ptr).get_current_boss() {
                    Some(boss) => boss,
                    None => break,
                } };

                if boss.cur_hp == 0 {
                    return events;
                }

                Self::process_player_summons_and_traps(
                    self.tick,
                    &mut self.generator,
                    player,
                    boss,
                    &mut events);

                let boss = unsafe { match (*self_ptr).get_current_boss() {
                    Some(boss) => boss,
                    None => break,
                } };

                if boss.cur_hp == 0 {
                    return events;
                }

                let has_casted_skill = Self::process_player_skills(
                    self.tick,
                    self.encounter.allowed_skill_cast_count,
                    &mut self.generator,
                    player,
                    boss,
                    &mut events,
                    party_ref);

                let boss = unsafe { match (*self_ptr).get_current_boss() {
                    Some(boss) => boss,
                    None => break,
                } };

                if boss.cur_hp == 0 {
                    return events;
                }

                if has_casted_skill {
                    Self::apply_after_cast_effects(
                        tick,
                        &mut self.generator,
                        player,
                        party_ref,
                        &mut events);
                }

                if boss.cur_hp == 0 {
                    return events;
                }
            }

            let boss = unsafe { match (*self_ptr).get_current_boss() {
                Some(boss) => boss,
                None => break,
            } };

            let party = unsafe { (*self_ptr).encounter.parties.choose_mut(&mut self.generator).unwrap() };
            let player = party.members.choose_mut(&mut self.generator).unwrap();

            Self::try_damage_or_knock_player(
                &mut self.generator,
                &mut events,
                &boss,
                &player,
                self.encounter.boss_attack_chance,
                self.encounter.boss_knock_chance);
        }

        events.extend(self.check_special_actions());

        self.tick += 1;
        events
    }

    fn apply_after_cast_effects(
        tick: u64,
        generator: &mut IdentityGenerator,
        player: &mut Player,
        party: &mut Party,
        events: &mut Vec<EncounterEvent>) {
        match player.is_support {
            true => {
                // println!("{:?}", player.id);

                for member in party.members.iter_mut() {

                    let id = generator.new_u32();
                    let buff = Buff {
                        id,
                        unique_group: 1,
                        is_party: COMBAT_BLESSING_III.is_party,
                        effect_id: COMBAT_BLESSING_III.id,
                        buff_type: COMBAT_BLESSING_III.buff_type,
                        duration: COMBAT_BLESSING_III.duration,
                        expires_at: tick + COMBAT_BLESSING_III.duration as u64,
                    };

                    let old = member.buffs.insert(COMBAT_BLESSING_III.id, buff);

                    if let Some(old) = old {
                        generator.remove_u32(old.id);
                        events.push(EncounterEvent::RemovePartyBuff { id: old.id, target_id: player.id });
                    }

                    events.push(EncounterEvent::AddPartyBuff {
                        id,
                        effect_id: COMBAT_BLESSING_III.id,
                        duration: COMBAT_BLESSING_III.duration,
                        source_id: player.id,
                        target_id: member.id
                    });
                }
            },
            false => {
                let id = generator.new_u32();

                let buff = Buff {
                    id,
                    effect_id: ADRENALINE.id,
                    buff_type: ADRENALINE.buff_type,
                    duration: ADRENALINE.duration,
                    is_party: ADRENALINE.is_party,
                    unique_group: ADRENALINE.unique_group,
                    expires_at: tick + ADRENALINE.duration as u64,
                };

                let old = player.buffs.insert(ADRENALINE.id, buff);

                if let Some(old) = old {
                    generator.remove_u32(old.id);
                    events.push(EncounterEvent::RemoveBuff { id: old.id, target_id: player.id });
                }

                events.push(EncounterEvent::AddBuff {
                    id,
                    effect_id: ADRENALINE.id,
                    duration: ADRENALINE.duration,
                    source_id: player.id,
                    target_id: player.id
                });
            },
        }
    }

    fn process_player_summons_and_traps(
        tick: u64,
        generator: &mut IdentityGenerator,
        player: &mut Player,
        boss: &mut Boss,
        events: &mut Vec<EncounterEvent>,
    ) {
        let boss_id = boss.id;

        let mut expired_summons = Vec::new();
        for (&id, summon) in &player.summons {
            if tick >= summon.expires_at {
                expired_summons.push(id);
            } else {
                let attack_power = player.attack_power as f64;
                let damage = summon.damage_ratio * attack_power;
                let damage = damage as i64;

                boss.cur_hp = boss.cur_hp.saturating_sub(damage);

                if boss.cur_hp == 0 {
                    return
                }

                events.push(EncounterEvent::Damage {
                    source_id: id,
                    target_id: boss_id,
                    damage,
                    skill_id: summon.skill_id,
                    hit_option: HitOption::None,
                    hit_flag: HitFlag::Normal,
                    cur_hp: boss.cur_hp,
                    max_hp: boss.max_hp,
                });
            }
        }

        for id in expired_summons {
            player.summons.remove(&id);
            events.push(EncounterEvent::RemoveObject(id));
        }

        let mut expired_traps = Vec::new();
        for (&id, trap) in &player.traps {
            if tick >= trap.expires_at {
                expired_traps.push(id);
            } else {
                let (damage, hit_flag, hit_option) = calculate_damage(
                    generator,
                    &player,
                    trap.damage_ratio,
                    &boss.debuffs
                );
                boss.cur_hp = boss.cur_hp.saturating_sub(damage);

                events.push(EncounterEvent::Damage {
                    source_id: id,
                    target_id: boss_id,
                    damage,
                    skill_id: trap.skill_id,
                    hit_option,
                    hit_flag,
                    cur_hp: boss.cur_hp,
                    max_hp: boss.max_hp,
                });
            }
        }

        for id in expired_traps {
            player.traps.remove(&id);
            events.push(EncounterEvent::RemoveObject(id));
        }
    }

    fn process_player_skills(
        tick: u64,
        allowed_skill_cast_count: u32,
        generator: &mut IdentityGenerator,
        player: &mut Player,
        boss: &mut Boss,
        events: &mut Vec<EncounterEvent>,
        party_ref: &mut Party,
    ) -> bool {
        debug!("Processing player skills - PlayerId: {}", player.id);

        let mut has_casted_skill = false;
        let boss_id = boss.id;
        let boss_max_hp = boss.max_hp;

        let ptr: *mut Player = player;
        let player_ref: &mut Player = unsafe { &mut *ptr };
        let mut skill_cast_count = 0;

        for skill in &mut player.skills {
            let skill_id = skill.id;

            if !skill.is_ready(tick) {
                continue;
            }

            let is_in_unique_group = player.buffs.iter().any(|(_, buff)| {
                skill.effects.iter().any(|effect| {
                    matches!(effect, SkillEffect::Buff { unique_group, .. } if buff.unique_group == *unique_group)
                })
            });

            if player.is_support && is_in_unique_group {
                continue;
            }

            if skill_cast_count > allowed_skill_cast_count {
                break;
            }

            skill_cast_count += 1;
            has_casted_skill = true;
            skill.cooldown_ends_at = tick + skill.cooldown as u64;

            events.push(EncounterEvent::SkillStart {
                source_id: player.id,
                skill_id,
                tripod_index: skill.tripod_index,
                tripod_level: skill.tripod_level
            });

            events.push(EncounterEvent::SkillCast { source_id: player.id, skill_id });

            if skill.is_counter {
                events.push(EncounterEvent::Counter(player.id));
            }
            
            // println!("{:?}, {}", skill.effects, skill.effects.len());
            for effect in skill.effects.clone() {

                match effect {
                    SkillEffect::Damage { damage_ratio } => {
                        let (damage, hit_flag, hit_option) =
                            calculate_player_damage(
                                generator,
                                player_ref,
                                damage_ratio,
                                skill.hit_option,
                                &boss.debuffs);

                        boss.cur_hp = boss.cur_hp.saturating_sub(damage);

                        events.push(EncounterEvent::Damage {
                            source_id: player.id,
                            target_id: boss_id,
                            damage,
                            skill_id: skill.id,
                            hit_option,
                            hit_flag,
                            cur_hp: boss.cur_hp,
                            max_hp: boss_max_hp,
                        });
                    }
                    SkillEffect::Buff { effect_id, unique_group, is_party, duration, buff_type } => {
                        Self::apply_buff(
                            tick,
                            generator,
                            player_ref,
                            party_ref,
                            boss,
                            events,
                            effect_id,
                            unique_group,
                            is_party,
                            duration,
                            buff_type);
                    }
                    SkillEffect::Summon { duration, skill_id, npc_id, damage_ratio } => {
                        Self::spawn_summon(tick,
                            generator,
                            player_ref,
                            events,
                            skill_id,
                            npc_id,
                            damage_ratio,
                            duration);
                    }
                    SkillEffect::Projectile { damage_ratio } => {
                        Self::spawn_projectile(
                            generator,
                            player_ref,
                            events,
                            boss,
                            skill.id,
                            damage_ratio);
                    }
                    SkillEffect::Trap { duration, skill_id, damage_ratio } => {
                        Self::spawn_trap(tick,
                            generator,
                            player_ref,
                            events,
                            skill_id,
                            damage_ratio,
                            duration);
                    }
                }
            }
        }

        has_casted_skill
    }

    fn apply_buff(
        tick: u64,
        generator: &mut IdentityGenerator,
        player: &mut Player,
        party_ref: &mut Party,
        boss: &mut Boss,
        events: &mut Vec<EncounterEvent>,
        effect_id: u32,
        unique_group: u32,
        is_party: bool,
        duration: u32,
        buff_type: BuffType,
    ) {
        debug!("Applying buff - PlayerId: {}", player.id);
        
        if let BuffType::Brand(_) = buff_type {
            let id = generator.new_u32();
            let buff = Buff {
                id,
                is_party: false,
                effect_id,
                unique_group,
                buff_type,
                duration,
                expires_at: tick + duration as u64,
            };
            let old = boss.debuffs.insert(effect_id, buff);

            if let Some(old) = old {
                generator.remove_u32(old.id);
                events.push(EncounterEvent::RemoveBuff { id: old.id, target_id: player.id });
            }

            events.push(EncounterEvent::AddBuff { id, effect_id, duration, source_id: player.id, target_id: boss.id });
        } else if is_party {
            for member in &mut party_ref.members {
                let id = generator.new_u32();
                let buff_type = match buff_type {
                    BuffType::AttackPower(multiplier) => BuffType::AttackPower(multiplier * player.attack_power as f64),
                    buff => buff
                };
                let buff = Buff {
                    id,
                    is_party,
                    effect_id,
                    unique_group,
                    buff_type,
                    duration,
                    expires_at: tick + duration as u64,
                };

                let old = member.buffs.insert(effect_id, buff);

                if let Some(old) = old {
                    generator.remove_u32(old.id);
                    events.push(EncounterEvent::RemovePartyBuff { id: old.id, target_id: member.id });
                }

                // println!("PlayerId: {} {}", player.id, effect_id);
                let event = EncounterEvent::AddPartyBuff {
                    id,
                    effect_id,
                    duration,
                    source_id: player.id,
                    target_id: member.id
                };

                events.push(event);
            }
        } else {
            let id = generator.new_u32();
            let buff = Buff {
                id,
                is_party,
                effect_id,
                unique_group,
                buff_type,
                duration,
                expires_at: tick + duration as u64,
            };
            
            let old = player.buffs.insert(effect_id, buff);

            if let Some(old) = old {
                generator.remove_u32(old.id);
                events.push(EncounterEvent::RemoveBuff { id: old.id, target_id: player.id });
            }

            events.push(EncounterEvent::AddBuff { id, effect_id, duration, source_id: player.id, target_id: player.id });
        }
    }

    fn spawn_summon(
        tick: u64,
        generator: &mut IdentityGenerator,
        player: &mut Player,
        events: &mut Vec<EncounterEvent>,
        skill_id: u32,
        npc_id: u32,
        damage_ratio: f64,
        duration: u32) {
        let id = generator.new_u64();
        let summon = Summon {
            id,
            skill_id,
            damage_ratio,
            expires_at: tick + duration as u64,
        };
        player.summons.insert(id, summon);

        events.push(EncounterEvent::NpcSummon {
            id,
            npc_id,
            owner_id: player.id,
            hp: 1,
        });
    }

    fn spawn_projectile(
        generator: &mut IdentityGenerator,
        player: &mut Player,
        events: &mut Vec<EncounterEvent>,
        boss: &mut Boss,
        skill_id: u32,
        damage_ratio: f64) {
        let id = generator.new_u64();
        events.push(EncounterEvent::Projectile { 
            id,
            skill_id,
            skill_effect_id: None,
            owner_id: player.id
        });

        let attack_power = player.attack_power as f64;
        let damage = damage_ratio * attack_power;
        let damage = damage as i64;

        boss.cur_hp = boss.cur_hp.saturating_sub(damage);
        events.push(EncounterEvent::Damage {
            source_id: id,
            target_id: boss.id,
            damage,
            skill_id,
            hit_option: HitOption::None,
            hit_flag: HitFlag::Normal,
            cur_hp: boss.cur_hp,
            max_hp: boss.max_hp,
        });

        events.push(EncounterEvent::RemoveObject(id));
    }

    fn spawn_trap(
        tick: u64,
        generator: &mut IdentityGenerator,
        player: &mut Player,
        events: &mut Vec<EncounterEvent>,
        skill_id: u32,
        damage_ratio: f64,
        duration: u32) {
        let id = generator.new_u64();
        let trap = Trap {
            id,
            skill_id,
            damage_ratio,
            expires_at: tick + duration as u64,
        };
        player.traps.insert(id, trap);

        events.push(EncounterEvent::Trap { id, owner_id: player.id, skill_id });
    }

    fn process_buffs(tick: u64, player: &mut Player, events: &mut Vec<EncounterEvent>) {

        player.buffs.retain(|id, buff| {
            if tick >= buff.expires_at {
                let event = if buff.is_party {
                    EncounterEvent::RemovePartyBuff {
                        id: *id,
                        target_id: player.id,
                    }
                } else {
                    EncounterEvent::RemoveBuff {
                        id: *id,
                        target_id: player.id,
                    }
                };
                events.push(event);
                false
            } else {
                true
            }
        });
    }

    fn try_damage_or_knock_player(
        generator: &mut IdentityGenerator,
        events: &mut Vec<EncounterEvent>,
        boss: &Boss,
        player: &Player,
        attack_chance: f64,
        knock_chance: f64) {

        let can_attack = generator.random_bool(attack_chance);

        if !can_attack {
            return
        }

        debug!("Boss attack BossId: {} PlayerId {}", boss.id, player.id);

        let can_knock = generator.random_bool(knock_chance);

        if can_knock {
            let id = generator.new_u32();
            let debuff = generator.get_random_cc_debuff();

            let event = EncounterEvent::AddBuff {
                id,
                effect_id: debuff.id,
                duration: debuff.duration,
                source_id: boss.id,
                target_id: player.id,
            };

            events.push(event);
        }

        let damage = generator.random_range(boss.min_damage..boss.max_damage);
        let skill_id = *boss.skill_ids.choose(generator).unwrap();

        let event = EncounterEvent::Damage {
            source_id: boss.id,
            target_id: player.id,
            damage,
            skill_id,
            hit_option: HitOption::None,
            hit_flag: HitFlag::Normal,
            cur_hp: player.cur_hp,
            max_hp: player.max_hp
        };

        events.push(event);
    }

    fn process_boss_debuffs(
        boss: &mut Boss,
        tick: u64,
        events: &mut Vec<EncounterEvent>,
    ) {
        let mut expired = Vec::new();

        for (&id, debuff) in boss.debuffs.iter() {
            if tick >= debuff.expires_at {
                expired.push(id);
                events.push(EncounterEvent::RemoveBuff {
                    id,
                    target_id: boss.id,
                });
            }
        }

        for id in expired {
            boss.debuffs.remove(&id);
        }
    }
}