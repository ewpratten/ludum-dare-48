use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
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
        draw_handle.clear_background(Color::GRAY);
        // TODO: Maybe we can stick some art here?

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
            "Credits:\n\t- @ewpratten\n\t- @rsninja722\n\t- @wm-c\n\t- @catarinaburghi",
            (win_width / 2) - (SCREEN_PANEL_SIZE.x as i32 / 2) + 10,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 120,
            20,
            Color::BLACK,
        );

        // Close and quit buttons
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

        // Check if the mouse is over either button
        let mouse_over_bottom_left_button =
            bottom_left_button_dimensions.check_collision_point_rec(mouse_position);
        let mouse_over_bottom_right_button =
            bottom_right_button_dimensions.check_collision_point_rec(mouse_position);

        // Render buttons
        draw_handle.draw_rectangle_lines_ex(
            bottom_left_button_dimensions,
            3,
            match mouse_over_bottom_left_button {
                true => Color::GRAY,
                false => Color::BLACK,
            },
        );
        draw_handle.draw_text(
            "Menu",
            bottom_left_button_dimensions.x as i32 + 15,
            bottom_left_button_dimensions.y as i32 + 5,
            30,
            Color::BLACK,
        );
        draw_handle.draw_rectangle_lines_ex(
            bottom_right_button_dimensions,
            3,
            match mouse_over_bottom_right_button {
                true => Color::GRAY,
                false => Color::BLACK,
            },
        );
        draw_handle.draw_text(
            "Close",
            bottom_right_button_dimensions.x as i32 + 15,
            bottom_right_button_dimensions.y as i32 + 5,
            30,
            Color::BLACK,
        );

        // Handle click actions on the buttons
        if draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            if mouse_over_bottom_left_button {
                return Some(GameState::MainMenu);
            } else if mouse_over_bottom_right_button {
                return Some(game_core.last_state);
            }
        }

        return None;
    }
}
