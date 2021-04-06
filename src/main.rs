mod lib;

use lib::{animation::wrapper::FrameAnimationWrapper, audio::wrapper::AudioWrapper};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("LDJam48 Tech Demo")
        .build();
    rl.set_target_fps(120);

    // Set up audio
    let mut audio = RaylibAudio::init_audio_device();
    let mut distraction_dance_sound = AudioWrapper::new(
        Music::load_music_stream(&thread, "assets/audio/distraction_dance.mp3").unwrap(),
    );

    // Build an animation
    let texture = 
        rl.load_texture(&thread, "assets/img/chr_henryStickman/spritesheet.png")
            .unwrap();
    let mut stickman_animation =
        FrameAnimationWrapper::new(texture, Vector2 { x: 472.0, y: 562.0 }, 44, 24);

    // State
    let mut is_playing_dance = false;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Clear frame
        d.clear_background(Color::WHITE);

        // Update the audio buffer
        distraction_dance_sound.update(&mut audio);

        // Draw an animation frame
        stickman_animation.draw(&mut d, Vector2 { x: 200.0, y: 0.0 });

        // Begin the dance if needed
        if !is_playing_dance {
            // Play audio
            distraction_dance_sound.play(&mut audio);

            // Play animation
            stickman_animation.start(&d);

            is_playing_dance = true;
        }

        // Reset the loop at the end of the track
        if !distraction_dance_sound.is_playing(&mut audio) {
            // Stop animation
            stickman_animation.stop();

            is_playing_dance = false;
        }

        // Render debug info
        d.draw_text("Tech Demo - do not redistribute", 10, 10, 20, Color::BLACK);
        let fps = d.get_fps();
        d.draw_text(&format!("FPS: {}/120", fps), 10, 25, 20, Color::BLACK);
        let spf = d.get_frame_time();
        d.draw_text(&format!("SPF: {}", spf), 10, 40, 20, Color::BLACK);
        let dpi = d.get_window_scale_dpi();
        d.draw_text(&format!("DPI: {:?}", dpi), 10, 55, 20, Color::BLACK);
        let frame = stickman_animation.get_current_frame_id(&d);
        d.draw_text(&format!("Frame: {}", frame), 10, 70, 20, Color::BLACK);
    }
}
