use raylib::prelude::*;

use crate::gamecore::GameCore;

const NORMAL_PLAYER_SPEED: i32 = 4;
const BOOST_PLAYER_SPEED: i32 = NORMAL_PLAYER_SPEED * 2;
const CAMERA_FOLLOW_SPEED: f32 = 0.7;

pub fn update_player_movement(
    draw_handle: &mut RaylibDrawHandle,
    game_core: &mut GameCore,
    window_center: Vector2,
) {
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
    let speed_multiplier = match user_request_boost && game_core.player.boost_percent >= 0.0 {
        true => BOOST_PLAYER_SPEED as f32,
        false => NORMAL_PLAYER_SPEED as f32,
    };

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
