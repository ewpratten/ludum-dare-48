use std::marker::PhantomData;

use raylib::prelude::*;

use crate::{items::ItemBase, player::Player, world::World};

use super::itemui::ShopItemUi;

pub struct ShopItemWrapper<T: ItemBase + Clone> {
    bounds: Rectangle,
    ui: ShopItemUi,
    item: T,
}

impl<T: ItemBase + Clone> ShopItemWrapper<T> {
    pub fn new(
        item: T,
        from_inventory: &Option<T>,
        first_item_bounds: Rectangle,
        index: u8
    ) -> Self {
        // Build new bounds for the UI row
        let new_bounds = Rectangle {
            x: first_item_bounds.x,
            y: first_item_bounds.y + ((first_item_bounds.height + 5.0) * index as f32),
            width: first_item_bounds.width,
            height: first_item_bounds.height,
        };

        Self {
            bounds: new_bounds,
            ui: ShopItemUi::new(
                item.get_name(),
                match from_inventory {
                    Some(x) => x.get_level(),
                    None => 0,
                },
                3,
                item.get_cost(),
            ),
            item,
        }
    }

    pub fn get_item(&self) -> &T {
        &self.item
    }

    pub fn can_player_afford(&self, player: &Player) -> bool {
        return player.coins >= self.item.get_cost();
    }

    pub fn purchase(&self, player: &mut Player) -> T {
        // Take the currency from the player
        player.coins -= self.item.get_cost();

        // Return a clone of the item
        return self.item.clone();
    }

    pub fn user_clicked_buy(&self, draw_handle: &mut RaylibDrawHandle) -> bool {
        return self.ui.buy_button_hovered && draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);
    }

    pub fn user_hovering_row(&self, draw_handle: &mut RaylibDrawHandle) -> bool {
        return self.bounds.check_collision_point_rec(draw_handle.get_mouse_position());
    }

    pub fn render(&mut self, draw_handle: &mut RaylibDrawHandle, player: &Player) {
        self.ui.render(draw_handle, self.bounds, self.can_player_afford(player));
    }
}
