use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
};

use super::screen::Screen;

pub struct MainMenuScreen {}

impl MainMenuScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for MainMenuScreen {
    fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
        // Window dimensions
        let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();

        // Clear frame
        draw_handle.clear_background(Color::BLUE);

        // Render title
        draw_handle.draw_text(
            "ONE BREATH",
            (win_height / 2) - 80,
            win_width / 4,
            40,
            Color::BLACK,
        );

        // Play and quit
        draw_handle.draw_text(
            "Play",
            (win_height / 2) - 80,
            (win_width / 4) + 100,
            20,
            Color::BLACK,
        );
        draw_handle.draw_text(
            "Quit",
            (win_height / 2) - 80,
            (win_width / 4) + 140,
            20,
            Color::BLACK,
        );

        // Handle button presses
        let mouse_position = draw_handle.get_mouse_position();
        let mouse_clicked = draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);

        // Check clicks
        if mouse_clicked {
            if mouse_position.y > (win_width as f32 / 4.0) + 100.0
                && mouse_position.y < (win_width as f32 / 4.0) + 120.0
            {
                return Some(GameState::InGame);
            } else if mouse_position.y > (win_width as f32 / 4.0) + 140.0
                && mouse_position.y < (win_width as f32 / 4.0) + 180.0
            {
                return Some(GameState::GameQuit);
            }
        }

        return None;
    }
}
