mod entities;
mod gamecore;
mod items;
mod lib;
mod logic;
mod pallette;
mod player;
mod resources;
mod world;

use gamecore::{GameCore, GameProgress, GameState};
use lib::{utils::profiler::GameProfiler, wrappers::audio::player::AudioPlayer};
use logic::{
    gameend::GameEndScreen, ingame::InGameScreen, loadingscreen::LoadingScreen,
    mainmenu::MainMenuScreen, pausemenu::PauseMenuScreen, screen::Screen, shop::ShopScreen,
    winscreen::WinScreen,
};
use raylib::prelude::*;
use world::{load_world_colliders, World};

// Game Launch Configuration
const DEFAULT_WINDOW_DIMENSIONS: Vector2 = Vector2 {
    x: 1080.0,
    y: 720.0,
};
const WINDOW_TITLE: &str = r"One Breath";
const MAX_FPS: u32 = 60;

fn main() {
    // Configure a window
    let (mut raylib, raylib_thread) = raylib::init()
        .size(
            DEFAULT_WINDOW_DIMENSIONS.x as i32,
            DEFAULT_WINDOW_DIMENSIONS.y as i32,
        )
        .msaa_4x()
        .title(WINDOW_TITLE)
        .build();
    raylib.set_target_fps(MAX_FPS);

    // Override the default exit key
    raylib.set_exit_key(None);

    // Load the world
    let world_colliders = load_world_colliders("./assets/img/map/cave.json".to_string())
        .expect("Failed to load world colliders");
    let world = World::load_from_json(
        "./assets/worlds/mainworld.json".to_string(),
        world_colliders,
    )
    .expect("Failed to load main world JSON");

    // Load the game progress
    let game_progress = GameProgress::try_from_file("./savestate.json".to_string());

    // Set up the game's core state
    let mut game_core = GameCore::new(&mut raylib, &raylib_thread, world, game_progress);
    game_core.player.inventory = game_core.progress.inventory.clone();
    game_core.player.coins = game_core.progress.coins;

    // Set up the game's profiler
    let mut profiler = GameProfiler::new();
    profiler.start();

    // Init the audio subsystem
    let mut audio_system = AudioPlayer::new(RaylibAudio::init_audio_device());

    // Create all the game screens
    let mut loading_screen = LoadingScreen::new();
    let mut main_menu_screen = MainMenuScreen::new();
    let mut pause_menu_screen = PauseMenuScreen::new();
    let mut ingame_screen;
    unsafe {
        ingame_screen = InGameScreen::new(&game_core);
    }
    let mut game_end_screen = GameEndScreen::new();
    let mut shop_screen = ShopScreen::new();
    let mut win_screen = WinScreen::new();

    // Main rendering loop
    while !raylib.window_should_close() {
        let mut draw_handle = raylib.begin_drawing(&raylib_thread);

        // Call appropriate render function
        let new_state: Option<GameState> = match game_core.state {
            GameState::Loading => loading_screen.render(
                &mut draw_handle,
                &raylib_thread,
                &mut audio_system,
                &mut game_core,
            ),
            GameState::MainMenu => main_menu_screen.render(
                &mut draw_handle,
                &raylib_thread,
                &mut audio_system,
                &mut game_core,
            ),
            GameState::PauseMenu => pause_menu_screen.render(
                &mut draw_handle,
                &raylib_thread,
                &mut audio_system,
                &mut game_core,
            ),
            GameState::GameQuit => None,
            GameState::InGame => ingame_screen.render(
                &mut draw_handle,
                &raylib_thread,
                &mut audio_system,
                &mut game_core,
            ),
            GameState::GameEnd => game_end_screen.render(
                &mut draw_handle,
                &raylib_thread,
                &mut audio_system,
                &mut game_core,
            ),
            GameState::InShop => shop_screen.render(
                &mut draw_handle,
                &raylib_thread,
                &mut audio_system,
                &mut game_core,
            ),
            GameState::WinGame => win_screen.render(
                &mut draw_handle,
                &raylib_thread,
                &mut audio_system,
                &mut game_core,
            ),
        };

        // If needed, update the global state
        if new_state.is_some() {
            let new_state = new_state.unwrap();

            // Handle game quit
            if new_state == GameState::GameQuit {
                // Save the game state
                let new_progress = game_core
                    .player
                    .create_statistics(&game_core, draw_handle.get_time());
                game_core.progress.update(&new_progress);

                // Break the render loop
                break;
            }

            game_core.switch_state(new_state, Some(&draw_handle));
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
            profiler.data.player_coins = game_core.player.coins;
            profiler.data.player_boost_percent = game_core.player.boost_percent;
            profiler.data.player_breath_percent = game_core.player.breath_percent;
            profiler.data.player_pose = game_core.player.position;

            // Send telemetry data
            profiler.update();
        }

        // Debug key
        if draw_handle.is_key_pressed(KeyboardKey::KEY_F3) {
            game_core.show_simple_debug_info = !game_core.show_simple_debug_info;
        }

        // Handle showing some simple debug info if needed
        if game_core.show_simple_debug_info {
            draw_handle.draw_text(
                &format!("FPS: {}", draw_handle.get_fps()),
                0,
                0,
                20,
                Color::RED,
            );
            #[cfg(debug_assertions)]
            draw_handle.draw_text("DEBUG BUILD", 0, 20, 20, Color::RED);
        }

        // Set the first frame flag
        game_core.has_rendered_first_frame = true;

        // Update the frame time
        game_core.last_frame_time = draw_handle.get_time();
    }

    // Save the game state
    let new_progress = game_core
        .player
        .create_statistics(&game_core, raylib.get_time());
    game_core.progress.update(&new_progress);

    // Cleanup
    profiler.stop();
}
