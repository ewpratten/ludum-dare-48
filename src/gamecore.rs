//! This file contains the global state of the game. Data here is passed around to all handler functions.

use std::fmt;

use raylib::{RaylibHandle, RaylibThread};

use crate::resources::GlobalResources;

/// Overall states for the game
#[derive(Debug, PartialEq)]
pub enum GameState {
    Loading,
    MainMenu,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// This structure contains the entire game state, and should be passed around to various logic functions.
pub struct GameCore {
    /// The game's overall state
    pub state: GameState,
    pub last_state_change_time: f64,
    pub has_rendered_first_frame: bool,

    /// Resources
    pub resources: GlobalResources,
}

impl GameCore {
    pub fn new(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            state: GameState::Loading,
            last_state_change_time: 0.0,
            has_rendered_first_frame: false,
            resources: GlobalResources::load_all(raylib, thread).expect("Failed to load game assets. Can not launch!"),
        }
    }
}
