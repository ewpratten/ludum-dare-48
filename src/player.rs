use raylib::prelude::*;

use crate::{gamecore::{GameCore, GameProgress}, items::ShopItems, lib::utils::{calculate_linear_slide, triangles::rotate_vector}, pallette::{TRANSLUCENT_WHITE_64, TRANSLUCENT_WHITE_96}, resources::GlobalResources, world::World};

const AOE_RING_MAX_RADIUS: f32 = 40.0;

#[derive(Debug, Default)]
pub struct Player {
    pub position: Vector2,
    pub direction: Vector2,
    pub size: Vector2,
    pub coins: u32,
    pub boost_percent: f32,
    pub breath_percent: f32,
    pub is_moving: bool,
    pub is_boosting: bool,
    pub is_boost_charging: bool,
    pub inventory: Vec<ShopItems>,
    pub stun_timer: f64,
    pub attacking_timer: f64,
}

impl Player {
    pub fn new(spawn: &Vector2) -> Self {
        Self {
            boost_percent: 1.0,
            size: Vector2 { x: 11.0, y: 21.0 },
            breath_percent: 1.0,
            position: spawn.clone(),
            ..Default::default()
        }
    }

    pub fn collides_with_rec(&self, rectangle: &Rectangle) -> bool {
        // // Build a bounding box of the player by their corners
        // let top_left_corner = self.position - (self.size / 2.0);
        // let bottom_right_corner = self.position + (self.size / 2.0);
        // let top_right_corner = Vector2 {
        //     x: bottom_right_corner.x,
        //     y: top_left_corner.y,
        // };
        // let bottom_left_corner = Vector2 {
        //     x: top_left_corner.x,
        //     y: bottom_right_corner.y,
        // };

        // // Get the rotation
        // let rotation = Vector2::zero().angle_to(self.direction);

        // // Rotate the bounds
        // let top_left_corner = rotate_vector(top_left_corner, rotation);
        // let bottom_right_corner = rotate_vector(bottom_right_corner, rotation);
        // let top_right_corner = rotate_vector(top_right_corner, rotation);
        // let bottom_left_corner = rotate_vector(bottom_left_corner, rotation);

        // // Check for collisions
        // return rectangle.check_collision_point_rec(top_left_corner)
        //     || rectangle.check_collision_point_rec(bottom_right_corner)
        //     || rectangle.check_collision_point_rec(top_right_corner)
        //     || rectangle.check_collision_point_rec(bottom_left_corner);

        return rectangle.check_collision_circle_rec(self.position, (self.size.y * 0.5) / 2.0);
    }

    /// Stun the player
    pub fn set_stun_seconds(&mut self, seconds: f64) {
        self.stun_timer = seconds;
    }

    /// Try to attack with the stun gun
    pub fn begin_attack(&mut self) {
        if true || self.inventory.contains(&ShopItems::StunGun) && self.stun_timer == 0.0 {
            self.stun_timer = 2.0;
        }
    }

    /// Calculate how far the player is
    pub fn calculate_depth_percent(&self, world: &World) -> f32 {
        let dist_from_player_to_end = self.position.distance_to(world.end_position);
        let dist_from_start_to_end = Vector2::zero().distance_to(world.end_position);
        return ((dist_from_start_to_end - dist_from_player_to_end) / dist_from_start_to_end)
            .clamp(0.0, 1.0);
    }

    /// Create GameProgress from the current life
    pub fn create_statistics(&self, game_core: &GameCore, current_time: f64) -> GameProgress {
        GameProgress {
            coins: self.coins,
            inventory: self.inventory.clone(),
            max_depth: self.calculate_depth_percent(&game_core.world),
            fastest_time: Some(current_time - game_core.last_state_change_time),
        }
    }

    /// Render the player
    pub fn render(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        resources: &mut GlobalResources,
        dt: f64
    ) {
        // Convert the player direction to a rotation
        let player_rotation = Vector2::zero().angle_to(self.direction);

        // Render the player's boost ring
        // This functions both as a breath meter, and as a boost meter
        let boost_ring_max_radius = self.size.x + 5.0;
        context_2d.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            boost_ring_max_radius * self.boost_percent,
            TRANSLUCENT_WHITE_64,
        );
        context_2d.draw_ring(
            Vector2 {
                x: self.position.x as i32 as f32,
                y: self.position.y as i32 as f32,
            },
            boost_ring_max_radius,
            boost_ring_max_radius + 1.0,
            0,
            (360.0 * self.breath_percent) as i32,
            0,
            TRANSLUCENT_WHITE_96,
        );

        // Calculate AOE ring
        let aoe_ring = calculate_linear_slide(self.attacking_timer) as f32;
        self.stun_timer = (self.stun_timer - dt).max(0.0);

        // Render attack AOE
        context_2d.draw_circle_lines(
            self.position.x as i32,
            self.position.y as i32,
            AOE_RING_MAX_RADIUS * aoe_ring,
            TRANSLUCENT_WHITE_64,
        );

        // Render the player based on what is happening
        if self.is_boost_charging {
            resources.player_animation_boost_charge.draw(
                context_2d,
                self.position,
                player_rotation.to_degrees() - 90.0,
            );
        } else if self.is_boosting {
            resources.player_animation_boost.draw(
                context_2d,
                self.position,
                player_rotation.to_degrees() - 90.0,
            );
        } else if self.is_moving {
            resources.player_animation_regular.draw(
                context_2d,
                self.position,
                player_rotation.to_degrees() - 90.0,
            );
        } else {
            resources.player_animation_regular.draw_frame(
                context_2d,
                self.position,
                player_rotation.to_degrees() - 90.0,
                0,
            );
        }
    }
}
