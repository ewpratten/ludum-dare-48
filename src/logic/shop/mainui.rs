use raylib::prelude::*;

use crate::{gamecore::{GameCore, GameState}, items::{AirBag, Flashlight, Flippers, ItemBase, StunGun}, lib::wrappers::audio::player::AudioPlayer};

use super::{item::ShopItemWrapper, itemui::ShopItemUi};

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

    // Bounds for use in item row sizing
    let first_bounds = Rectangle {
        x: bounds.x + 5.0,
        y: bounds.y + 100.0,
        width: bounds.width - 10.0,
        height: 50.0,
    };

    // Create items
    let stun_gun_buy_ui = ShopItemWrapper::new(
        match &game_core.player.inventory.stun_gun {
            Some(x) => match x.get_level() {
                1 => StunGun::lvl2(),
                _ => StunGun::lvl3(),
            },
            None => StunGun::lvl1(),
        },
        &game_core.player.inventory.stun_gun,
        first_bounds,
        0,
    );
    let air_bag_buy_ui = ShopItemWrapper::new(
        match &game_core.player.inventory.air_bag {
            Some(x) => match x.get_level() {
                1 => AirBag::lvl2(),
                _ => AirBag::lvl3(),
            },
            None => AirBag::lvl1(),
        },
        &game_core.player.inventory.air_bag,
        first_bounds,
        1,
    );
    let flashlight_buy_ui = ShopItemWrapper::new(
        match &game_core.player.inventory.flashlight {
            Some(x) => match x.get_level() {
                1 => Flashlight::lvl2(),
                _ => Flashlight::lvl3(),
            },
            None => Flashlight::lvl1(),
        },
        &game_core.player.inventory.flashlight,
        first_bounds,
        2,
    );
    let flippers_buy_ui = ShopItemWrapper::new(
        match &game_core.player.inventory.flippers {
            Some(x) => match x.get_level() {
                1 => Flippers::lvl2(),
                _ => Flippers::lvl3(),
            },
            None => Flippers::lvl1(),
        },
        &game_core.player.inventory.flippers,
        first_bounds,
        3,
    );

    // Render items
    stun_gun_buy_ui.render(draw_handle, &game_core.player);
    air_bag_buy_ui.render(draw_handle, &game_core.player);
    flashlight_buy_ui.render(draw_handle, &game_core.player);
    flippers_buy_ui.render(draw_handle, &game_core.player);


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
