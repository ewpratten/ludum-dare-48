use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::{utils::button::OnScreenButton, wrappers::audio::player::AudioPlayer},
};

use super::screen::Screen;

const SCREEN_PANEL_SIZE: Vector2 = Vector2 { x: 300.0, y: 380.0 };

pub struct PauseMenuScreen {}

impl PauseMenuScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for PauseMenuScreen {
    fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
        let mouse_position = draw_handle.get_mouse_position();
        // draw_handle.clear_background(Color::GRAY);
        // // TODO: Maybe we can stick some art here?

        // Render the background
        draw_handle.draw_texture(&game_core.resources.shop_background, 0, 0, Color::WHITE);

        // If escape is pressed again, return to the previous render state
        if draw_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return Some(game_core.last_state);
        }

        // Window dimensions
        let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();

        // Render the backing to the menu itself
        draw_handle.draw_rectangle(
            (win_width / 2) - ((SCREEN_PANEL_SIZE.x as i32 + 6) / 2),
            (win_height / 2) - ((SCREEN_PANEL_SIZE.y as i32 + 6) / 2),
            SCREEN_PANEL_SIZE.x as i32 + 6,
            SCREEN_PANEL_SIZE.y as i32 + 6,
            Color::BLACK,
        );
        draw_handle.draw_rectangle(
            (win_width / 2) - (SCREEN_PANEL_SIZE.x as i32 / 2),
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2),
            SCREEN_PANEL_SIZE.x as i32,
            SCREEN_PANEL_SIZE.y as i32,
            Color::WHITE,
        );

        // Render heading text
        draw_handle.draw_text(
            "PAUSED",
            (win_width / 2) - 80,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 10,
            40,
            Color::BLACK,
        );

        // Render volume select
        draw_handle.draw_text(
            "Volume:",
            (win_width / 2) - (SCREEN_PANEL_SIZE.x as i32 / 2) + 10,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 60,
            20,
            Color::BLACK,
        );

        // Determine the slider position based on volume and screen dimensions
        let slider_base_x = (SCREEN_PANEL_SIZE.x - 30.0) * audio_system.get_master_volume();
        let slider_grab_bounds = Rectangle {
            x: (win_width as f32 / 2.0) - (SCREEN_PANEL_SIZE.x / 2.0) + 10.0 + slider_base_x,
            y: (win_height as f32 / 2.0) - (SCREEN_PANEL_SIZE.y / 2.0) + 85.0,
            width: 10.0,
            height: 20.0,
        };
        let slider_left_x = (win_width / 2) - (SCREEN_PANEL_SIZE.x as i32 / 2) + 10;
        let slider_right_x = slider_left_x + SCREEN_PANEL_SIZE.x as i32 - 20;

        // Render the slider
        draw_handle.draw_rectangle(
            slider_left_x,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 90,
            SCREEN_PANEL_SIZE.x as i32 - 20,
            5,
            Color::GRAY,
        );
        draw_handle.draw_rectangle(
            slider_grab_bounds.x as i32,
            slider_grab_bounds.y as i32,
            slider_grab_bounds.width as i32,
            slider_grab_bounds.height as i32,
            Color::BLACK,
        );

        // Handle user interaction with the slider
        if draw_handle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            // Check if the cursor X is near the volume slider
            if (mouse_position.x as i32) > slider_left_x
                && (mouse_position.y as i32) < slider_right_x
            {
                if mouse_position.y > slider_grab_bounds.y
                    && mouse_position.y < slider_grab_bounds.y + slider_grab_bounds.height
                {
                    // Calculate the offset mouse position on the slider
                    let offset_x_position = mouse_position.x as i32 - slider_left_x;

                    // Set the volume
                    audio_system
                        .set_master_volume(offset_x_position as f32 / (SCREEN_PANEL_SIZE.x - 20.0));
                }
            }
        }

        // Render credits
        draw_handle.draw_text(
            "Credits:\n\t- @ewpratten\n\t- @rsninja722\n\t- @wm-c\n\t- @catarinaburghi\n\t- @kondroel",
            (win_width / 2) - (SCREEN_PANEL_SIZE.x as i32 / 2) + 10,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 150,
            20,
            Color::BLACK,
        );

        // Bottom buttons

        let bottom_left_button_dimensions = Rectangle {
            x: (win_width as f32 / 2.0) - (SCREEN_PANEL_SIZE.x / 2.0) + 5.0,
            y: (win_height as f32 / 2.0) + (SCREEN_PANEL_SIZE.y / 2.0) - 50.0,
            width: (SCREEN_PANEL_SIZE.x / 2.0) - 15.0,
            height: 40.0,
        };
        let bottom_right_button_dimensions = Rectangle {
            x: (win_width as f32 / 2.0) + 5.0,
            y: bottom_left_button_dimensions.y,
            width: bottom_left_button_dimensions.width,
            height: bottom_left_button_dimensions.height,
        };

        let menu_button = OnScreenButton::new(
            "Menu".to_string(),
            bottom_left_button_dimensions,
            Color::WHITE,
            Color::BLACK,
            Color::GRAY,
            30,
            true,
        );
        let close_button = OnScreenButton::new(
            "Close".to_string(),
            bottom_right_button_dimensions,
            Color::WHITE,
            Color::BLACK,
            Color::GRAY,
            30,
            true,
        );

        // Render both
        menu_button.render(draw_handle);
        close_button.render(draw_handle);

        // Handle click actions on the buttons
        if draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            if menu_button.is_hovered(draw_handle) {
                return Some(GameState::MainMenu);
            } else if close_button.is_hovered(draw_handle) {
                return Some(game_core.last_state);
            }
        }

        return None;
    }
}
