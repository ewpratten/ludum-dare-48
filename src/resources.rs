use raylib::{
    math::Vector2,
    shaders::Shader,
    texture::{Image, RenderTexture2D, Texture2D},
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
    pub pixel_shader: Shader,
    pub shader_texture: RenderTexture2D,

    // Enemies
    pub jellyfish_animation_regular: FrameAnimationWrapper,
    pub jellyfish_animation_attack: FrameAnimationWrapper,
    pub octopus_animation_regular: FrameAnimationWrapper,
    pub octopus_animation_attack: FrameAnimationWrapper,
	pub whirlpool: FrameAnimationWrapper,
	pub pufferfish_big: FrameAnimationWrapper,
	pub pufferfish_small: FrameAnimationWrapper,
	pub pufferfish_attack: FrameAnimationWrapper,
	pub pufferfish_expand: FrameAnimationWrapper,

    // Darkness layer
    pub darkness_overlay: Texture2D,

    // Backgrounds
    pub background_front: Texture2D,
    pub background_back: Texture2D,

    // Shop & items
    pub shop_background: Texture2D,

    pub flashlight_one: Texture2D,
    pub flashlight_two: Texture2D,
    pub flashlight_three: Texture2D,

    pub stun_gun_one: Texture2D,
    pub stun_gun_two: Texture2D,
    pub stun_gun_three: Texture2D,

    pub air_one: Texture2D,
    pub air_two: Texture2D,
    pub air_three: Texture2D,

    pub flippers_one: Texture2D,
    pub flippers_two: Texture2D,
    pub flippers_three: Texture2D,

    // Treasure
    pub transponder: FrameAnimationWrapper,
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
                    &Image::load_image("./assets/img/map/fishStill.png")?,
                )?,
                Vector2 { x: 13.0, y: 9.0 },
                14,
                30,
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
            pixel_shader: raylib.load_shader(&thread, None, Some("./assets/shaders/pixel.fs"))?,
            shader_texture: raylib.load_render_texture(
                &thread,
                raylib.get_screen_width() as u32,
                raylib.get_screen_height() as u32,
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
            octopus_animation_regular: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/enemies/octopus.png")?,
                )?,
                Vector2 { x: 20.0, y: 20.0 },
                6,
                4,
            ),
            octopus_animation_attack: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/enemies/octopusSuck.png")?,
                )?,
                Vector2 { x: 30.0, y: 20.0 },
                4,
                4,
            ),
            darkness_overlay: raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/map/darkness.png")?,
            )?,
            background_front: raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/map/backFront.png")?,
            )?,
            background_back: raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/map/backBack.png")?,
            )?,
            shop_background: raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/map/shopHighRes.png")?,
            )?,
            flashlight_one: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/flashlight1.png")?,
            )?),
            flashlight_two: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/flashlight2.png")?,
            )?),
            flashlight_three: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/flashlight3.png")?,
            )?),
            stun_gun_one: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/stun1.png")?,
            )?),
            stun_gun_two: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/stun2.png")?,
            )?),
            stun_gun_three: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/stun3.png")?,
            )?),
            air_one: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/air1.png")?,
            )?),
            air_two: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/air2.png")?,
            )?),
            air_three: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/air3.png")?,
            )?),
            flippers_one: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/flippers1.png")?,
            )?),
            flippers_two: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/flippers2.png")?,
            )?),
            flippers_three: (raylib.load_texture_from_image(
                &thread,
                &Image::load_image("./assets/img/items/flippers3.png")?,
            )?),
            transponder: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/map/transponder.png")?,
                )?,
                Vector2 { x: 10.0, y: 20.0 },
                6,
                2,
            ),
            whirlpool: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/enemies/whirlpool.png")?,
                )?,
                Vector2 { x: 20.0, y: 20.0 },
                4,
                4,
			),
            pufferfish_big: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/enemies/pufferFishBigIdle.png")?,
                )?,
                Vector2 { x: 19.0, y: 19.0 },
                3,
                2,
			),
            pufferfish_small: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/enemies/pufferFishIdle.png")?,
                )?,
                Vector2 { x: 19.0, y: 19.0 },
                6,
                2,
			),
            pufferfish_attack: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/enemies/pufferFishAttack.png")?,
                )?,
                Vector2 { x: 39.0, y: 25.0 },
                4,
                2,
			),
            pufferfish_expand: FrameAnimationWrapper::new(
                raylib.load_texture_from_image(
                    &thread,
                    &Image::load_image("./assets/img/enemies/pufferFishExpand.png")?,
                )?,
                Vector2 { x: 19.0, y: 19.0 },
                4,
                2,
			),
        })
    }
}
