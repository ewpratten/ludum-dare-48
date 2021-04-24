use raylib::prelude::*;

use crate::{player::Player, resources::GlobalResources};

pub trait EnemyBase {
    fn render(&mut self, context_2d: &mut RaylibMode2D<RaylibDrawHandle>, resources: &mut GlobalResources);
    fn handle_logic(&mut self, player: &mut Player, dt: f64);
    fn handle_getting_attacked(&mut self);
}