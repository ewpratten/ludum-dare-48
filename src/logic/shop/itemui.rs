use crate::lib::utils::button::OnScreenButton;
use raylib::prelude::*;

pub struct ShopItemUi {
    name: String,
    current_level: u8,
    max_level: u8,
    pub cost: u32,
    pub buy_button_hovered: bool,
}

impl ShopItemUi {
    pub fn new(name: String, current_level: u8, max_level: u8, cost: u32) -> Self {
        Self {
            name,
            current_level,
            max_level,
            cost,
            buy_button_hovered: false,
        }
    }

    pub fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        bounds: Rectangle,
        can_be_bought: bool,
    ) {
        // Render the background box
        draw_handle.draw_rectangle_rec(bounds, Color::WHITE);
        draw_handle.draw_rectangle_lines_ex(bounds, 3, Color::BLACK);

        // Render the name
        draw_handle.draw_text(
            &format!("{}: {}/{}", self.name, self.current_level, self.max_level),
            bounds.x as i32 + 10,
            bounds.y as i32 + ((bounds.height as i32 - 20) / 2),
            20,
            Color::BLACK,
        );

        // Render the buy button
        let buy_button = OnScreenButton::new(
            format!("Buy - {}f", self.cost),
            Rectangle {
                x: bounds.x + bounds.width - 150.0,
                y: bounds.y + 5.0,
                width: 145.0,
                height: bounds.height - 10.0,
            },
            match can_be_bought {
                true => Color::WHITE,
                false => Color::GRAY,
            },
            Color::BLACK,
            Color::GRAY,
            20,
            true,
        );
        buy_button.render(draw_handle);
        self.buy_button_hovered = buy_button.is_hovered(draw_handle);
    }
}
