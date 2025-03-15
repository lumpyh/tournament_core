use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

use crate::bewerb::Bewerb;
use crate::day::Day;
use crate::error::Error;

use crate::tournament::{DayData, SimpleDay};

use crate::arena_slot::{ArenaSlot, ArenaSlotId};
use crate::container::UidContainer;
use crate::group::{Group, GroupId};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tournament {
    pub name: String,
    pub days: UidContainer<Day>,
    pub bewerbs: UidContainer<Bewerb>,
}

impl Tournament {
    pub fn new() -> Self {
        Tournament {
            name: "".to_string(),
            days: UidContainer::default(),
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
        self.days.remove(id);
    }

    pub fn get_simple_days(&self) -> Vec<SimpleDay> {
        self.days.iter().map(|e| e.into()).collect()
    }

    pub fn add_bewerb(&mut self, name: String, n_groups: u32, n_rounds: u32) {
        let bewerb = Bewerb::new(name, n_groups, n_rounds);
        self.bewerbs.push(bewerb);
    }

    pub fn remove_bewerb(&mut self, id: u32) {
        let Some(bewerb) = self.bewerbs.get(id) else {
            return;
        };

        let groups = bewerb.get_all_groups();
        for group in groups {
            let _ = self.freeup_group(&group);
        }

        self.bewerbs.remove(id);
    }

    fn get_group_by_id_internal<'a>(
        bewerbs: &'a mut UidContainer<Bewerb>,
        id: &GroupId,
    ) -> Option<&'a mut Group> {
        let bewerb = bewerbs.get_mut(id.bewerb_id)?;

        bewerb.get_group_by_id(id)
    }

    pub fn get_group_by_id(&mut self, id: &GroupId) -> Option<&mut Group> {
        Self::get_group_by_id_internal(&mut self.bewerbs, id)
    }

    fn get_arena_by_id_internal<'a>(
        days: &'a mut UidContainer<Day>,
        id: &ArenaSlotId,
    ) -> Option<&'a mut ArenaSlot> {
        let day = days.get_mut(id.day_id)?;
        day.get_arena(id)
    }

    pub fn get_arena_by_id(&mut self, id: &ArenaSlotId) -> Option<&mut ArenaSlot> {
        Self::get_arena_by_id_internal(&mut self.days, id)
    }

    fn freeup_group(&mut self, id: &GroupId) -> Result<(), Error> {
        let Some(group) = Self::get_group_by_id_internal(&mut self.bewerbs, id) else {
            return Err(Error::InvalidInput(format!("Ivalid group_id {:?}", id)));
        };

        let Some(curr_arena_id) = group.get_arena() else {
            return Ok(());
        };

        let arena = Self::get_arena_by_id_internal(&mut self.days, curr_arena_id);

        match arena {
            Some(arena) => arena.set_group(None),
            None => println!("warning: curr arena {:?} not found", curr_arena_id),
        }
        group.set_arena(None);

        Ok(())
    }

    fn freeup_arena(&mut self, id: &ArenaSlotId) -> Result<(), Error> {
        let Some(arena) = Self::get_arena_by_id_internal(&mut self.days, id) else {
            return Err(Error::InvalidInput(format!("Ivalid arena_id {:?}", id)));
        };

        let Some(curr_group_id) = arena.get_group() else {
            return Ok(());
        };

        let group = Self::get_group_by_id_internal(&mut self.bewerbs, curr_group_id);

        match group {
            Some(group) => group.set_arena(None),
            None => println!("warning: curr group {:?} not found", curr_group_id),
        }
        arena.set_group(None);

        Ok(())
    }

    pub fn add_group_to_arena(
        &mut self,
        group_id: &GroupId,
        arena_id: &ArenaSlotId,
    ) -> Result<(), Error> {
        self.freeup_arena(arena_id)?;
        self.freeup_group(group_id)?;

        let Some(arena) = Self::get_arena_by_id_internal(&mut self.days, arena_id) else {
            return Err(Error::InvalidInput("Ivalid arena_id".to_string()));
        };

        let Some(group) = Self::get_group_by_id_internal(&mut self.bewerbs, group_id) else {
            return Err(Error::InvalidInput("Ivalid group_id".to_string()));
        };

        arena.set_group(Some(group_id.clone()));
        group.set_arena(Some(arena_id.clone()));

        Ok(())
    }

    pub fn get_day_data(&self, id: u32) -> Result<DayData, Error> {
        let Some(day) = self.days.get(id) else {
            return Err(Error::InvalidInput(format!("Ivalid arena_id {:?}", id)));
        };

        Ok(day.into())
    }
}
