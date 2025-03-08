use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

use crate::bewerb::Bewerb;
use crate::day::Day;
use crate::error::Error;

use crate::tournament_service::tournament::SimpleDay;

use crate::container::UidContainer;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tournament {
    pub name: String,
    pub days: Vec<Day>,
    pub bewerbs: UidContainer<Bewerb>,
}

impl Tournament {
    pub fn new() -> Self {
        Tournament {
            name: "".to_string(),
            days: Vec::new(),
            bewerbs: UidContainer::default(),
        }
    }

    pub fn from_json_file(path: &Path) -> Result<Tournament, Error> {
        let file = File::open(path)?;
        let tournament = serde_json::from_reader(file)?;
        Ok(tournament)
    }

    pub fn to_json_file(&self, path: &Path) -> Result<(), Error> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn add_day(&mut self, mut day: Day) {
        let mut id = 0;
        let ids: Vec<u32> = self.days.iter().map(|x| x.id).collect();
        while ids.contains(&id) {
            id += 1;
        }
        day.id = id;

        self.days.push(day);
    }

    pub fn remove_day(&mut self, id: u32) {
        self.days.retain(|e| e.id != id);
    }

    pub fn get_simple_days(&self) -> Vec<SimpleDay> {
        self.days.iter().map(|e| e.into()).collect()
    }

    pub fn add_bewerb(&mut self, name: String, n_groups: u32, n_rounds: u32) {
        let bewerb = Bewerb::new(name, n_groups, n_rounds);
        self.bewerbs.push(bewerb);
    }

    pub fn remove_bewerb(&mut self, id: u32) {
        self.bewerbs.remove(id);
    }

}
