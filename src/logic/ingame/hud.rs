use raylib::prelude::*;

use crate::{gamecore::GameCore, pallette::TRANSLUCENT_WHITE_96};

pub fn render_hud(
    draw_handle: &mut RaylibDrawHandle,
    game_core: &mut GameCore,
    window_center: Vector2,
) {
    // Get the relevant data
    let progress = game_core.player.calculate_depth_percent(&game_core.world);

    // Determine the progress slider position
    let slider_bound_height = 20.0;
    let progress_slider_position = Vector2 {
        x: window_center.x * 2.0,
        y: (((window_center.y * 2.0) - (slider_bound_height * 2.0)) * progress)
            + slider_bound_height,
    };

    // Render the base of the slider
    draw_handle.draw_rectangle(
        (progress_slider_position.x - slider_bound_height) as i32,
        (progress_slider_position.y - slider_bound_height / 2.0) as i32,
        slider_bound_height as i32,
        slider_bound_height as i32,
        TRANSLUCENT_WHITE_96,
    );
    draw_handle.draw_triangle(
        Vector2 {
            x: (progress_slider_position.x - slider_bound_height),
            y: (progress_slider_position.y - slider_bound_height / 2.0),
        },
        Vector2 {
            x: (progress_slider_position.x - slider_bound_height - (slider_bound_height / 2.0)),
            y: progress_slider_position.y,
        },
        Vector2 {
            x: (progress_slider_position.x - slider_bound_height),
            y: (progress_slider_position.y + slider_bound_height / 2.0),
        },
        TRANSLUCENT_WHITE_96,
    );

    // Render the outline of the slider
    draw_handle.draw_line_ex(
        Vector2 {
            x: (progress_slider_position.x - slider_bound_height),
            y: (progress_slider_position.y - slider_bound_height / 2.0),
        },
        Vector2 {
            x: progress_slider_position.x,
            y: (progress_slider_position.y - slider_bound_height / 2.0),
        },
        3.0,
        Color::BLACK,
    );
    draw_handle.draw_line_ex(
        Vector2 {
            x: (progress_slider_position.x - slider_bound_height),
            y: (progress_slider_position.y + slider_bound_height / 2.0),
        },
        Vector2 {
            x: progress_slider_position.x,
            y: (progress_slider_position.y + slider_bound_height / 2.0),
        },
        3.0,
        Color::BLACK,
    );
    draw_handle.draw_line_ex(
        Vector2 {
            x: (progress_slider_position.x - slider_bound_height),
            y: (progress_slider_position.y - slider_bound_height / 2.0),
        },
        Vector2 {
            x: (progress_slider_position.x - slider_bound_height - (slider_bound_height / 2.0)),
            y: progress_slider_position.y,
        },
        3.0,
        Color::BLACK,
    );
    draw_handle.draw_line_ex(
        Vector2 {
            x: (progress_slider_position.x - slider_bound_height),
            y: (progress_slider_position.y + slider_bound_height / 2.0),
        },
        Vector2 {
            x: (progress_slider_position.x - slider_bound_height - (slider_bound_height / 2.0)),
            y: progress_slider_position.y,
        },
        3.0,
        Color::BLACK,
    );

}
