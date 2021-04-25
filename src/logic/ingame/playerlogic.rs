use raylib::prelude::*;

use crate::{
    gamecore::GameCore,
    pallette::{TRANSLUCENT_WHITE_128, TRANSLUCENT_WHITE_64, TRANSLUCENT_WHITE_96},
};

const NORMAL_PLAYER_SPEED: i32 = 1;
const BOOST_PLAYER_SPEED: i32 = NORMAL_PLAYER_SPEED * 2;
const CAMERA_FOLLOW_SPEED: f32 = 0.7;
const TURN_SPEED: f32 = 0.15;
const BOOST_DECREASE_PER_SECOND: f32 = 0.65;
const BOOST_REGEN_PER_SECOND: f32 = 0.25;
const BREATH_DECREASE_PER_SECOND: f32 = 0.02;

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

    let tau: f32 = PI as f32 * 2.0;
    // get angles as floats
    let mut player_angle: f32 = Vector2::zero().angle_to(game_core.player.direction);
    let mut desired_angle: f32 = Vector2::zero().angle_to(normalized_movement_direction);

    // make angle positive
    if desired_angle < 0.0 {
        desired_angle += tau;
    }

    // turn towards mouse at turn speed
    if player_angle % tau > desired_angle {
        if (player_angle % tau) - desired_angle > PI as f32 {
            player_angle += TURN_SPEED;
        } else {
            player_angle -= TURN_SPEED;
        }
    } else {
        if desired_angle - (player_angle % tau) > PI as f32 {
            player_angle -= TURN_SPEED;
        } else {
            player_angle += TURN_SPEED;
        }
    }

    // snap to mouse if close enough
    if f32::abs(player_angle - desired_angle) < (TURN_SPEED * 1.1) {
        player_angle = desired_angle;
    }
    if player_angle > tau {
        player_angle -= tau;
    }
    if player_angle < 0.0 {
        player_angle += tau;
    }

    // set angle
    if !game_core.player.is_stunned() {
        game_core.player.direction = Vector2::new(f32::cos(player_angle), f32::sin(player_angle));
    }

    // In the case the player is in "null", just jump the camera to them
    if game_core.player.position == Vector2::zero() {
        game_core.master_camera.target = game_core.player.position - (window_center / 2.0);
    }

    // Handle action buttons
    let user_request_boost = draw_handle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);
    let user_request_action = draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON);

    if user_request_action {
        game_core.player.begin_attack(&mut game_core.world, draw_handle.get_time());
    }

    // Move the player in their direction
    let mut speed_multiplier;
    if user_request_boost && game_core.player.boost_percent >= 0.0 {
        // Set the speed multiplier
        speed_multiplier = BOOST_PLAYER_SPEED as f32;

        // Decrease the boost
        game_core.player.boost_percent -= BOOST_DECREASE_PER_SECOND * dt as f32;
        game_core.player.is_boosting = true;
        if game_core.player.boost_percent >= 0.9 {
            game_core
                .resources
                .player_animation_boost_charge
                .start(draw_handle);
            game_core.resources.player_animation_regular.stop();
            game_core.player.is_boost_charging = true;
        } else {
            game_core.resources.player_animation_boost_charge.stop();
            game_core
                .resources
                .player_animation_boost
                .start(draw_handle);
            game_core.player.is_boost_charging = false;
        }
    } else {
        // Set the speed multiplier
        speed_multiplier = NORMAL_PLAYER_SPEED as f32;

        // Reset boost animation
        game_core.player.is_boosting = false;
        game_core.player.is_boost_charging = false;
        game_core.resources.player_animation_boost_charge.stop();
        game_core.resources.player_animation_boost.stop();
        game_core
            .resources
            .player_animation_regular
            .start(draw_handle);

        // Handle boost regen
        if !user_request_boost {
            game_core.player.boost_percent = (game_core.player.boost_percent
                + BOOST_REGEN_PER_SECOND * dt as f32)
                .clamp(0.0, 1.0);
        }
    }

    // Handle flippers doing a speed increase
    if game_core.player.inventory.flippers.is_some() {
        speed_multiplier = speed_multiplier
            * game_core
                .player
                .inventory
                .flippers
                .as_ref()
                .unwrap()
                .speed_increase;
    }

    // Update the player's breath
    game_core.player.breath_percent =
        (game_core.player.breath_percent - BREATH_DECREASE_PER_SECOND * dt as f32).clamp(0.0, 1.0);

    // Only do this if the mouse is far enough away
    let player_stunned = game_core.player.stun_timer > 0.0;
    let mut player_real_movement = game_core.player.direction * speed_multiplier;

    // Handle the player wearing flippers
    if game_core.player.inventory.flippers.is_some() {
        player_real_movement *= game_core.player.inventory.flippers.as_ref().unwrap().speed_increase;
    }

    // Handle movement and collisions
    if raw_movement_direction.distance_to(Vector2::zero()) > game_core.player.size.y / 2.0
        && !game_core.player.is_stunned()
    {
        if game_core.player.is_moving {
            // move in x
            game_core.player.position.x += player_real_movement.x;

            // Check for any collisions
            for collider in game_core.world.colliders.iter() {
                if game_core.player.collides_with_rec(collider) {
                    game_core.player.position.x -= player_real_movement.x;
                    player_real_movement.x = 0.0;
                    break;
                }
            }

            // move in y
            game_core.player.position.y += player_real_movement.y;

            // Check for any collisions
            for collider in game_core.world.colliders.iter() {
                if game_core.player.collides_with_rec(collider) {
                    game_core.player.position.y -= player_real_movement.y;
                    player_real_movement.y = 0.0;
                    break;
                }
            }
        }
    }

    // Handle updating the stun timer
    if player_stunned {
        game_core.player.stun_timer -= dt;
    }

    // Move the camera to follow the player
    let direction_from_cam_to_player =
        (game_core.player.position - window_center) - game_core.master_camera.target;
    let player_screen_position =
        draw_handle.get_world_to_screen2D(game_core.player.position, game_core.master_camera);

    // Camera only moves if you get close to the edge of the screen
    if player_screen_position.distance_to(window_center).abs() > 100.0 {
        game_core.master_camera.target += player_real_movement;
    }

    // If the player is not on screen, snap the camera to them
    if player_screen_position.distance_to(window_center).abs() > window_center.y {
        game_core.master_camera.target = game_core.player.position - (window_center / 2.0);
    }

    // // Clamp camera target y to 0
    // if game_core.master_camera.target.y < -100.0 {
    //     game_core.master_camera.target.y = -100.0;
    // }
}
