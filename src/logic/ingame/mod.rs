mod hud;
mod playerlogic;

use raylib::prelude::*;
use crate::{entities::enemy::{base::EnemyBase, whirlpool::Whirlpool}, gamecore::{self, GameCore, GameState}, lib::wrappers::audio::player::AudioPlayer};

use super::screen::Screen;
use crate::entities::fish::FishEntity;

pub struct InGameScreen {
    shader_time_var_location: i32,
    swim_playing: bool,
}

impl InGameScreen {
    pub unsafe fn new(game_core: &GameCore) -> Self {
        Self {
            shader_time_var_location: raylib::ffi::GetShaderLocation(
                *game_core.resources.pixel_shader,
                rstr!("time").as_ptr(),
            ),
            swim_playing: false,
        }
    }

    fn render_world(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        game_core: &mut GameCore,
        dt: f64,
        audio_system: &mut AudioPlayer,
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
        // context_2d.draw_rectangle_gradient_v(
        //     world_bounds.x as i32,
        //     world_bounds.y as i32,
        //     world_bounds.width as i32,
        //     world_bounds.height as i32,
        //     WATER,
        //     WATER_DARK,
        // );
        context_2d.draw_texture_pro(
            &game_core.resources.background_back,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: game_core.resources.background_back.width as f32,
                height: game_core.resources.background_back.height as f32,
            },
            Rectangle::new(
                0.0,
                0.0,
                (game_core.resources.background_back.width * 2) as f32,
                (game_core.resources.background_back.height * 2) as f32,
            ),
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
        context_2d.draw_texture_pro(
            &game_core.resources.background_front,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: game_core.resources.background_front.width as f32,
                height: game_core.resources.background_front.height as f32,
            },
            Rectangle::new(
                0.0,
                0.0,
                (game_core.resources.background_front.width * 2) as f32,
                (game_core.resources.background_front.height * 2) as f32,
            ),
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );

        // Render fish
        let fish_clone = game_core.world.fish.clone();
        for fish in game_core.world.fish.iter_mut() {
            if fish.update_position(&mut game_core.player, dt, &fish_clone) {
                audio_system.play_sound(&game_core.resources.fish_pickup);
            }
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
        _thread: &RaylibThread,
        _audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
        // Calculate DT
        let dt = draw_handle.get_time() - game_core.last_frame_time;

        // Handle the pause menu being opened
        if draw_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return Some(GameState::PauseMenu);
        }

        // music
        if !_audio_system.is_sound_playing(&game_core.resources.song_swim) {
            _audio_system.play_sound(&game_core.resources.song_swim);
        }

        // Window dimensions
        let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();
        let window_center = Vector2 {
            x: (win_width as f32 / 2.0),
            y: (win_height as f32 / 2.0),
        };

        // Update player movement
        playerlogic::update_player_movement(draw_handle, game_core, window_center, _audio_system);

        if draw_handle.get_time() % 10.0 <= 0.1 && !_audio_system.is_sound_playing(&game_core.resources.breath){
            _audio_system.set_sound_volume(&game_core.resources.breath, 0.5);
            _audio_system.play_sound(&game_core.resources.breath);
        }

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
                self.render_world(&mut context_2d, game_core, dt, _audio_system);
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
                    if octopus.handle_logic(&mut game_core.player, dt) == 1 {
                        _audio_system.play_sound(&game_core.resources.succ);
                    }
                    octopus.render(
                        &mut context_2d,
                        &mut game_core.player,
                        &mut game_core.resources,
                        dt,
                    );
                }

				// Iterates over whirlpools and runs render and logic funcs
				for whirlpool_mob in game_core.world.whirlpool.iter_mut(){
					whirlpool_mob.handle_logic(&mut game_core.player, dt);
					whirlpool_mob.render(&mut context_2d, &mut game_core.player, &mut game_core.resources, dt);

					// Spawns 10 fish on spawn
					if whirlpool_mob.should_remove(){
						for _ in 0..10{
							game_core.world.fish.push(FishEntity::new(whirlpool_mob.position));
						}
					}
					
					
				}

				// Iterates over pufferfish
				for pufferfish in game_core.world.pufferfish.iter_mut(){

					pufferfish.handle_logic(&mut game_core.player, dt);
					pufferfish.render(&mut context_2d, &mut game_core.player, &mut game_core.resources, dt);



				}


				// Removes whirlpools set for removal
				game_core.world.whirlpool.retain(|x| !x.should_remove());
				



                // Render transponder
                game_core.resources.transponder.draw(
                    &mut context_2d,
                    game_core.world.end_position,
                    0.0,
                );

                // Render Player
                game_core
                    .player
                    .render(&mut context_2d, &mut game_core.resources, dt);
            }
            unsafe {
                raylib::ffi::EndTextureMode();
            }
        }

        // Update the shader's internal time
        game_core
            .resources
            .pixel_shader
            .set_shader_value(self.shader_time_var_location, draw_handle.get_time() as f32);

        // Render the 2D context via the ripple shader
        {
            let mut shader_context =
                draw_handle.begin_shader_mode(&game_core.resources.pixel_shader);

            // Blit the texture
            shader_context.draw_texture_pro(
                &game_core.resources.shader_texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: game_core.resources.shader_texture.width() as f32,
                    height: (game_core.resources.shader_texture.height() as f32) * -1.0,
                },
                Rectangle {
                    x: -10.0,
                    y: -10.0,
                    width: win_width as f32 + 20.0,
                    height: win_height as f32 + 20.0,
                },
                Vector2::zero(),
                0.0,
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

        if game_core
            .world
            .end_position
            .distance_to(game_core.player.position)
            <= 40.0
        {
            return Some(GameState::WinGame);
        }

        return None;
    }
}
