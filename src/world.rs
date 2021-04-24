use std::{fs::File, io::BufReader};

use raylib::math::{Rectangle, Vector2};
use serde::{Deserialize, Serialize};
use std::io::Read;
use failure::Error;

use crate::entities::fish::FishEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct World {
    pub end_position: Vector2,
    pub player_spawn: Vector2,

    #[serde(rename = "fish")]
    pub fish_positions: Vec<Vector2>,

    #[serde(skip)]
    pub fish: Vec<FishEntity>,

    #[serde(skip)]
    pub colliders: Vec<Rectangle>
}

impl World {
    pub fn load_from_json(file: String, colliders: Vec<Rectangle>) -> Result<Self, Error> {
        // Load the file
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        // Deserialize
        let mut result: World = serde_json::from_reader(reader)?;

        // Init all fish
        result.fish = FishEntity::new_from_positions(&result.fish_positions);

        // Init colliders
        result.colliders = colliders;

        Ok(result)
    }

    pub fn spend_coins(&mut self, count: usize) {
        for _ in 0..count {
            self.fish.pop();
        }
    }

    pub fn reset(&mut self) {
        for fish in self.fish.iter_mut() {
            fish.following_player = false;
        }
    }
}


pub fn load_world_colliders(file: String) -> Result<Vec<Rectangle>, Error> {
    // Load the file
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    // Deserialize
    Ok(serde_json::from_reader(reader)?)
}