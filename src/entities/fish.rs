use raylib::prelude::*;

use crate::{gamecore::GameCore, lib::utils::triangles::rotate_vector, player::Player};

const FISH_FOLLOW_PLAYER_DISTANCE: f32 = 80.0;
const FISH_FOLLOW_PLAYER_SPEED: f32 = 2.0;
const FISH_FOLLOW_PLAYER_SPEED_FAST: f32 = 6.0;

#[derive(Debug, Clone)]
pub struct FishEntity {
    position: Vector2,
    direction: Vector2,
    following_player: bool,
    size: Vector2
}

impl FishEntity {
    pub fn new(position: Vector2) -> Self {
        Self {
            position: position,
            direction: Vector2::zero(),
            following_player: true,
            size: Vector2 {
                x: 5.0,
                y: 8.0
            }
        }
    }

    pub fn new_from_positions(positions: &Vec<Vector2>) -> Vec<Self> {
        let mut output = Vec::new();
        for position in positions {
            output.push(FishEntity::new(*position));
        }
        return output;
    }

    pub fn handle_follow_player(&mut self, player: &Player) {
        // Distance and direction to player
        let dist_to_player = player.position - self.position;
        let dist_to_player_lin = self.position.distance_to(player.position);
        let mut direction_to_player = dist_to_player;
        direction_to_player.normalize();

        // Fish movement
        let movement;

        // If the fish is double its follow distance from the player
        if dist_to_player_lin.abs() > (FISH_FOLLOW_PLAYER_DISTANCE * 2.0) {
            movement = direction_to_player * FISH_FOLLOW_PLAYER_SPEED_FAST;
        } else {
            // Move slowly in the direction of the player unless too close
            if dist_to_player_lin.abs() > FISH_FOLLOW_PLAYER_DISTANCE {
                movement = direction_to_player * FISH_FOLLOW_PLAYER_SPEED;
            } else {
                movement = Vector2::zero();
            }
        }

        // Move the fish
        self.direction = direction_to_player;
        self.position += movement;
    }

    pub fn handle_free_movement(&mut self) {}

    pub fn update_position(&mut self, player: &Player, dt: f64) {
        if self.following_player {
            self.handle_follow_player(player);
        } else {
            self.handle_free_movement();
        }
    }

    pub fn render(&self, context_2d: &mut RaylibMode2D<RaylibDrawHandle>) {

        // Direction
        let direction = Vector2::zero().angle_to(self.direction.normalized()) + (90.0 as f32).to_radians();

        // Get the corners of the fish
        let fish_front = rotate_vector(Vector2 {
            x: 0.0,
            y: (self.size.y / 2.0) * -1.0,
        }, direction);
        let fish_bl = rotate_vector(Vector2 {
            x: (self.size.x / 2.0) * -1.0,
            y: (self.size.y / 2.0),
        }, direction);
        let fish_br = rotate_vector(Vector2 {
            x: (self.size.x / 2.0),
            y: (self.size.y / 2.0),
        }, direction);

        // Draw the fish as a triangle with rotation
        context_2d.draw_triangle(self.position + fish_front, self.position + fish_bl, self.position + fish_br, Color::BLACK);

    }
}
