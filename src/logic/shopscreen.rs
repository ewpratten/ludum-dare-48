use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
};

use super::screen::Screen;

const SCREEN_PANEL_SIZE: Vector2 = Vector2 { x: 300.0, y: 380.0 };

pub struct Item {
    x_pose: i32,
    y_pose: i32,
    width: i32,
    height: i32,
    cost: u8,
    level: u8,
    name: String,
}

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

    // // Creates all the items
    // pub fn create_items(&mut self, screen_dimension: Vector2) {
    //     // gets every item.. hacky
    //     let items = ShopItems::get_inital_items();

    //     // sets sizes any random number is just a number I think looks good
    //     let screen_width = screen_dimension.x as f32;
    //     let screen_height = screen_dimension.y as f32;

    //     let box_height = screen_height * 0.15;
    //     let box_width = screen_width * 0.1;

    //     let start_width = screen_width - (box_width * 4.0) - 40.0;
    //     let draw_height = screen_height - 20.0 - box_height;

    //     let mut item_vec = Vec::new();

    //     for box_num in 0..4 {
    //         let x_pose = start_width + box_width * box_num as f32;

    //         // adds an item struct to the item list
    //         item_vec.push(Item {
    //             x_pose: ((x_pose + (5 * box_num) as f32) as i32),
    //             y_pose: (draw_height as i32),
    //             width: (box_width as i32),
    //             height: (box_height as i32),
    //             // Crazy hacky but this gets the data from the enum
    //             cost: (ShopItems::get_cost(&items.get(box_num).unwrap())),
    //             level: (ShopItems::get_level(&items.get(box_num).unwrap())),
    //             name: (ShopItems::get_name(&items.get(box_num).unwrap())),
    //         });
    //     }

    //     self.shop_items = item_vec;
    // }
}

impl Screen for ShopScreen {
    fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
        let mouse_position = draw_handle.get_mouse_position();
        draw_handle.clear_background(Color::GRAY);
        // TODO: Maybe we can stick some art here?

        // If escape is pressed again, return to the previous render state
        if draw_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return Some(game_core.last_state);
        }

        // Window dimensions
        let win_height = draw_handle.get_screen_height();
        let win_width = draw_handle.get_screen_width();

        // Render the backing to the menu itself
        draw_handle.draw_rectangle(
            (win_width / 2) - ((SCREEN_PANEL_SIZE.x as i32 + 6) / 2),
            (win_height / 2) - ((SCREEN_PANEL_SIZE.y as i32 + 6) / 2),
            SCREEN_PANEL_SIZE.x as i32 + 6,
            SCREEN_PANEL_SIZE.y as i32 + 6,
            Color::BLACK,
        );
        draw_handle.draw_rectangle(
            (win_width / 2) - (SCREEN_PANEL_SIZE.x as i32 / 2),
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2),
            SCREEN_PANEL_SIZE.x as i32,
            SCREEN_PANEL_SIZE.y as i32,
            Color::WHITE,
        );

        // Render heading text
        draw_handle.draw_text(
            "SHOP",
            (win_width / 2) - 80,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 10,
            40,
            Color::BLACK,
        );

        // Close and quit buttons
        let bottom_left_button_dimensions = Rectangle {
            x: (win_width as f32 / 2.0) - (SCREEN_PANEL_SIZE.x / 2.0) + 5.0,
            y: (win_height as f32 / 2.0) + (SCREEN_PANEL_SIZE.y / 2.0) - 50.0,
            width: (SCREEN_PANEL_SIZE.x / 2.0) - 15.0,
            height: 40.0,
        };
        let bottom_right_button_dimensions = Rectangle {
            x: (win_width as f32 / 2.0) + 5.0,
            y: bottom_left_button_dimensions.y,
            width: bottom_left_button_dimensions.width,
            height: bottom_left_button_dimensions.height,
        };

        // Check if the mouse is over either button
        let mouse_over_bottom_left_button =
            bottom_left_button_dimensions.check_collision_point_rec(mouse_position);
        let mouse_over_bottom_right_button =
            bottom_right_button_dimensions.check_collision_point_rec(mouse_position);

        // Render buttons
        draw_handle.draw_rectangle_lines_ex(
            bottom_left_button_dimensions,
            3,
            match mouse_over_bottom_left_button {
                true => Color::GRAY,
                false => Color::BLACK,
            },
        );
        draw_handle.draw_text(
            "Quit",
            bottom_left_button_dimensions.x as i32 + 15,
            bottom_left_button_dimensions.y as i32 + 5,
            30,
            Color::BLACK,
        );
        draw_handle.draw_rectangle_lines_ex(
            bottom_right_button_dimensions,
            3,
            match mouse_over_bottom_right_button {
                true => Color::GRAY,
                false => Color::BLACK,
            },
        );
        draw_handle.draw_text(
            "Close",
            bottom_right_button_dimensions.x as i32 + 15,
            bottom_right_button_dimensions.y as i32 + 5,
            30,
            Color::BLACK,
        );

        // Handle click actions on the buttons
        if draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            if mouse_over_bottom_left_button {
                return Some(GameState::GameQuit);
            } else if mouse_over_bottom_right_button {
                return Some(game_core.last_state);
            }
        }

        return None;
    }
}
// pub fn render_shop(
//     draw_handle: &mut RaylibDrawHandle,
//     game_core: &mut GameCore,
//     inGameScreen: &mut InGameScreen,
// ) {
//     // Pressing F exits from buying
//     if draw_handle.is_key_pressed(KeyboardKey::KEY_F) {
//         inGameScreen.current_state = InGameState::SWIMMING;
//     }

