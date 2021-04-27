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
        _thread: &RaylibThread,
        _audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
        // Window dimensions
        let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();

        // // Clear frame
        // draw_handle.clear_background(Color::BLUE);
        // Render the background
        draw_handle.draw_texture(&game_core.resources.shop_background, 0, 0, Color::WHITE);

        // Render title
        draw_handle.draw_text(
            "DEEP BREATH",
            (win_height / 2) - 100,
            win_width / 8,
            80,
            Color::BLACK,
        );

        // Get mouse position data
        let mouse_position = draw_handle.get_mouse_position();
        let hovering_play_button = mouse_position.y > (win_width as f32 / 4.0)
            && mouse_position.y < (win_width as f32 / 4.0) + 60.0;
        let hovering_shop_button = mouse_position.y > (win_width as f32 / 4.0) + 100.0
            && mouse_position.y < (win_width as f32 / 4.0) + 160.0;
        let hovering_quit_button = mouse_position.y > (win_width as f32 / 4.0) + 200.0
            && mouse_position.y < (win_width as f32 / 4.0) + 260.0;

        // Play and quit
        draw_handle.draw_text(
            "Play",
            (win_height / 2) + 120,
            win_width / 4,
            60,
            match hovering_play_button {
                true => Color::BLUE,
                false => Color::BLACK,
            },
        );
        draw_handle.draw_text(
            "Shop",
            (win_height / 2) + 120,
            (win_width / 4) + 100,
            60,
            match hovering_shop_button {
                true => Color::BLUE,
                false => Color::BLACK,
            },
        );
        draw_handle.draw_text(
            "Quit",
            (win_height / 2) + 130,
            (win_width / 4) + 200,
            60,
            match hovering_quit_button {
                true => Color::RED,
                false => Color::BLACK,
            },
        );

        // Handle button presses
        let mouse_clicked = draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);

        // Check clicks
        if mouse_clicked {
            if hovering_play_button {
                // Reset the world
                game_core.world.reset(&mut game_core.player);
                _audio_system.play_sound(&game_core.resources.ui_click);
                // Start playing
                return Some(GameState::InGame);
            } else if hovering_shop_button {
                _audio_system.play_sound(&game_core.resources.ui_click);
                return Some(GameState::InShop);
            } else if hovering_quit_button {
                return Some(GameState::GameQuit);
            }
        }

        return None;
    }
}
