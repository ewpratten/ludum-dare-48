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
    pub player_animation_boost_charge: FrameAnimationWrapper,
    pub player_animation_boost: FrameAnimationWrapper,
    pub player_animation_stunned: FrameAnimationWrapper,
    
    // Fish
    pub fish_animation_idle: FrameAnimationWrapper,
    pub fish_animation_swim: FrameAnimationWrapper,

    // Cave
    pub cave_mid_layer: Texture2D,

    // Enemies
    pub jellyfish_animation_regular: FrameAnimationWrapper,
    pub jellyfish_animation_attack: FrameAnimationWrapper,

    // Darkness layer
    pub darkness_overlay: Texture2D
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
            player_animation_boost_charge: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/character/diveStrokeCharge.png")?,
                )?,
                Vector2 { x: 11.0, y: 21.0 },
                21,
                100 / 4,
            ),
            player_animation_boost: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/character/diveStroke.png")?,
                )?,
                Vector2 { x: 17.0, y: 21.0 },
                21,
                30,
            ),
            player_animation_stunned: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/character/stunned.png")?,
                )?,
                Vector2 { x: 12.0, y: 22.0 },
                4,
                100 / 8,
            ),
            fish_animation_idle: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/map/fishIdle.png")?,
                )?,
                Vector2 { x: 13.0, y: 9.0 },
                14,
                4,
            ),
            fish_animation_swim: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/map/fish.png")?,
                )?,
                Vector2 { x: 13.0, y: 9.0 },
                63,
                30,
            ),
            cave_mid_layer: raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/map/cave.png")?,
            )?,
            jellyfish_animation_regular: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/enemies/jelly.png")?,
                )?,
                Vector2 { x: 10.0, y: 10.0 },
                6,
                4,
            ),
            jellyfish_animation_attack: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/enemies/jellyAttack.png")?,
                )?,
                Vector2 { x: 20.0, y: 20.0 },
                15,
                4,
            ),
            darkness_overlay: raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/map/darkness.png")?,
            )?,
        })
    }
}
