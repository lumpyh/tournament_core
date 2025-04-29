use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use crate::bewerb::{Bewerb, BewerbSaveable};
use crate::day::{Day, DaySaveable};
use crate::error::Error;

use crate::tournament::{DayData, SimpleDay, SimpleFencer};

use crate::arena_slot::{ArenaSlot, ArenaSlotId};
use crate::container::UidContainer;
use crate::fencer::{Fencer, Fencers};
use crate::group::{Group, GroupId};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TournamentInternal {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct Tournament {
    pub inner: TournamentInternal,
    pub bewerbs: UidContainer<Bewerb>,
    pub days: UidContainer<Day>,
    pub fencers: Fencers,
}

impl Tournament {
    pub fn new() -> Self {
        Tournament::default()
    }

    pub fn days_to_json_file(&self) -> Result<(), Error> {
        let days: Vec<DaySaveable> = self.days.iter().map(|x| x.into()).collect();
        let file = File::create("days.json")?;
        serde_json::to_writer_pretty(file, &days)?;
        Ok(())
    }

    pub fn load_days_from_json_file() -> Result<UidContainer<Day>, Error> {
        let file = File::open("days.json")?;
        let days: Vec<DaySaveable> = serde_json::from_reader(file)?;

        let mut res: UidContainer<Day> = Default::default();
        for day in days {
            res.insert(Day::from_saveable(day.clone()));
        }

        Ok(res)
    }

    pub fn bewerbs_to_json_file(&self) -> Result<(), Error> {
        let days: Vec<BewerbSaveable> = self.bewerbs.iter().map(|x| x.into()).collect();
        let file = File::create("bewerbs.json")?;
        serde_json::to_writer_pretty(file, &days)?;
        Ok(())
    }

    pub fn load_bewerbs_from_json_file() -> Result<UidContainer<Bewerb>, Error> {
        let file = File::open("bewerbs.json")?;
        let bewerbs: Vec<BewerbSaveable> = serde_json::from_reader(file)?;

        let mut res: UidContainer<Bewerb> = Default::default();
        for bewerb in &bewerbs {
            res.insert(Bewerb::from_saveable(bewerb));
        }

        Ok(res)
    }

    pub fn from_json_file(path: &Path) -> Result<Tournament, Error> {
        let file = File::open(path)?;
        let inner = serde_json::from_reader(file)?;
        let fencers = Fencers::from_json_file().unwrap_or_default();
        let days = Self::load_days_from_json_file().unwrap_or_default();
        let bewerbs = Self::load_bewerbs_from_json_file().unwrap_or_default();

        Ok(Tournament {
            inner,
            bewerbs,
            fencers,
            days,
        })
    }

    pub fn to_json_file(&self, path: &Path) -> Result<(), Error> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &self.inner)?;
        if let Err(err) = self.fencers.save_to_file() {
            println!("error while saving fencers {:?}", err);
        }
        if let Err(err) = self.days_to_json_file() {
            println!("error while saving fencers {:?}", err);
        }
        if let Err(err) = self.bewerbs_to_json_file() {
            println!("error while saving fencers {:?}", err);
        }

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

    pub fn add_bewerb(&mut self, name: String, n_rounds: u32, n_groups: u32) {
        let id = self.bewerbs.get_next_id();
        let bewerb = Bewerb::new(id, name, n_rounds, n_groups);
        self.bewerbs.insert(bewerb);
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

    pub fn get_bewerbs(&self) -> Vec<&Bewerb> {
        self.bewerbs.iter().collect()
    }

    pub fn get_all_free_groups(&self) -> Vec<GroupId> {
        let mut res = Vec::new();

        for bewerb in self.bewerbs.iter() {
            let mut round_groups = bewerb.get_free_groups();
            res.append(&mut round_groups);
        }
        res
    }

    fn get_group_by_id_internal(
        bewerbs: &mut UidContainer<Bewerb>,
        id: &GroupId,
    ) -> Option<Arc<Group>> {
        let bewerb = bewerbs.get_mut(id.bewerb_id)?;

        bewerb.get_group_by_id(id)
    }

    pub fn get_group_by_id(&mut self, id: &GroupId) -> Option<Arc<Group>> {
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

    pub fn freeup_group(&mut self, id: &GroupId) -> Result<(), Error> {
        let Some(group) = Self::get_group_by_id_internal(&mut self.bewerbs, id) else {
            return Err(Error::InvalidInput(format!("Ivalid group_id {:?}", id)));
        };

        let Some(curr_arena_id) = group.get_arena() else {
            return Ok(());
        };

        let arena = Self::get_arena_by_id_internal(&mut self.days, &curr_arena_id);

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

    pub fn get_all_fencers(&mut self) -> Result<Vec<SimpleFencer>, Error> {
        Ok(self.fencers.iter().map(|x| x.as_ref().into()).collect())
    }

    pub fn update_fencers(&mut self, fencers: Vec<SimpleFencer>) {
        for fencer in fencers {
            if let Some(item) = self.fencers.iter_mut().find(|x| x.is_same(&fencer)) {
                item.update(fencer.clone());
            } else {
                let new_fencer = Fencer::new(
                    fencer.name.to_owned(),
                    fencer.bewerbs.iter().map(|x| x.into()).collect(),
                );
                self.fencers.push(new_fencer);
            }
        }
    }
}
