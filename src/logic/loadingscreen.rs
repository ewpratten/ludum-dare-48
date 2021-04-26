use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::{utils::calculate_linear_slide, wrappers::audio::player::AudioPlayer},
};

use super::screen::Screen;

const SECONDS_PER_LOGO: f64 = 4.0;

#[derive(Debug, PartialEq)]
enum LoadingScreenState {
    GameLogo,
    RaylibLogo,
    Finished,
}

pub struct LoadingScreen {
    state: LoadingScreenState,
    last_state_switch_time: f64,
}

impl LoadingScreen {
    pub fn new() -> Self {
        Self {
            state: LoadingScreenState::GameLogo,
            last_state_switch_time: 0.0,
        }
    }

    fn get_logo_mask(&self, playthrough_percent: f64) -> Color {
        // Determine the alpha
        let alpha = calculate_linear_slide(playthrough_percent);

        // Build a color mask
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: (255.0 * alpha) as u8,
        }
    }

    fn show_game_logo(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        game_core: &mut GameCore,
        win_height: i32,
        win_width: i32,
    ) {
        // // Determine how far through rendering this logo we are
        // // This value is used to determine the logo alpha
        // let playthrough_percent =
        //     (draw_handle.get_time() - self.last_state_switch_time) / SECONDS_PER_LOGO;

        // // Build a color mask
        // let mask = self.get_logo_mask(playthrough_percent);

        // // Get the logo
        // let logo = &game_core.resources.game_logo;

        // // Render the logo
        // draw_handle.draw_texture(
        //     logo,
        //     (win_width / 2) - (logo.width / 2),
        //     (win_height / 2) - (logo.height / 2),
        //     mask,
        // );

        // Move on to next logo if needed
        // if playthrough_percent >= 1.0 {
            self.state = LoadingScreenState::RaylibLogo;
            self.last_state_switch_time = draw_handle.get_time();
        // }
    }

    fn show_raylib_logo(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        _game_core: &mut GameCore,
        win_height: i32,
        win_width: i32,
    ) {
        // Determine how far through rendering this logo we are
        // This value is used to determine the logo alpha
        let playthrough_percent =
            (draw_handle.get_time() - self.last_state_switch_time) / SECONDS_PER_LOGO;

        // Build a color mask
        let mask = self.get_logo_mask(playthrough_percent);

        // Create modified colors
        let alpha_black = Color {
            r: 0,
            g: 0,
            b: 0,
            a: mask.a,
        };

        // Render the raylib logo
        draw_handle.draw_rectangle(
            win_width / 2 - 128,
            win_height / 2 - 128,
            256,
            256,
            alpha_black,
        );
        draw_handle.draw_rectangle(
            win_width / 2 - 112,
            win_height / 2 - 112,
            224,
            224,
            Color::WHITE,
        );
        // draw_handle.draw_text(
        //     "rust",
        //     win_width / 2 - 69,
        //     win_height / 2 + 18,
        //     50,
        //     alpha_orange,
        // );
        draw_handle.draw_text(
            "raylib",
            win_width / 2 - 44,
            win_height / 2 + 48,
            50,
            alpha_black,
        );

        // Move on to next logo if needed
        if playthrough_percent >= 1.0 {
            self.state = LoadingScreenState::Finished;
            self.last_state_switch_time = draw_handle.get_time();
        }
    }
}

impl Screen for LoadingScreen {
    fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        _audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
        // Clear frame
        draw_handle.clear_background(Color::WHITE);

        // Window dimensions
        let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();

        // Call the appropriate internal handler function
        match self.state {
            LoadingScreenState::GameLogo => {
                self.show_game_logo(draw_handle, game_core, win_height, win_width)
            }
            LoadingScreenState::RaylibLogo => {
                self.show_raylib_logo(draw_handle, game_core, win_height, win_width)
            }
            LoadingScreenState::Finished => return Some(GameState::MainMenu),
        }

        // A DEBUG warning and skip button
        #[cfg(debug_assertions)]
        {
            // Render debug text
            draw_handle.draw_text("RUNNING IN DEBUG MODE", 0, 0, 20, Color::RED);
            draw_handle.draw_text("Press ESC to skip this screen", 0, 25, 20, Color::RED);

            if draw_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                return Some(GameState::MainMenu);
            }
        }

        return None;
    }
}
