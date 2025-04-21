use crate::{get_templates, EncounterTemplate};


pub struct Simulator {

}

impl Simulator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_templates(&self) -> Vec<EncounterTemplate> {
        get_templates()
    }
}