use super::base::EnemyBase;
use crate::{player::Player, resources::GlobalResources};
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct JellyFish {
    pub position: Vector2,

    #[serde(skip)]
    pub stunned_timer: f64,

    #[serde(skip)]
    pub do_stun_player: bool
}

impl JellyFish {}

impl EnemyBase for JellyFish {
    fn render(
        &mut self,
        context_2d: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
        resources: &mut GlobalResources,
    ) {
        // Simple sine position
        let v_trans = context_2d.get_time().sin();

        // Render the jellyfish
        resources.jellyfish_animation_regular.draw(
            context_2d,
            Vector2 {
                x: self.position.x,
                y: self.position.y + (2.0 * v_trans as f32),
            },
            0.0,
        );
        resources.jellyfish_animation_attack.draw(
            context_2d,
            Vector2 {
                x: self.position.x
                    ,
                y: self.position.y + (2.0 * v_trans as f32)
                    ,
            },
            0.0,
        );
        self.do_stun_player = resources.jellyfish_animation_attack.get_current_frame_id(context_2d) == 13;
    }

    fn handle_logic(&mut self, player: &mut Player, dt: f64) {
        
        // Handle stunning the player
        if self.do_stun_player {
            
        }
    }

    fn handle_getting_attacked(&mut self) {
        todo!()
    }
}
