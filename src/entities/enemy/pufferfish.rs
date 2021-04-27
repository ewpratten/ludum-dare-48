use raylib::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{lib::utils::calculate_linear_slide, pallette::TRANSLUCENT_RED_64};

use super::base::EnemyBase;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PufferState {
    SmallIdle,
    Growing,
    LargeIdle,
    Blowing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pufferfish {
    pub position: Vector2,
    pub is_knocking_back: bool,
    pub time_knocking_back: f64,

    pub inflate_timer: f64,
    pub is_large: bool,
    pub stun_timer: f64,
    pub puffer_state: PufferState,
}

impl EnemyBase for Pufferfish {
    fn render(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        player: &mut crate::player::Player,
        resources: &mut crate::resources::GlobalResources,
        dt: f64,
    ) {

		let is_stunned = self.stun_timer > 0.0;

		// Render the stun ring
        if is_stunned {
			// println!("Stunned");
            let stun_ring_alpha =
                calculate_linear_slide(self.stun_timer / 1.0);
            context_2d.draw_circle_v(
                self.position,
                12.0,
                TRANSLUCENT_RED_64.fade(0.55 * stun_ring_alpha as f32),
            );

            self.stun_timer -= dt;
        }


		let angle = player.position.angle_to(self.position).to_degrees();

		
        match self.puffer_state {
            PufferState::SmallIdle => {
                resources.pufferfish_small.draw(
                    context_2d,
                    Vector2 {
                        x: self.position.x,
                        y: self.position.y,
                    },
                    angle,
                );

				if self.position.distance_to(player.position).abs() <= 100.0 && self.inflate_timer > 2.0{
					self.puffer_state = PufferState::Growing;
				}
				self.is_large = false;
            },
            PufferState::Growing => {
				self.inflate_timer = 0.0;
                resources.pufferfish_expand.draw(
                    context_2d,
                    Vector2 {
                        x: self.position.x,
                        y: self.position.y,
                    },
                    angle,
                );

                if resources.pufferfish_expand.get_current_frame_id(context_2d) == 3 {
                   self.puffer_state = PufferState::LargeIdle;
                }
				self.is_large = true;
                
            },
            PufferState::LargeIdle => {
				self.inflate_timer = 0.0;
                resources.pufferfish_big.draw(
                    context_2d,
                    Vector2 {
                        x: self.position.x,
                        y: self.position.y,
                    },
                    angle,
                );

				if self.position.distance_to(player.position).abs() <= 65.0{
					self.puffer_state = PufferState::Blowing;
					self.is_knocking_back = true;
					self.time_knocking_back = 0.0;
				}

				self.is_large = true;
            },
            PufferState::Blowing => {
				
				resources.pufferfish_attack.draw(
                    context_2d,
                    Vector2 {
                        x: self.position.x,
                        y: self.position.y,
                    },
                    angle,
                );


				if resources.pufferfish_expand.get_current_frame_id(context_2d) == 3 && self.inflate_timer > 1.0{
                    self.puffer_state = PufferState::SmallIdle;
					self.inflate_timer = 0.0;
                }
				self.is_large = false;
			},
        }
    }

    fn handle_logic(&mut self, player: &mut crate::player::Player, dt: f64) -> u8 {



		self.inflate_timer += dt;
        self.time_knocking_back += dt;

		if self.time_knocking_back >= 0.5{
			self.is_knocking_back = false;
		}

		if self.position.distance_to(player.position).abs() > 120.0 && self.is_large {
			self.puffer_state = PufferState::Blowing;
			self.inflate_timer = 0.0;
		}

        return 0;

    }

    fn handle_getting_attacked(&mut self, stun_duration: f64, current_time: f64) {

		self.stun_timer = stun_duration;

	}
}
