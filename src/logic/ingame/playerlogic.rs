use raylib::prelude::*;

use crate::{gamecore::GameCore, pallette::{TRANSLUCENT_WHITE_128, TRANSLUCENT_WHITE_64, TRANSLUCENT_WHITE_96}};

const NORMAL_PLAYER_SPEED: i32 = 4;
const BOOST_PLAYER_SPEED: i32 = NORMAL_PLAYER_SPEED * 2;
const CAMERA_FOLLOW_SPEED: f32 = 0.7;
const BOOST_DECREASE_PER_SECOND: f32 = 0.75;
const BOOST_REGEN_PER_SECOND: f32 = 0.25;
const BREATH_DECREASE_PER_SECOND: f32 = 0.01;

pub fn update_player_movement(
    draw_handle: &mut RaylibDrawHandle,
    game_core: &mut GameCore,
    window_center: Vector2,
) {
    // Calculate DT
    let dt = draw_handle.get_time() - game_core.last_frame_time;

    // Handle player movement
    let mouse_pose = draw_handle.get_mouse_position();
    let mouse_world_pose = draw_handle.get_screen_to_world2D(mouse_pose, game_core.master_camera);
    let raw_movement_direction = mouse_world_pose - game_core.player.position;
    let mut normalized_movement_direction = raw_movement_direction;
    normalized_movement_direction.normalize();
    game_core.player.direction = normalized_movement_direction;

    // In the case the player is in "null", just jump the camera to them
    if game_core.player.position == Vector2::zero() {
        game_core.master_camera.target = game_core.player.position - (window_center / 2.0);
    }

    // Handle action buttons
    let user_request_boost = draw_handle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);
    let user_request_action = draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON);

    // Move the player in their direction
    let speed_multiplier;
    if user_request_boost && game_core.player.boost_percent >= 0.0 {
        // Set the speed multiplier
        speed_multiplier = BOOST_PLAYER_SPEED as f32;

        // Decrease the boost
        game_core.player.boost_percent -= BOOST_DECREASE_PER_SECOND * dt as f32;
    } else {
        // Set the speed multiplier
        speed_multiplier = NORMAL_PLAYER_SPEED as f32;

        // Handle boost regen
        game_core.player.boost_percent =
            (game_core.player.boost_percent + BOOST_REGEN_PER_SECOND * dt as f32).clamp(0.0, 1.0);
    }

    // Update the player's breath
    game_core.player.breath_percent = (game_core.player.breath_percent - BREATH_DECREASE_PER_SECOND * dt as f32).clamp(0.0, 1.0);

    // Only do this if the mouse is far enough away
    let player_real_movement = game_core.player.direction * speed_multiplier;
    if raw_movement_direction.distance_to(Vector2::zero()) > game_core.player.size.y / 2.0 {
        game_core.player.position += player_real_movement;
    }

    // Move the camera to follow the player
    let direction_from_cam_to_player =
        (game_core.player.position - window_center) - game_core.master_camera.target;
    let player_screen_position =
        draw_handle.get_world_to_screen2D(game_core.player.position, game_core.master_camera);

    // Camera only moves if you get close to the edge of the screen
    if player_screen_position.distance_to(window_center).abs() > (window_center.y - 40.0) {
        game_core.master_camera.target += player_real_movement;
    }
}

pub fn render_player(context_2d: &mut RaylibMode2D<RaylibDrawHandle>, game_core: &mut GameCore) {
    // Get the player
    let player = &game_core.player;

    // Convert the player direction to a rotation
    let player_rotation = Vector2::zero().angle_to(player.direction);

    // Render the player's boost ring
    // This functions both as a breath meter, and as a boost meter
    let boost_ring_max_radius = player.size.x + 5.0;
    context_2d.draw_circle(
        player.position.x as i32,
        player.position.y as i32,
        boost_ring_max_radius * player.boost_percent,
        TRANSLUCENT_WHITE_64,
    );
    context_2d.draw_ring(
        player.position,
        boost_ring_max_radius - 2.0,
        boost_ring_max_radius + 2.0,
        0,
        (360.0 * player.breath_percent) as i32,
        0,
        TRANSLUCENT_WHITE_96,
    );

    // TODO: tmp rect
    context_2d.draw_rectangle_pro(
        Rectangle {
            x: player.position.x,
            y: player.position.y,
            width: player.size.x,
            height: player.size.y,
        },
        player.size / 2.0,
        player_rotation.to_degrees() + 90.0,
        Color::BLACK,
    );
}
