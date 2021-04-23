use raylib::prelude::RaylibDrawHandle;

use crate::{gamecore::{GameCore, GameState}, lib::wrappers::audio::player::AudioPlayer};

/// A trait describing all game screens
pub trait Screen {
    fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState>;
}
