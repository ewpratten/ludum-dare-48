use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
};

use super::itemui::ShopItemUi;

pub fn render_shop(
    draw_handle: &mut RaylibDrawHandle,
    _thread: &RaylibThread,
    audio_system: &mut AudioPlayer,
    game_core: &mut GameCore,
    bounds: Rectangle,
) -> Option<GameState> {
    // Render background
    draw_handle.draw_rectangle_rec(bounds, Color::WHITE);
    draw_handle.draw_rectangle_lines_ex(bounds, 3, Color::BLACK);

    // Title
    draw_handle.draw_text(
        "SHOP",
        bounds.x as i32 + (bounds.width / 2.0) as i32 - 50,
        bounds.y as i32 + 20,
        40,
        Color::BLACK,
    );

    // Items
    let stun_gun_buy_ui = ShopItemUi::new(
        "Stun Gun".to_string(),
        match &game_core.player.inventory.stun_gun {
            Some(x) => x.level,
            None => 0,
        },
        3,
        10,
    );
    stun_gun_buy_ui.render(
        draw_handle,
        Rectangle {
            x: bounds.x + 5.0,
            y: bounds.y + 100.0,
            width: bounds.width - 10.0,
            height: 50.0,
        },
        game_core.player.coins >= stun_gun_buy_ui.cost,
    );

    return None;
}

pub fn render_stats(
    draw_handle: &mut RaylibDrawHandle,
    game_core: &mut GameCore,
    bounds: Rectangle,
) {
    // Render background
    draw_handle.draw_rectangle_rec(bounds, Color::WHITE);
    draw_handle.draw_rectangle_lines_ex(bounds, 3, Color::BLACK);

    // Coins
    draw_handle.draw_text(
        &format!("Fish: {}", game_core.player.coins.min(99)),
        bounds.x as i32 + 5,
        bounds.y as i32 + 5,
        20,
        Color::BLACK,
    );
}
