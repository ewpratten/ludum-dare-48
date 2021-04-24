use rand::{Rng, prelude::ThreadRng};
use raylib::prelude::*;

use crate::{gamecore::GameCore, lib::utils::triangles::rotate_vector, player::Player, world::World};

const FISH_FOLLOW_PLAYER_DISTANCE: f32 = 30.0;
const FISH_FOLLOW_PLAYER_SPEED: f32 = 1.8;
const FISH_FOLLOW_PLAYER_SPEED_FAST: f32 = FISH_FOLLOW_PLAYER_SPEED * 3.0;
const FISH_ATTACH_RADIUS: f32 = 20.0;

const FISH_VISION: f32 = 25.0;
const FISH_MAX_SPEED: f32 = 2.0;
const FISH_MAX_FORCE: f32 = 0.05;
const FISH_FACTOR_ATTRACTION: f32 = 1.0;
const FISH_FACTOR_PLAYER: f32 = 0.1;
const FISH_FACTOR_COHESION: f32 = 0.1;
const FISH_SEPARATION_DISTANCE: f32 = 15.0;
const FISH_FACTOR_SEPARATION: f32 = 1.5;

#[derive(Debug, Clone)]
pub struct FishEntity {
    position: Vector2,
    direction: Vector2,
    velocity: Vector2,
    pub following_player: bool,
    size: Vector2,
    rng: ThreadRng
}

impl FishEntity {
    pub fn new(position: Vector2) -> Self {
        Self {
            position: position,
            direction: Vector2::zero(),
            velocity: Vector2::zero(),
            following_player: false,
            size: Vector2 { x: 5.0, y: 8.0 },
            rng: rand::thread_rng()
        }
    }

    pub fn new_from_positions(positions: &Vec<Vector2>) -> Vec<Self> {
        let mut output = Vec::new();
        for position in positions {
            output.push(FishEntity::new(*position));
        }
        return output;
    }

    pub fn handle_follow_player(&mut self, player: &Player, dt: f64, other_fish: &Vec<FishEntity>) {
        let mut acceleration: Vector2 = Vector2::zero();
 
        let mut steer: Vector2 = Vector2::zero();
        let mut count1: u16 = 0;
        let mut sum1: Vector2 = Vector2::zero();
        let mut count2: u16 = 0;
        let mut sum2: Vector2 = Vector2::zero();
        let mut count3: u16 = 0;
        // separation
        for i in other_fish {
            let dist = (self.position - i.position).length();
            if dist < FISH_SEPARATION_DISTANCE && dist > 0.0 {
                let mut diff: Vector2 = self.position - i.position;
                diff.normalize();
                diff /= dist;
                steer += diff;
                count1 += 1;
            }
            if dist < FISH_VISION && dist > 0.0 {
                sum1 += i.direction;
                count2 += 1;
                sum2 += i.position;
                count3 += 1;
            }
        }
        if count1 > 0 {
            steer /= count1 as f32;
        }
        if steer.x != 0.0 || steer.y != 0.0 {
            steer.normalize();
            steer *= FISH_MAX_SPEED;
            steer -= self.velocity;
            steer.x = f32::min(f32::max(steer.x, -FISH_MAX_FORCE), FISH_MAX_FORCE);
            steer.y = f32::min(f32::max(steer.y, -FISH_MAX_FORCE), FISH_MAX_FORCE);
            acceleration += steer * FISH_FACTOR_SEPARATION;
        }

        // attraction
        if count2 > 0 {
            sum1 /= count2 as f32;
            sum1.normalize();
            sum1 *= FISH_MAX_SPEED;
            sum1 -= self.velocity;
            sum1.x = f32::min(f32::max(sum1.x, -FISH_MAX_FORCE), FISH_MAX_FORCE);
            sum1.y = f32::min(f32::max(sum1.y, -FISH_MAX_FORCE), FISH_MAX_FORCE);
            acceleration += sum1 * FISH_FACTOR_ATTRACTION;
        }

        // cohesion
        if count3 > 0 {
            sum2 /= count3 as f32;
            let mut desired: Vector2 = sum2 - self.position;

            desired.normalize();
            desired *= FISH_MAX_SPEED;

            desired.x = f32::min(f32::max(desired.x, -FISH_MAX_FORCE), FISH_MAX_FORCE);
            desired.y = f32::min(f32::max(desired.y, -FISH_MAX_FORCE), FISH_MAX_FORCE);

            acceleration += desired * FISH_FACTOR_COHESION;
        }

        // turn to player
        let mut player_factor: Vector2 = player.position - self.position;
        player_factor.normalize();
        acceleration += player_factor * FISH_FACTOR_PLAYER;

        // Move the fish
        self.direction = self.velocity.normalized();
        self.velocity += acceleration;
        
        self.velocity.x = f32::min(f32::max(self.velocity.x, -FISH_MAX_SPEED), FISH_MAX_SPEED);
        self.velocity.y = f32::min(f32::max(self.velocity.y, -FISH_MAX_SPEED), FISH_MAX_SPEED);
        self.position += self.velocity;
    }

    pub fn handle_free_movement(&mut self, player: &mut Player, dt: f64) {
        // Distance and direction to player
        let dist_to_player = player.position - self.position;
        let dist_to_player_lin = self.position.distance_to(player.position);
        let mut direction_to_player = dist_to_player;
        direction_to_player.normalize();

        // Handle player picking up fish
        if player.position.distance_to(self.position).abs() <= player.size.y * 2.2 {
            self.following_player = true;
            self.velocity = self.direction.normalized();

            // Add currency to the player
            player.coins += 1;
        }

        // Look at the player;
        self.position = self.position;
        self.direction = direction_to_player;
    }

    pub fn update_position(&mut self, player: &mut Player, dt: f64, other_fish: &Vec<FishEntity>) {
        if self.following_player {
            self.handle_follow_player(player, dt, other_fish);
        } else {
            self.handle_free_movement(player, dt);
        }
    }

    pub fn render(&self, context_2d: &mut RaylibMode2D<RaylibDrawHandle>) {
        // Direction
        let direction =
            Vector2::zero().angle_to(self.direction.normalized()) + (90.0 as f32).to_radians();

        // Get the corners of the fish
        let fish_front = rotate_vector(
            Vector2 {
                x: 0.0,
                y: (self.size.y / 2.0) * -1.0,
            },
            direction,
        );
        let fish_bl = rotate_vector(
            Vector2 {
                x: (self.size.x / 2.0) * -1.0,
                y: (self.size.y / 2.0),
            },
            direction,
        );
        let fish_br = rotate_vector(
            Vector2 {
                x: (self.size.x / 2.0),
                y: (self.size.y / 2.0),
            },
            direction,
        );

        // Draw the fish as a triangle with rotation
        context_2d.draw_triangle(
            self.position + fish_front,
            self.position + fish_bl,
            self.position + fish_br,
            Color::BLACK,
        );
    }
}
