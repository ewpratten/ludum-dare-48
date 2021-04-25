use raylib::prelude::*;

pub struct OnScreenButton {
    bounds: Rectangle,
    text: String,
    background: Color,
    border: Color,
    border_hover: Color,
    font_px: i32,
    draw_border: bool,
}

impl OnScreenButton {
    pub fn new(
        text: String,
        bounds: Rectangle,
        background: Color,
        border: Color,
        border_hover: Color,
        font_px: i32,
        draw_border: bool,
    ) -> Self {
        Self {
            bounds,
            text,
            background,
            border,
            border_hover,
            font_px,
            draw_border,
        }
    }

    pub fn is_hovered(&self, draw_handle: &RaylibDrawHandle) -> bool {
        return self
            .bounds
            .check_collision_point_rec(draw_handle.get_mouse_position());
    }

    pub fn render(&self, draw_handle: &mut RaylibDrawHandle) {
        // Draw the button background
        draw_handle.draw_rectangle_rec(self.bounds, self.background);

        // Check mouse info
        let is_being_hovered = self.is_hovered(draw_handle);

        // Render the border
        if self.draw_border {
            draw_handle.draw_rectangle_lines_ex(
                self.bounds,
                3,
                match is_being_hovered {
                    true => self.border_hover,
                    false => self.border,
                },
            );
        }

        // Render the text
        draw_handle.draw_text(
            &self.text,
            self.bounds.x as i32 + 10,
            self.bounds.y as i32 + ((self.bounds.height as i32 - self.font_px) / 2),
            self.font_px,
            match is_being_hovered {
                true => self.border_hover,
                false => self.border,
            },
        )
    }
}
