use raylib::prelude::*;

use crate::gamecore::{GameCore, GameState};


pub fn handle_loading_screen(draw_handle: &mut RaylibDrawHandle, game_core: &mut GameCore) -> Option<GameState> {

    // Clear frame
    draw_handle.clear_background(Color::WHITE);


    return None;
}