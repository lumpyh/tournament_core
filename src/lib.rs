use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

mod error;

use error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tournament {
    pub name: String,
}

impl Tournament {
    pub fn new() -> Self {
        Tournament {
            name: "".to_string(),
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
}
