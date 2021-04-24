use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
};

use super::screen::Screen;

const SCREEN_PANEL_SIZE: Vector2 = Vector2 { x: 300.0, y: 300.0 };

pub struct GameEndScreen {}

impl GameEndScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for GameEndScreen {
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
            "OUT OF BREATH",
            (win_width / 2) - 80,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 10,
            40,
            Color::BLACK,
        );


        // // Close and quit buttons
        // let bottom_left_button_dimensions = Rectangle {
        //     x: (win_width as f32 / 2.0) - (SCREEN_PANEL_SIZE.x / 2.0) + 5.0,
        //     y: (win_height as f32 / 2.0) + (SCREEN_PANEL_SIZE.y / 2.0) - 50.0,
        //     width: (SCREEN_PANEL_SIZE.x / 2.0) - 15.0,
        //     height: 40.0,
        // };
        // let bottom_right_button_dimensions = Rectangle {
        //     x: (win_width as f32 / 2.0) + 5.0,
        //     y: bottom_left_button_dimensions.y,
        //     width: bottom_left_button_dimensions.width,
        //     height: bottom_left_button_dimensions.height,
        // };

        // // Check if the mouse is over either button
        // let mouse_over_bottom_left_button =
        //     bottom_left_button_dimensions.check_collision_point_rec(mouse_position);
        // let mouse_over_bottom_right_button =
        //     bottom_right_button_dimensions.check_collision_point_rec(mouse_position);

        // // Render buttons
        // draw_handle.draw_rectangle_lines_ex(
        //     bottom_left_button_dimensions,
        //     3,
        //     match mouse_over_bottom_left_button {
        //         true => Color::GRAY,
        //         false => Color::BLACK,
        //     },
        // );
        // draw_handle.draw_text(
        //     "Quit",
        //     bottom_left_button_dimensions.x as i32 + 15,
        //     bottom_left_button_dimensions.y as i32 + 5,
        //     30,
        //     Color::BLACK,
        // );
        // draw_handle.draw_rectangle_lines_ex(
        //     bottom_right_button_dimensions,
        //     3,
        //     match mouse_over_bottom_right_button {
        //         true => Color::GRAY,
        //         false => Color::BLACK,
        //     },
        // );
        // draw_handle.draw_text(
        //     "Close",
        //     bottom_right_button_dimensions.x as i32 + 15,
        //     bottom_right_button_dimensions.y as i32 + 5,
        //     30,
        //     Color::BLACK,
        // );

        // // Handle click actions on the buttons
        // if draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        //     if mouse_over_bottom_left_button {
        //         return Some(GameState::GameQuit);
        //     } else if mouse_over_bottom_right_button {
        //         return Some(game_core.last_state);
        //     }
        // }

        return None;
    }
}