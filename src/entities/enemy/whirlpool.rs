use raylib::prelude::*;

use super::base::EnemyBase;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Whirlpool{
	pub position: Vector2,

	// Track if it needs removing
	pub should_remove: bool,
	
	// variable for tracking rotation
	pub rotation: f32,
}

impl Whirlpool{

	// hook to see if item needs removing
	pub fn should_remove(&self) -> bool{
		return self.should_remove;
	}

}

impl EnemyBase for Whirlpool{
    fn render(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        player: &mut crate::player::Player,
        resources: &mut crate::resources::GlobalResources,
        dt: f64,
    ) {

		resources.whirlpool.draw(context_2d, Vector2{x: self.position.x, y: self.position.y}, self.rotation);
        self.rotation += 1.0;
    }

    fn handle_logic(&mut self, player: &mut crate::player::Player, dt: f64) {

    }

	// Whirlpool removed if shoot
    fn handle_getting_attacked(&mut self, stun_duration: f64, current_time: f64) {
		self.should_remove = true;
    }



}