use raylib::prelude::*;

use serde::{Deserialize, Serialize};

use super::base::EnemyBase;


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Pufferfish{
	pub position: Vector2,

}


impl EnemyBase for Pufferfish{
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
        
    }
}