use raylib::prelude::*;

use serde::{Deserialize, Serialize};

use super::base::EnemyBase;


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Pufferfish{
	pub position: Vector2,
	pub is_knocking_back: bool,
	pub time_knocking_back: f64,

	pub inflate_timer: f64,
	pub size: f32,
	pub is_stunned: bool,
}


impl EnemyBase for Pufferfish{
    fn render(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        player: &mut crate::player::Player,
        resources: &mut crate::resources::GlobalResources,
        dt: f64,
    ) {
		

		

        context_2d.draw_circle(self.position.x as i32, self.position.y as i32, self.size, Color::RED);
    }

    fn handle_logic(&mut self, player: &mut crate::player::Player, dt: f64) {
        if self.position.distance_to(player.position).abs() <= self.size + 6.0{
			self.is_knocking_back = true;
			
		} 

		if self.is_knocking_back{
			self.time_knocking_back += dt;
		}

		if self.time_knocking_back >= 0.5{
			self.is_knocking_back = false;
			self.time_knocking_back = 0.0;
		}

		if self.position.distance_to(player.position).abs() <= 100.0{
			self.inflate_timer += dt;
		}else{
			self.inflate_timer = 0.0;
		}

		self.size = (6.0 * (1.0 + self.inflate_timer / 1.0)).clamp(6.0, 12.0) as f32;




    }

    fn handle_getting_attacked(&mut self, stun_duration: f64, current_time: f64) {
        
    }
}