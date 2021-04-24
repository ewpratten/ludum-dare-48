use std::{fs::File, io::BufReader};

use raylib::math::Vector2;
use serde::{Deserialize, Serialize};
use std::io::Read;
use failure::Error;

use crate::entities::fish::FishEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct World {
    pub end_position: Vector2,

    #[serde(rename = "fish")]
    pub fish_positions: Vec<Vector2>,

    #[serde(skip)]
    pub fish: Vec<FishEntity>
}

impl World {
    pub fn load_from_json(file: String) -> Result<Self, Error> {
        // Load the file
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        // Deserialize
        let mut result: World = serde_json::from_reader(reader)?;

        // Init all fish
        result.fish = FishEntity::new_from_positions(&result.fish_positions);

        Ok(result)
    }
}
