use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    items::{AirBag, Flashlight, Flippers, ItemBase, StunGun},
    lib::{utils::button::OnScreenButton, wrappers::audio::player::AudioPlayer},
};

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
    let mut stun_gun_buy_ui = ShopItemWrapper::new(
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
    let mut air_bag_buy_ui = ShopItemWrapper::new(
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
    let mut flashlight_buy_ui = ShopItemWrapper::new(
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
    let mut flippers_buy_ui = ShopItemWrapper::new(
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
    stun_gun_buy_ui.render(draw_handle, &game_core.player, &game_core.player.inventory.stun_gun);
    air_bag_buy_ui.render(draw_handle, &game_core.player, &game_core.player.inventory.air_bag);
    flashlight_buy_ui.render(draw_handle, &game_core.player, &game_core.player.inventory.flashlight);
    flippers_buy_ui.render(draw_handle, &game_core.player, &game_core.player.inventory.flippers);

    // Handle buying items
    if stun_gun_buy_ui.can_player_afford(&game_core.player, &game_core.player.inventory.stun_gun)
        && stun_gun_buy_ui.user_clicked_buy(draw_handle)
    {
        let item = stun_gun_buy_ui.purchase(&mut game_core.player);
        game_core.player.inventory.stun_gun = Some(item);
    }
    if air_bag_buy_ui.can_player_afford(&game_core.player, &game_core.player.inventory.air_bag)
        && air_bag_buy_ui.user_clicked_buy(draw_handle)
    {
        let item = air_bag_buy_ui.purchase(&mut game_core.player);
        game_core.player.inventory.air_bag = Some(item);
    }
    if flashlight_buy_ui.can_player_afford(&game_core.player, &game_core.player.inventory.flashlight)
        && flashlight_buy_ui.user_clicked_buy(draw_handle)
    {
        let item = flashlight_buy_ui.purchase(&mut game_core.player);
        game_core.player.inventory.flashlight = Some(item);
    }
    if flippers_buy_ui.can_player_afford(&game_core.player, &game_core.player.inventory.flippers)
        && flippers_buy_ui.user_clicked_buy(draw_handle)
    {
        let item = flippers_buy_ui.purchase(&mut game_core.player);
        game_core.player.inventory.flippers = Some(item);
    }

    // Render the tooltip box
    let hovering_stun_gun = stun_gun_buy_ui.user_hovering_row(draw_handle);
    let hovering_air_bag = air_bag_buy_ui.user_hovering_row(draw_handle);
    let hovering_flashlight = flashlight_buy_ui.user_hovering_row(draw_handle);
    let hovering_flippers = flippers_buy_ui.user_hovering_row(draw_handle);
    let should_show_tooltip =
        hovering_stun_gun || hovering_air_bag || hovering_flashlight || hovering_flippers;

    if should_show_tooltip {
        // Create bounds
        let box_bounds = Rectangle {
            x: bounds.x + 5.0,
            y: bounds.y + 350.0,
            width: bounds.width - 10.0,
            height: 250.0,
        };

        // Get the hovered item
        let hovered_item: &dyn ItemBase;
        if hovering_stun_gun {
            hovered_item = stun_gun_buy_ui.get_item();
        } else if hovering_air_bag {
            hovered_item = air_bag_buy_ui.get_item();
        } else if hovering_flashlight {
            hovered_item = flashlight_buy_ui.get_item();
        } else {
            hovered_item = flippers_buy_ui.get_item();
        }

        // Draw background box
        draw_handle.draw_rectangle_rec(box_bounds, Color::WHITE);
        draw_handle.draw_rectangle_lines_ex(box_bounds, 3, Color::BLACK);

        // TODO: draw item sprite
        draw_handle.draw_rectangle_v(
            Vector2 {
                x: box_bounds.x + (box_bounds.width / 2.0) - 40.0,
                y: box_bounds.y + 10.0,
            },
            Vector2 { x: 80.0, y: 80.0 },
            Color::BLACK,
        );

        // Render item description
        draw_handle.draw_text(
            &hovered_item.get_description(),
            box_bounds.x as i32 + 10,
            box_bounds.y as i32 + 100,
            30,
            Color::BLACK,
        );
    }

    // Handle exit buttons
    let bottom_left_button_dimensions = Rectangle {
        x: bounds.x + 5.0,
        y: bounds.y + bounds.height - 50.0,
        width: (bounds.width / 2.0) - 15.0,
        height: 40.0,
    };
    let bottom_right_button_dimensions = Rectangle {
        x: (bounds.x + bottom_left_button_dimensions.width) + 15.0,
        y: bottom_left_button_dimensions.y,
        width: bottom_left_button_dimensions.width,
        height: bottom_left_button_dimensions.height,
    };

    let menu_button = OnScreenButton::new(
        "Menu".to_string(),
        bottom_left_button_dimensions,
        Color::WHITE,
        Color::BLACK,
        Color::GRAY,
        30,
        true,
    );
    let play_button = OnScreenButton::new(
        "Play".to_string(),
        bottom_right_button_dimensions,
        Color::WHITE,
        Color::BLACK,
        Color::GRAY,
        30,
        true,
    );

    // Render both
    menu_button.render(draw_handle);
    play_button.render(draw_handle);

    // Handle click actions on the buttons
    if draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        // Handle saving core state
        if menu_button.is_hovered(draw_handle) || play_button.is_hovered(draw_handle) {
            let new_progress = game_core
                .player
                .create_statistics(game_core, draw_handle.get_time());
            game_core.progress.update(&new_progress);
        }

        if menu_button.is_hovered(draw_handle) {
            return Some(GameState::MainMenu);
        } else if play_button.is_hovered(draw_handle) {
            // Reset the world
            game_core.world.reset(&mut game_core.player);

            // Start playing
            return Some(GameState::InGame);
        }
    }

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
        &format!("Fish: {}", game_core.player.coins.min(999)),
        bounds.x as i32 + 5,
        bounds.y as i32 + 5,
        20,
        Color::BLACK,
    );
}
