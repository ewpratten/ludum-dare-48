mod item;
mod itemui;
mod mainui;

use self::mainui::{render_shop, render_stats};
use super::screen::Screen;
use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
};
use raylib::prelude::*;

#[derive(Debug, Default)]
pub struct ShopScreen {
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
        // Render the background
        draw_handle.draw_texture(&game_core.resources.shop_background, 0, 0, Color::WHITE);

        if !audio_system.is_sound_playing(&game_core.resources.song_shop) {
            audio_system.play_sound(&game_core.resources.song_shop);
        }

        draw_handle.draw_text(
            &format!("you really need to get to the bottom huh?\n Well then buy my wares,\n they will help you get to your doo-hicky"),
            10,
            50,
            20,
            Color::WHITE,
        );

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
