mod hud;
mod playerlogic;

use raylib::prelude::*;

use crate::{
    entities::enemy::base::EnemyBase,
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
    pallette::{SKY, WATER, WATER_DARK},
};

use super::screen::Screen;

pub enum InGameState {
    BUYING,
    SWIMMING,
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

    fn render_world(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        game_core: &mut GameCore,
        dt: f64,
    ) {
        // Build source bounds
        let source_bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: game_core.resources.cave_mid_layer.width as f32,
            height: game_core.resources.cave_mid_layer.height as f32,
        };
        let world_bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: game_core.resources.cave_mid_layer.width as f32,
            height: game_core.resources.cave_mid_layer.height as f32,
        };

        // Clear the background
        context_2d.draw_rectangle_gradient_v(
            world_bounds.x as i32,
            world_bounds.y as i32,
            world_bounds.width as i32,
            world_bounds.height as i32,
            WATER,
            WATER_DARK,
        );

        // Render fish
        let fish_clone = game_core.world.fish.clone();
        for fish in game_core.world.fish.iter_mut() {
            fish.update_position(&mut game_core.player, dt, &fish_clone);
            fish.render(context_2d, &mut game_core.resources);
        }

        // Render the world texture
        context_2d.draw_texture_rec(
            &game_core.resources.cave_mid_layer,
            source_bounds,
            Vector2 {
                x: world_bounds.x,
                y: world_bounds.y,
            },
            Color::WHITE,
        );
    }

    fn render_colliders(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        game_core: &mut GameCore,
    ) {
        // Render every collider
        for collider in game_core.world.colliders.iter() {
            context_2d.draw_rectangle_lines_ex(collider, 1, Color::RED);
        }
    }

    fn render_darkness(&mut self, draw_handle: &mut RaylibDrawHandle, game_core: &mut GameCore) {
        // Calculate the min view radius based on the current flashlight
        let mut min_radius = 0.0;
        if game_core.player.inventory.flashlight.is_some() {
            min_radius = game_core
                .player
                .inventory
                .flashlight
                .as_ref()
                .unwrap()
                .radius;
        }

        // Get the window center
        let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();

        // Calculate the occusion radius based on depth
        let radius = (1.0
            - (game_core.player.calculate_depth_percent(&game_core.world) * 1.3).clamp(0.0, 1.0))
        .max(min_radius);

        // Determine width and height scales
        // This is clamped to make the rendering logic below easier by removing the need to overdraw
        let width_scale = (5.0 * radius).max(0.5);
        let height_scale = (5.0 * radius).max(0.5);

        // Get the base sizes of everything
        let texture_width = game_core.resources.darkness_overlay.width as f32;
        let texture_height = game_core.resources.darkness_overlay.height as f32;
        let texture_width_scaled = texture_width * width_scale;
        let texture_height_scaled = texture_height * height_scale;

        // Render the overlay
        draw_handle.draw_texture_pro(
            &game_core.resources.darkness_overlay,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: texture_width,
                height: texture_height,
            },
            Rectangle {
                x: (win_width as f32 - texture_width_scaled) / 2.0,
                y: (win_height as f32 - texture_height_scaled) / 2.0,
                width: texture_width_scaled,
                height: texture_height_scaled,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
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
        // Calculate DT
        let dt = draw_handle.get_time() - game_core.last_frame_time;

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
        let camera_window_center = window_center * (1.0 / game_core.master_camera.zoom);

        // Update player movement
        playerlogic::update_player_movement(draw_handle, game_core, window_center);

        // Open a 2D context
        {
            unsafe {
                raylib::ffi::BeginTextureMode(*game_core.resources.shader_texture);
            }
            {
                let mut context_2d = draw_handle.begin_mode2D(game_core.master_camera);

                // Clear frame
                context_2d.clear_background(Color::BLACK);

                // Render the world
                self.render_world(&mut context_2d, game_core, dt);
                if game_core.show_simple_debug_info {
                    self.render_colliders(&mut context_2d, game_core);
                }

                // Render entities
                for jellyfish in game_core.world.jellyfish.iter_mut() {
                    jellyfish.handle_logic(&mut game_core.player, dt);
                    jellyfish.render(
                        &mut context_2d,
                        &mut game_core.player,
                        &mut game_core.resources,
                        dt,
                    );
                }
                for octopus in game_core.world.octopus.iter_mut() {
                    octopus.handle_logic(&mut game_core.player, dt);
                    octopus.render(
                        &mut context_2d,
                        &mut game_core.player,
                        &mut game_core.resources,
                        dt,
                    );
                }

                // Render Player
                game_core
                    .player
                    .render(&mut context_2d, &mut game_core.resources, dt);
            }
            unsafe {
                raylib::ffi::EndTextureMode();
            }
        }

        // Render the 2D context via the ripple shader
        {
            let mut shader_context =
                draw_handle.begin_shader_mode(&game_core.resources.pixel_shader);

            // Blit the texture
            shader_context.draw_texture_rec(
                &game_core.resources.shader_texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: game_core.resources.shader_texture.width() as f32,
                    height: (game_core.resources.shader_texture.height() as f32) * -1.0,
                },
                Vector2::zero(),
                Color::WHITE,
            );
        }

        // Render the darkness layer
        self.render_darkness(draw_handle, game_core);

        // Render the hud
        hud::render_hud(draw_handle, game_core, window_center);

        // Handle player out of breath
        if game_core.player.breath_percent == 0.0 {
            return Some(GameState::GameEnd);
        }

		if game_core.world.end_position.distance_to(game_core.player.position) <= 70.0{
			return Some(GameState::WinGame);
		}

        return None;
    }
}
