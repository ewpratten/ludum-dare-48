//! This file contains the global state of the game. Data here is passed around to all handler functions.

use std::{fmt, fs::File, io::BufReader};

use raylib::{
    camera::Camera2D, math::Vector2, prelude::RaylibDrawHandle, RaylibHandle, RaylibThread,
};

use crate::{
    player::{Player, PlayerInventory},
    resources::GlobalResources,
    world::World,
};

use failure::Error;
use log::debug;
use serde::{Deserialize, Serialize};

/// Overall states for the game
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    Loading,
    MainMenu,
    PauseMenu,
    GameQuit,
    InGame,
    GameEnd,
    InShop
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GameProgress {
    pub coins: u32,
    pub max_depth: f32,
    pub fastest_time: Option<f64>,
    pub inventory: PlayerInventory,
}

impl GameProgress {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_file(file: String) -> Result<Self, Error> {
        // Load the file
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        // Deserialize
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn try_from_file(file: String) -> Self {
        // Load from file
        let loaded = GameProgress::from_file(file);
        if loaded.is_ok() {
            return loaded.unwrap();
        } else {
            return GameProgress::new();
        }
    }

    pub fn to_file(&self, file: String) -> Result<(), Error> {
        // Serialize
        let json = serde_json::to_string(self)?;

        // Write to file
        std::fs::write(file, json)?;

        Ok(())
    }
}

/// This structure contains the entire game state, and should be passed around to various logic functions.
pub struct GameCore {
    /// The game's overall state
    pub state: GameState,
    pub last_state: GameState,
    pub last_state_change_time: f64,
    pub last_frame_time: f64,
    pub has_rendered_first_frame: bool,

    /// Resources
    pub resources: GlobalResources,

    /// Camera (more than one maybe?)
    pub master_camera: Camera2D,

    /// Debug features
    pub show_simple_debug_info: bool,

    /// The world
    pub world: World,

    /// The player
    pub player: Player,
    pub progress: GameProgress,
}

impl GameCore {
    pub fn new(
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
        world: World,
        progress: GameProgress,
    ) -> Self {
        let player = Player::new(&world.player_spawn);
        Self {
            state: GameState::Loading,
            last_state: GameState::Loading,
            last_state_change_time: 0.0,
            last_frame_time: 0.0,
            has_rendered_first_frame: false,
            resources: GlobalResources::load_all(raylib, thread)
                .expect("Failed to load game assets. Can not launch!"),
            master_camera: Camera2D {
                offset: Vector2::zero(),
                target: world.player_spawn,
                rotation: 0.0,
                zoom: 2.0,
            },
            show_simple_debug_info: false,
            world: world,
            player,
            progress: progress,
        }
    }

    pub fn switch_state(&mut self, new_state: GameState, draw_handle: Option<&RaylibDrawHandle>) {
        debug!("Switching global state to: {}", new_state);

        self.last_state = self.state;
        self.state = new_state;

        if draw_handle.is_some() {
            self.last_state_change_time = draw_handle.as_ref().unwrap().get_time();
        }
    }
}
