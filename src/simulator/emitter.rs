use std::{sync::mpsc::{self, Sender}, thread::{self, sleep}, time::Duration};

use anyhow::Result;
use log::info;
use meter_core_fake::packets::opcodes::Pkt;

use crate::{simulator::builder::{EncounterBuilder, EncounterBuilderArgs}, EncounterSimulator};

pub fn run_encounter(emitter: Sender<(Pkt, Vec<u8>)>, delay: Duration) -> Result<()> {
    let args = EncounterBuilderArgs {
        ..Default::default()
    };

    let builder = EncounterBuilder::new(args);
    let encounter = builder.build();
    let mut simulator = EncounterSimulator::from(encounter);

    while !simulator.is_finished() {
        let events = simulator.next_events();

        for event in events {
            
            let packet = event.to_packet()?;
            emitter.send(packet)?;
            
            sleep(delay);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{simulator::builder::PlayerConfiguration, types::{BerserkerSpecialisation, ClassWithSpecialisation}};

    use super::*;
    use std::{collections::HashMap, time::Duration};

    #[test]
    fn should_contain_spawn_packets() {
        let mut player_configuration_map: HashMap<u8, PlayerConfiguration> = HashMap::new();

        let player_configuration = PlayerConfiguration {
            dies_at: 3,
            class_with_specialisation: ClassWithSpecialisation::Berserker(BerserkerSpecialisation::Mayhem),
            hp: 300_000,
            skill_tripod_map: HashMap::new()
        };

        player_configuration_map.insert(1, player_configuration);

        let args = EncounterBuilderArgs {
            player_configuration_map,
            ..Default::default()
        };

        let builder = EncounterBuilder::new(args);
        let encounter = builder.build();

        let member = encounter.parties.first().unwrap().members.iter().nth(1).unwrap();

        // println!("{:?}", member.class);
        // println!("{:?}", encounter.parties);
        
        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut simulator = EncounterSimulator::from(encounter);

        for index in 0..10 {
            let counts: std::collections::HashMap<_, _> = simulator.next_events()
                .iter()
                .fold(std::collections::HashMap::new(), |mut acc, e| { 
                    *acc.entry(e.as_ref().to_string()).or_insert(0) += 1; acc });

            let message = counts.iter()
                .map(|(name, count)| format!("{} = {}", name, count))
                .collect::<Vec<_>>()
                .join(", ");
            println!("Tick {} - {}", index, message);            
        }

        // println!("{:?}", events);
        // let packet = event.to_packet().unwrap();
    }
}
