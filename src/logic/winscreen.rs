use raylib::prelude::*;

use crate::lib::utils::button::OnScreenButton;
use crate::{
    gamecore::{GameCore, GameState},
    lib::wrappers::audio::player::AudioPlayer,
};

use super::screen::Screen;

const SCREEN_PANEL_SIZE: Vector2 = Vector2 { x: 300.0, y: 300.0 };

pub struct WinScreen {}

impl WinScreen {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Screen for WinScreen {
    fn render(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        audio_system: &mut AudioPlayer,
        game_core: &mut GameCore,
    ) -> Option<GameState> {
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
            "You've Won!!",
            (win_width / 2) - ((SCREEN_PANEL_SIZE.x as i32 + 6) / 2) + 60,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 10,
            30,
            Color::BLACK,
        );

        // Render message
        draw_handle.draw_text(
            "You can use the transponder to \ncontact help!",
            ((win_width / 2) - ((SCREEN_PANEL_SIZE.x as i32 + 6) / 2)) + (0.15 * SCREEN_PANEL_SIZE.x)as i32,
            (win_height / 2) - (SCREEN_PANEL_SIZE.y as i32 / 2) + 80,
            15,
            Color::BLACK,
        );

        // Render button
        let go_to_menu_button = OnScreenButton::new(
            String::from("Return to menu"),
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

        go_to_menu_button.render(draw_handle);

        if go_to_menu_button.is_hovered(draw_handle)
            && draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
        {
            game_core.switch_state(GameState::MainMenu, Some(draw_handle));
        }
		
        return None;
    }
}
