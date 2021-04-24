use raylib::prelude::*;

use crate::gamecore::GameCore;

pub fn render_hud(
    draw_handle: &mut RaylibDrawHandle,
    game_core: &mut GameCore,
    window_center: Vector2,
) {
    // Get the relevant data
    let breath = game_core.player.breath_percent;
    let dist_from_player_to_end = game_core
        .player
        .position
        .distance_to(game_core.world.end_position);
    let dist_from_start_to_end = Vector2::zero().distance_to(game_core.world.end_position);
    let progress = (dist_from_start_to_end - dist_from_player_to_end) / dist_from_start_to_end;

    // Render the base of the progress bar
    let progress_bar_rect = Rectangle {
        x: 20.0,
        y: (window_center.y * 2.0) - 20.0 - 40.0,
        width: (window_center.x * 2.0) - 40.0,
        height: 40.0,
    };
    draw_handle.draw_rectangle_rec(progress_bar_rect, Color::BLUE);
    draw_handle.draw_rectangle_lines_ex(progress_bar_rect, 6, Color::WHITE);

    // Render the slider of the progress bar
    let progress_bar_slider = Rectangle {
        x: (((window_center.x * 2.0) - 40.0) * progress.abs().clamp(0.0, 1.0)) + 10.0,
        y: (window_center.y * 2.0) - 20.0 - 50.0,
        width: 40.0,
        height: 60.0,
    };
    draw_handle.draw_rectangle_rec(progress_bar_slider, Color::BLUE);
    //TODO: This causes a render bug
    draw_handle.draw_rectangle_lines_ex(progress_bar_slider, 6, Color::WHITE);

    // TODO: Breath bar
    // TODO: Boost bar
}
