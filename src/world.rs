use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use std::io::Read;
use failure::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct World {}

impl World {
    pub fn load_from_json(file: String) -> Result<Self, Error> {
        // Load the file
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        // Deserialize
        Ok(serde_json::from_reader(reader)?)
    }
}
