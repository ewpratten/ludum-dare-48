use crate::{
    entities::enemy::base::EnemyBase,
    gamecore::{GameCore, GameProgress},
    items::{AirBag, Flashlight, Flippers, StunGun},
    lib::utils::calculate_linear_slide,
    pallette::{TRANSLUCENT_WHITE_64, TRANSLUCENT_WHITE_96},
    resources::GlobalResources,
    world::World,
};
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PlayerInventory {
    pub stun_gun: Option<StunGun>,
    pub air_bag: Option<AirBag>,
    pub flashlight: Option<Flashlight>,
    pub flippers: Option<Flippers>,
}

impl PlayerInventory {
    pub fn new() -> Self {
        Self {
            // stun_gun: Some(StunGun::lvl1()), //TMP
            // flashlight: Some(Flashlight::lvl1()), //TMP
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Player {
    pub position: Vector2,
    pub direction: Vector2,
    pub size: Vector2,
    pub radius: f32,
    pub coins: u32,
    pub boost_percent: f32,
    pub breath_percent: f32,
    pub is_moving: bool,
    pub is_boosting: bool,
    pub is_boost_charging: bool,
    pub inventory: PlayerInventory,
    pub stun_timer: f64,
    pub attacking_timer: f64,
}

impl Player {
    pub fn new(spawn: &Vector2) -> Self {
        Self {
            boost_percent: 1.0,
            size: Vector2 { x: 11.0, y: 21.0 },
            breath_percent: 1.0,
            is_moving: true,
            radius: 4.5,
            position: spawn.clone(),
            inventory: PlayerInventory::new(),
            ..Default::default()
        }
    }

    pub fn reset(&mut self, position: Vector2) {
        self.position = position;
        self.breath_percent = 1.0;
        self.boost_percent = 1.0;

        // Handle an air bag being used
        if self.inventory.air_bag.is_some() {
            self.breath_percent += self.inventory.air_bag.as_ref().unwrap().extra_oxygen;
        }
    }

    pub fn collides_with_rec(&self, rectangle: &Rectangle) -> bool {
        return rectangle.check_collision_circle_rec(self.position, self.radius);
    }

    /// Stun the player
    pub fn set_stun_seconds(&mut self, seconds: f64) {
        self.stun_timer = seconds;
    }

    /// Try to attack with the stun gun
    pub fn begin_attack(&mut self, world: &mut World, current_time: f64) {
        if self.inventory.stun_gun.is_some() && !self.is_stunned() {
            self.attacking_timer = self.inventory.stun_gun.as_ref().unwrap().duration;

            // Stun everything in reach
            let stun_reach = self.inventory.stun_gun.as_ref().unwrap().range;

            for jellyfish in world.jellyfish.iter_mut() {
                if jellyfish.position.distance_to(self.position).abs() <= stun_reach {
                    jellyfish.handle_getting_attacked(self.attacking_timer, current_time);
                }
            }
            for octopus in world.octopus.iter_mut() {
                if octopus.current_position.distance_to(self.position).abs() <= stun_reach {
                    octopus.handle_getting_attacked(self.attacking_timer, current_time);
                }
            }
			for whirlpool in world.whirlpool.iter_mut() {
                if whirlpool.position.distance_to(self.position).abs() <= stun_reach {
                    whirlpool.handle_getting_attacked(self.attacking_timer, current_time);
                }
            }
        }
    }

    pub fn is_stun_gun_active(&self) -> bool {
        return self.attacking_timer != 0.0 && self.inventory.stun_gun.is_some();
    }

    pub fn is_stunned(&self) -> bool {
        return self.stun_timer > 0.0;
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
        dt: f64,
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
        if self.is_stun_gun_active() {
            let animation_progression =
                self.attacking_timer / self.inventory.stun_gun.as_ref().unwrap().duration;
            let aoe_ring = calculate_linear_slide(animation_progression) as f32;
            self.attacking_timer = (self.attacking_timer - dt).max(0.0);

            // Render attack AOE
            if animation_progression >= 0.5 {
                context_2d.draw_circle_lines(
                    self.position.x as i32,
                    self.position.y as i32,
                    self.inventory.stun_gun.as_ref().unwrap().range * aoe_ring,
                    TRANSLUCENT_WHITE_64.fade(aoe_ring),
                );
            } else {
                context_2d.draw_circle_lines(
                    self.position.x as i32,
                    self.position.y as i32,
                    self.inventory.stun_gun.as_ref().unwrap().range,
                    TRANSLUCENT_WHITE_64.fade(aoe_ring),
                );
            }
        }

        // Render the player based on what is happening
        if self.is_stunned() {
            resources.player_animation_stunned.draw(
                context_2d,
                self.position,
                player_rotation.to_degrees() - 90.0,
            );
        } else if self.is_boost_charging {
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
