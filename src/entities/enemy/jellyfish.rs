use super::base::EnemyBase;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{player::Player, resources::GlobalResources};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct JellyFish {
    pub position: Vector2,

    #[serde(skip)]
    pub stunned_timer: f64
}

impl JellyFish {
    

}

impl EnemyBase for JellyFish {
    fn render(&self, context_2d: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>, resources: &mut GlobalResources) {

        // Render the jellyfish
        resources.jellyfish_animation_regular.draw(context_2d, self.position, 0.0);
        
        // // TODO
        // context_2d.draw_circle_v(self.position, 5.0, Color::RED);
    }

    fn handle_logic(&mut self, player: &mut Player, dt: f64) {
        todo!()
    }

    fn handle_getting_attacked(&mut self) {
        todo!()
    }
}