//     let mouse_position = draw_handle.get_mouse_position();

//     draw_handle.draw_text(
//         &format!("Coins: {}", game_core.player.coins),
//         15,
//         15,
//         30,
//         Color::WHITE,
//     );

//     // Draws shop boxes
//     for mut item in inGameScreen.shop.shop_items.iter_mut() {
//         // If hovering on square draw full
//         if mouse_position.x >= item.x_pose as f32
//             && mouse_position.x <= item.x_pose as f32 + item.width as f32
//             && mouse_position.y >= item.y_pose as f32
//             && mouse_position.y <= item.y_pose as f32 + item.width as f32
//         {
//             // Draw rect
//             draw_handle.draw_rectangle(
//                 item.x_pose,
//                 item.y_pose,
//                 item.width,
//                 item.height,
//                 Color::BLACK,
//             );

//             // Preform purchasing functions
//             if draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
//                 && game_core.player.coins >= item.cost as u32
//             {
//                 // Remove currency
//                 game_core.world.spend_coins(item.cost.into());
//                 game_core.player.coins -= item.cost as u32;

//                 // Upgrade item in inventory
//                 match &(item.name)[..] {
//                     "Stun Gun" => {
//                         match item.level {
//                             0 => game_core.player.inventory.stun_gun = Some(items::StunGun::lvl1()),
//                             1 => game_core.player.inventory.stun_gun = Some(items::StunGun::lvl2()),
//                             2 => game_core.player.inventory.stun_gun = Some(items::StunGun::lvl3()),
//                             _ => (return),
//                         };
//                         item.cost += 5;
//                         item.level += 1;
//                     }
//                     "Air Bag" => {
//                         match item.level {
//                             0 => {
//                                 game_core.player.inventory.air_bag = Some(items::AirBag::lvl1());
//                             }
//                             1 => {
//                                 game_core.player.inventory.air_bag = Some(items::AirBag::lvl2());
//                             }
//                             2 => {
//                                 game_core.player.inventory.air_bag = Some(items::AirBag::lvl3());
//                             }
//                             _ => (return),
//                         };
//                         item.cost += 5;
//                         item.level += 1;
//                     }
//                     "Flash Light" => {
//                         match item.level {
//                             0 => {
//                                 game_core.player.inventory.flashlight =
//                                     Some(items::Flashlight::lvl1());
//                             }
//                             1 => {
//                                 game_core.player.inventory.flashlight =
//                                     Some(items::Flashlight::lvl2());
//                             }
//                             2 => {
//                                 game_core.player.inventory.flashlight =
//                                     Some(items::Flashlight::lvl3());
//                             }
//                             _ => (return),
//                         };
//                         item.cost += 5;
//                         item.level += 1;
//                     }
//                     "Flippers" => {
//                         match item.level {
//                             0 => {
//                                 game_core.player.inventory.flippers = Some(items::Flippers::lvl1());
//                             }
//                             1 => {
//                                 game_core.player.inventory.flippers = Some(items::Flippers::lvl2());
//                             }
//                             2 => {
//                                 game_core.player.inventory.flippers = Some(items::Flippers::lvl3());
//                             }
//                             _ => (return),
//                         };
//                         item.cost += 5;
//                         item.level += 1;
//                     }
//                     _ => (return),
//                 };
//             }
//         } else {
//             // outlines if not hovered
//             draw_handle.draw_rectangle_lines(
//                 item.x_pose,
//                 item.y_pose,
//                 item.width,
//                 item.height,
//                 Color::BLACK,
//             );
//         }
//         // Draw text about object
//         draw_handle.draw_text(
//             &format!("{}: ${}", item.name, item.cost),
//             item.x_pose + 5,
//             item.y_pose + 5,
//             12,
//             Color::BLACK,
//         );
//     }
// }
