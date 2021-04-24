use super::base::EnemyBase;
use crate::{
    lib::utils::calculate_linear_slide, pallette::TRANSLUCENT_RED_64, player::Player,
    resources::GlobalResources,
};
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct JellyFish {
    pub position: Vector2,

    #[serde(skip)]
    pub stunned_timer: f64,
    #[serde(skip)]
    pub max_stunned_time: f64,

    #[serde(skip)]
    pub do_stun_player: bool,
}

impl JellyFish {}

impl EnemyBase for JellyFish {
    fn render(
        &mut self,
        context_2d: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
        resources: &mut GlobalResources,
        dt: f64
    ) {
        let is_jelly_stunned = self.stunned_timer != 0.0;

        // Simple sine position
        let v_trans = if is_jelly_stunned {
            0.0
        } else {
            context_2d.get_time().sin()
        };
        let trans_pose = Vector2 {
            x: self.position.x,
            y: self.position.y + (2.0 * v_trans as f32),
        };

        // Render the stun ring
        if self.max_stunned_time > 0.0 && self.stunned_timer > 0.0 {
            let stun_ring_radius =
                calculate_linear_slide(self.stunned_timer / self.max_stunned_time);
            context_2d.draw_circle_v(
                trans_pose,
                stun_ring_radius as f32 * 20.0,
                TRANSLUCENT_RED_64,
            );
            self.stunned_timer -= dt;
        }

        // Render the jellyfish
        resources
            .jellyfish_animation_regular
            .draw(context_2d, trans_pose, 0.0);

        // Only do stun loop if not stunned
        if !is_jelly_stunned {
            resources
                .jellyfish_animation_attack
                .draw(context_2d, trans_pose, 0.0);
        }

        // Check if the jelly is in stun mode
        self.do_stun_player = (resources
            .jellyfish_animation_attack
            .get_current_frame_id(context_2d)
            == 13)
            && !is_jelly_stunned;
    }

    fn handle_logic(&mut self, player: &mut Player, dt: f64) {
        // Handle stunning the player
        if self.do_stun_player {}
    }

    fn handle_getting_attacked(&mut self, stun_duration: f64) {
        println!("Attack");
        self.stunned_timer = stun_duration;
        self.max_stunned_time = stun_duration;
    }
}
