mod gamecore;
mod lib;
mod logic;
mod resources;

use gamecore::{GameCore, GameState};
use lib::{utils::profiler::GameProfiler, wrappers::audio::player::AudioPlayer};
use logic::{loadingscreen::handle_loading_screen, mainmenu::handle_main_menu};
use raylib::prelude::*;

// Game Launch Configuration
const DEFAULT_WINDOW_DIMENSIONS: Vector2 = Vector2 { x: 800.0, y: 600.0 };
const WINDOW_TITLE: &str = r"Ludum Dare 48";
const MAX_FPS: u32 = 60;

fn main() {
    // Configure a window
    let (mut raylib, raylib_thread) = raylib::init()
        .size(
            DEFAULT_WINDOW_DIMENSIONS.x as i32,
            DEFAULT_WINDOW_DIMENSIONS.y as i32,
        )
        .title(WINDOW_TITLE)
        .build();
    raylib.set_target_fps(MAX_FPS);

    // Override the default exit key
    raylib.set_exit_key(None);

    // Set up the game's core state
    let mut game_core = GameCore::new();

    // Set up the game's profiler
    let mut profiler = GameProfiler::new();
    profiler.start();

    // Init the audio subsystem
    let mut audio_system = AudioPlayer::new(RaylibAudio::init_audio_device());

    // Main rendering loop
    while !raylib.window_should_close() {
        let mut draw_handle = raylib.begin_drawing(&raylib_thread);

        // Call appropriate render function
        let new_state: Option<GameState> = match game_core.state {
            GameState::Loading => handle_loading_screen(&mut draw_handle, &mut game_core),
            GameState::MainMenu => handle_main_menu(&mut draw_handle, &mut game_core),
        };

        if new_state.is_some() {
            game_core.state = new_state.unwrap();
            game_core.last_state_change_time = draw_handle.get_time();
        }

        // Feed the profiler
        // This only runs in the dev profile.
        #[cfg(debug_assertions)]
        {
            // Update all data
            profiler.data.seconds_per_frame = draw_handle.get_frame_time();
            profiler.data.frames_per_second = draw_handle.get_fps();
            profiler.data.monitor_count = raylib::core::window::get_monitor_count();
            profiler.data.audio_volume = audio_system.get_master_volume();
            profiler.data.active_sounds = audio_system.get_sounds_playing();
            profiler.data.game_state = game_core.state.to_string();

            // Send telemetry data
            profiler.update();
        }
    }

    // Cleanup
    profiler.stop();
}
