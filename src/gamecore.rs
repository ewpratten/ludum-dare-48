//! This file contains the global state of the game. Data here is passed around to all handler functions.

use std::fmt;

use raylib::{
    camera::Camera2D, math::Vector2, prelude::RaylibDrawHandle, RaylibHandle, RaylibThread,
};

use crate::{player::Player, resources::GlobalResources, world::World};

use log::debug;

/// Overall states for the game
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    Loading,
    MainMenu,
    PauseMenu,
    GameQuit,
    InGame,
    GameEnd
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
}

impl GameCore {
    pub fn new(raylib: &mut RaylibHandle, thread: &RaylibThread, world: World) -> Self {
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
                target: Vector2::zero(),
                rotation: 0.0,
                zoom: 1.0,
            },
            show_simple_debug_info: false,
            world: world,
            player: Player::new(),
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
