mod lib;
mod gamecore;

use gamecore::{GameCore, GameState};
use raylib::prelude::*;

// Game Launch Configuration
const DEFAULT_WINDOW_DIMENSIONS: Vector2 = Vector2 { x: 800.0, y: 600.0 };
const WINDOW_TITLE: &str = r"Ludum Dare 48";
const MAX_FPS: u32 = 60;

fn main() {
    // Configure a window
    let (mut raylib, raylib_thread) = raylib::init()
        .size(
            DEFAULT_WINDOW_DIMENSIONS.x as i32,
            DEFAULT_WINDOW_DIMENSIONS.y as i32,
        )
        .title(WINDOW_TITLE)
        .build();
    raylib.set_target_fps(MAX_FPS);

    // Set up the game's core state
    let mut game_core = GameCore{
        state: GameState::Loading
    };

    // Main rendering loop
    while !raylib.window_should_close() {
        let mut draw_handle = raylib.begin_drawing(&raylib_thread);

        // Clear frame
        draw_handle.clear_background(Color::WHITE);

        // Call appropriate render function
        // TODO: the usual match statement on `game_core.state`
    }
}
