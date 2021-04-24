use super::base::EnemyBase;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Octopus {
    pub position_a: Vector2,
    pub position_b: Vector2,

    #[serde(skip)]
    pub current_position: Vector2,

    #[serde(skip)]
    pub stunned_timer: f64,
    #[serde(skip)]
    pub max_stunned_time: f64,
}

impl Octopus {}

impl EnemyBase for Octopus {
    fn render(
        &mut self,
        context_2d: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
        resources: &mut crate::resources::GlobalResources,
        dt: f64,
    ) {
        let is_octopus_stunned = self.stunned_timer > 0.0;

        // Simple sine position
        let h_trans = if is_octopus_stunned {
            0.0
        } else {
            (context_2d.get_time() / 8.0).sin().abs() as f32
        };

        // Modify the current pose
        let dist_a_to_b = self.position_b - self.position_a;
        self.current_position =(dist_a_to_b * h_trans) + self.position_a;

        // TODO: TMP
        context_2d.draw_circle_v(self.current_position, 10.0, Color::RED);
    }

    fn handle_logic(&mut self, player: &mut crate::player::Player, dt: f64) {}

    fn handle_getting_attacked(&mut self, stun_duration: f64) {
        todo!()
    }
}
