use std::{fs::File, io::BufReader};

use failure::Error;
use raylib::math::{Rectangle, Vector2};
use serde::{Deserialize, Serialize};

use crate::{entities::{enemy::{jellyfish::JellyFish, octopus::Octopus, pufferfish::Pufferfish, whirlpool::Whirlpool}, fish::FishEntity}, player::Player};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct World {
    pub end_position: Vector2,
    pub player_spawn: Vector2,

    #[serde(rename = "fish")]
    pub fish_positions: Vec<Vector2>,

    #[serde(skip)]
    pub fish: Vec<FishEntity>,

    #[serde(skip)]
    pub colliders: Vec<Rectangle>,

	// Mobs
    pub jellyfish: Vec<JellyFish>,
    pub octopus: Vec<Octopus>,
	pub whirlpool: Vec<Whirlpool>,
	pub pufferfish: Vec<Pufferfish>
    
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
        result.colliders = Vec::new();
        for collider in colliders.iter() {
            result.colliders.push(Rectangle {
                x: collider.x - (collider.width / 2.0),
                y: collider.y - (collider.height / 2.0),
                width: collider.width,
                height: collider.height,
            });
        }

		

        Ok(result)
    }

    // pub fn spend_coins(&mut self, count: usize) {
    //     for _ in 0..count {
    //         self.fish.pop();
    //     }
    // }

    pub fn reset(&mut self, player: &mut Player) {
        // Init all fish
        self.fish = FishEntity::new_from_positions(&self.fish_positions);

        // Reset the player
        player.reset(self.player_spawn);
    }
}

pub fn load_world_colliders(file: String) -> Result<Vec<Rectangle>, Error> {
    // Load the file
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    // Deserialize
    Ok(serde_json::from_reader(reader)?)
}
