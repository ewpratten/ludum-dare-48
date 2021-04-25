mod item;
mod itemui;
mod mainui;

use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
};

use self::mainui::{render_shop, render_stats};

use super::screen::Screen;

const SCREEN_PANEL_SIZE: Vector2 = Vector2 { x: 300.0, y: 380.0 };

#[derive(Debug, Default)]
pub struct ShopScreen {
    // shop_items: Vec<Item>,
}

impl ShopScreen {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Screen for ShopScreen {
    fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
        let mouse_position = draw_handle.get_mouse_position();

        // Render the background
        draw_handle.draw_texture(&game_core.resources.shop_background, 0, 0, Color::WHITE);

        // Window dimensions
        let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();

        // Build a rect for the shop UI to sit inside
        let shop_ui_bounds = Rectangle {
            x: win_width as f32 - (win_width as f32 / 2.0),
            y: 10.0,
            width: (win_width as f32 / 2.0) - 10.0,
            height: win_height as f32 - 20.0,
        };
        let stats_ui_bounds = Rectangle {
            x: win_width as f32 - (win_width as f32 / 2.0) - 130.0,
            y: 10.0,
            width: 120.0,
            height: 30.0,
        };

        // Render the shop UI
        let next_state = render_shop(draw_handle, thread, audio_system, game_core, shop_ui_bounds);

        // Render the stats UI
        render_stats(draw_handle, game_core, stats_ui_bounds);

        return next_state;
    }
}
