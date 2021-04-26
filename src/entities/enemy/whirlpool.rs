use raylib::prelude::*;

use super::base::EnemyBase;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Whirlpool{
	pub position: Vector2,
	pub should_remove: bool,
}

impl Whirlpool{

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
        context_2d.draw_circle(self.position.x as i32, self.position.y as i32, 12.0, Color::RED);
    }

    fn handle_logic(&mut self, player: &mut crate::player::Player, dt: f64) {

    }

    fn handle_getting_attacked(&mut self, stun_duration: f64, current_time: f64) {
		self.should_remove = true;
    }



}