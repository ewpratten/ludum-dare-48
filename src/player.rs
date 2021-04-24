use raylib::math::{Rectangle, Vector2};

use crate::lib::utils::triangles::rotate_vector;

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

        return rectangle.check_collision_circle_rec(self.position, self.radius);
    }
}
