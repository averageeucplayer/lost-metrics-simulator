use std::{collections::HashSet, ops::{Deref, DerefMut}};

use rand::{distr::{Alphabetic, SampleString}, rng, rngs::ThreadRng, seq::{IndexedRandom, IteratorRandom}, Rng};

use crate::{templates::debuffs::ALL_DEBUFFS, types::{ClassWithSpecialisation, SkillBuff}};

pub struct IdentityGenerator {
    used_u64: HashSet<u64>,
    used_u32: HashSet<u32>,
    nicknames: HashSet<String>,
    rng: ThreadRng,
}

impl IdentityGenerator {
    pub fn new() -> Self {
        Self {
            used_u64: HashSet::new(),
            used_u32: HashSet::new(),
            nicknames: HashSet::new(),
            rng: rand::rng(),
        }
    }

    pub fn remove_u32(&mut self, id: u32) {
        self.used_u32.remove(&id);
    }

    pub fn remove_u64(&mut self, id: u64) {
        self.used_u64.remove(&id);
    }

    pub fn new_u32(&mut self) -> u32 {
        let mut id: u32 = self.rng.random();

        while self.used_u32.contains(&id) {
            id = self.rng.random();
        }

        id
    }

    pub fn new_u64(&mut self) -> u64 {
        let mut id: u64 = self.rng.random();

        while self.used_u64.contains(&id) {
            id = self.rng.random();
        }

        id
    }

    pub fn random_sup(&mut self) -> ClassWithSpecialisation {
        let class = ClassWithSpecialisation::SUPPORT().iter().choose(&mut self.rng).unwrap();
        *class
    }

    pub fn random_dps(&mut self) -> ClassWithSpecialisation {
        let class = ClassWithSpecialisation::DPS().iter().choose(&mut self.rng).unwrap();
        *class
    }

    pub fn random_nickname(&mut self) -> String {
        let mut rng = rng();
        let mut string;

        loop {
            string = Alphabetic.sample_string(&mut rng, 10);

            let char = string.get_mut(0..1).unwrap();
            char.make_ascii_uppercase();

            let str = string.get_mut(1..).unwrap();
            str.make_ascii_lowercase();

            if !self.nicknames.contains(str) {
                break;
            }
        }

        string
    }

    pub fn get_random_cc_debuff(&mut self) -> &'static SkillBuff {
        ALL_DEBUFFS.choose(&mut self.rng).unwrap()
    }
}

impl Deref for IdentityGenerator {
    type Target = ThreadRng;
    fn deref(&self) -> &Self::Target {
        &self.rng
    }
}

impl DerefMut for IdentityGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rng
    }
}