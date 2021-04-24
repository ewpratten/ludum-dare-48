use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
};

use super::screen::Screen;

pub struct InGameScreen {}

impl InGameScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for InGameScreen {
    fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
        // Clear frame
        draw_handle.clear_background(Color::WHITE);

        return None;
    }
}