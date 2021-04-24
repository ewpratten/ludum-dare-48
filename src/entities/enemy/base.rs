use raylib::prelude::*;

use crate::player::Player;

pub trait EnemyBase {
    fn render(&self, context_2d: &mut RaylibMode2D<RaylibDrawHandle>);
    fn handle_logic(&mut self, player: &mut Player, dt: f64);
    fn handle_getting_attacked(&mut self);
}