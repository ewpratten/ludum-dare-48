use rand::{prelude::ThreadRng, Rng};
use raylib::prelude::*;

use crate::{
    gamecore::{self, GameCore},
    lib::utils::triangles::rotate_vector,
    player::Player,
    resources::GlobalResources,
    world::World,
};

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
    current_frame: u8,
    animation_counter: u32,
    size: Vector2,
    color: u8,
    rng: ThreadRng,
}

impl FishEntity {
    pub fn new(position: Vector2) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position: position,
            direction: Vector2 {
                x: rng.gen_range(0.0..1.0),
                y: rng.gen_range(0.0..1.0),
            },
            velocity: Vector2::zero(),
            following_player: false,
            animation_counter: 0,
            current_frame: 0,
            size: Vector2 { x: 5.0, y: 8.0 },
            color: rng.gen_range(0..6),
            rng,
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

        // Handle player picking up fish
        if player.position.distance_to(self.position).abs() <= player.size.y * 2.2 {
            self.following_player = true;
            self.velocity = self.direction.normalized();
            self.current_frame = 0;
            self.animation_counter = 0;

            // Add currency to the player
            player.coins += 1;
        }
    }

    pub fn update_position(&mut self, player: &mut Player, dt: f64, other_fish: &Vec<FishEntity>) {
        if self.following_player {
            self.handle_follow_player(player, dt, other_fish);
        } else {
            self.handle_free_movement(player, dt);
        }
    }

    pub fn render(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        resources: &mut GlobalResources,
    ) {
        // Direction
        let direction = (Vector2::zero().angle_to(self.direction.normalized())).to_degrees();

        self.animation_counter += 1;

        // swimming
        if self.following_player {
            if self.animation_counter % 3 == 0 {
                self.current_frame += 1;
                if self.current_frame == 8 {
                    self.current_frame = 0;
                }
            }
            resources.fish_animation_swim.draw_frame(
                context_2d,
                self.position,
                direction,
                (self.current_frame + self.color * 9) as u32,
            );
        // idle
        } else {
            if self.animation_counter % 10 == 0 {
                if self.current_frame == 0 {
                    self.current_frame = 1;
                } else {
                    self.current_frame = 0;
                }
            }
            resources.fish_animation_idle.draw_frame(
                context_2d,
                self.position,
                direction,
                (self.current_frame + self.color * 2) as u32,
            );
        }
    }
}
