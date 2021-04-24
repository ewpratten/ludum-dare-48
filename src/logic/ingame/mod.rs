use nalgebra::distance;
use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
};

use super::screen::Screen;

const NORMAL_PLAYER_SPEED: i32 = 4;
const BOOST_PLAYER_SPEED: i32 = NORMAL_PLAYER_SPEED * 2;
const CAMERA_FOLLOW_SPEED: f32 = 3.0;

pub enum InGameState {
    BUYING,
    SWIMMING,
    DEAD,
}

pub struct InGameScreen {
    current_state: InGameState,
}

impl InGameScreen {
    pub fn new() -> Self {
        Self {
            current_state: InGameState::SWIMMING,
        }
    }

    fn update_player_movement(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        game_core: &mut GameCore,
    ) {
        let player_screen_position =
            draw_handle.get_screen_to_world2D(game_core.player.position, game_core.master_camera);
		

        // Handle player movement
        let mouse_pose = draw_handle.get_mouse_position();
        let mut raw_movement_direction = mouse_pose - player_screen_position;
        raw_movement_direction.normalize();
        game_core.player.direction = raw_movement_direction;

        // Handle action buttons
        let user_request_boost =
            draw_handle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);
        let user_request_action =
            draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON);

        // Move the player in their direction
        let speed_multiplier = match user_request_boost && game_core.player.boost_percent >= 0.0 {
            true => BOOST_PLAYER_SPEED as f32,
            false => NORMAL_PLAYER_SPEED as f32
        };
        

        // Move the camera to follow the player
        let direction_from_cam_to_player = player_screen_position;

		if game_core.player.position.distance_to(mouse_pose) <= 20.0{
			return;
		}

		game_core.player.position += game_core.player.direction * speed_multiplier;
        // game_core.master_camera.offset -= direction_from_cam_to_player * CAMERA_FOLLOW_SPEED;
        //game_core.master_camera.target = game_core.player.position + 0;
        
    }

    fn render_player(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        game_core: &mut GameCore,
    ) {
        // Get the player
        let player = &game_core.player;

        // TODO: tmp rect
        context_2d.draw_rectangle(
            player.position.x as i32 - 10,
            player.position.y as i32 - 10,
            20,
            30,
            Color::BLACK,
        );
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

        // Handle the pause menu being opened
        if draw_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return Some(GameState::PauseMenu);
        }

        // Window dimensions
        let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();
        let window_center = Vector2 {
            x: (win_width as f32 / 2.0),
            y: (win_height as f32 / 2.0),
        };

        // Update player movement
        self.update_player_movement(draw_handle, game_core);

        // Open a 2D context
        {
            let mut context_2d = draw_handle.begin_mode2D(game_core.master_camera);

            // Render Player
            self.render_player(&mut context_2d, game_core);
        }

        return None;
    }
}
