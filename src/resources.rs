use failure::Error;
use raylib::{
    math::Vector2,
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

use crate::lib::wrappers::animation::FrameAnimationWrapper;

/// This struct contains all textures and sounds that must be loaded into (V)RAM at the start of the game
pub struct GlobalResources {
    // Branding
    pub game_logo: Texture2D,

    // Player
    pub player_animation_regular: FrameAnimationWrapper,
}

impl GlobalResources {
    /// Load all resources. **THIS WILL HANG!**
    pub fn load_all(
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<GlobalResources, String> {
        Ok(GlobalResources {
            game_logo: raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/logos/game-logo.png")?,
            )?,
            player_animation_regular: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/character/diveNormal.png")?,
                )?,
                Vector2 { x: 11.0, y: 21.0 },
                8,
                100 / 8,
            ),
        })
    }
}
