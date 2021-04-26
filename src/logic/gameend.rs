use raylib::prelude::*;

use crate::{
    gamecore::{GameCore, GameState},
    lib::{utils::button::OnScreenButton, wrappers::audio::player::AudioPlayer},
};

use super::screen::Screen;

const SCREEN_PANEL_SIZE: Vector2 = Vector2 { x: 300.0, y: 300.0 };

pub struct GameEndScreen {}

impl GameEndScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for GameEndScreen {
    fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        _audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
        draw_handle.clear_background(Color::GRAY);
        // TODO: Maybe we can stick some art here?

        // Render the background
        draw_handle.draw_texture(&game_core.resources.shop_background, 0, 0, Color::WHITE);

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
            "OUT OF BREATH",
            (win_width / 2) - ((SCREEN_PANEL_SIZE.x as i32 + 6) / 2) + 25,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 10,
            30,
            Color::BLACK,
        );

        // Render message
        draw_handle.draw_text(
            "Your clone can now buy items ",
            ((win_width / 2) - ((SCREEN_PANEL_SIZE.x as i32 + 6) / 2))
                + (0.15 * SCREEN_PANEL_SIZE.x) as i32,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 50,
            15,
            Color::BLACK,
        );

        // Creates
        let go_to_menu_button = OnScreenButton::new(
            String::from("Return to shop"),
            Rectangle {
                x: (((win_width / 2) - ((SCREEN_PANEL_SIZE.x as i32 + 6) / 2) + 5)
                    + (0.15 * SCREEN_PANEL_SIZE.x) as i32) as f32,
                y: (((win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 90) as f32) + 100.0,
                width: 210.0,
                height: 50.0,
            },
            Color::WHITE,
            Color::BLACK,
            Color::GRAY,
            25,
            true,
        );

        // render button
        go_to_menu_button.render(draw_handle);

        // If the player clicks on the button send them to shop
        if go_to_menu_button.is_hovered(draw_handle)
            && draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
        {
            return Some(GameState::InShop);
        }

        return None;
    }
}
