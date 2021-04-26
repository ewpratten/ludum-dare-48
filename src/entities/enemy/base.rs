use raylib::prelude::*;

use crate::{player::Player, resources::GlobalResources};

pub trait EnemyBase {
    fn render(
        &mut self,
        context_2d: &mut RaylibMode2D<RaylibDrawHandle>,
        player: &mut Player,
        resources: &mut GlobalResources,
        dt: f64,
    );
    fn handle_logic(&mut self, player: &mut Player, dt: f64) -> u8;
    fn handle_getting_attacked(&mut self, stun_duration: f64, current_time: f64);
}
