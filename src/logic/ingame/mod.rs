mod hud;
mod playerlogic;
mod shop;

use raylib::prelude::*;

use crate::{gamecore::{self, GameCore, GameState}, lib::wrappers::audio::player::AudioPlayer};

use self::shop::Shop;

use super::screen::Screen;

#[derive(PartialEq)]
pub enum InGameState {
    BUYING,
    SWIMMING,
}

pub struct InGameScreen {
    current_state: InGameState,
	shop: Shop,
}

impl InGameScreen {
    pub fn new() -> Self {
        Self {
            current_state: InGameState::BUYING,
			shop: Shop::new(),
        }
    }

    fn render_world(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        game_core: &mut GameCore,
    ) {
        context_2d.draw_circle(0, 0, 10.0, Color::BLACK);
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
        // Calculate DT
        let dt = draw_handle.get_time() - game_core.last_frame_time;

		// Window dimensions
		let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();
        let window_center = Vector2 {
            x: (win_width as f32 / 2.0),
            y: (win_height as f32 / 2.0),
        };

		// Creates items for shop
		if draw_handle.get_time() - game_core.last_state_change_time >= 0.05{
			self.shop.create_items(Vector2::new(win_width as f32, win_height as f32));
		}
		

		// Only render shop in shop period, otherwise allow player movement
		if draw_handle.get_time() - game_core.last_state_change_time >= 0.05 
			&& self.current_state == InGameState::BUYING{
				
			shop::render_shop(draw_handle, game_core, self);
			
		}else{
			// Update player movement
			playerlogic::update_player_movement(draw_handle, game_core, window_center);
		}

        // Clear frame
        draw_handle.clear_background(Color::BLUE);

        // Handle the pause menu being opened
        if draw_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return Some(GameState::PauseMenu);
        }

        
        

        
        

        // Open a 2D context
        {
            let mut context_2d = draw_handle.begin_mode2D(game_core.master_camera);

            // Render the world
            self.render_world(&mut context_2d, game_core);

            // Render entities
            let mut fish = &mut game_core.world.fish;
            for fish in fish.iter_mut() {
                fish.update_position(&game_core.player, dt);
                fish.render(&mut context_2d);
            }

            // Render Player
            playerlogic::render_player(&mut context_2d, game_core);
        }

        // Render the hud
        hud::render_hud(draw_handle, game_core, window_center);



        // Handle player out of breath
        if game_core.player.breath_percent == 0.0 {
            return Some(GameState::GameEnd);
        }

        return None;
    }
}
