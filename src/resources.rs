use failure::Error;
use raylib::{RaylibHandle, RaylibThread, texture::{Image, Texture2D}};

/// This struct contains all textures and sounds that must be loaded into (V)RAM at the start of the game
pub struct GlobalResources {

    // Branding
    pub game_logo: Texture2D

}

impl GlobalResources {
    /// Load all resources. **THIS WILL HANG!**
    pub fn load_all(raylib: &mut RaylibHandle, thread: &RaylibThread) -> Result<GlobalResources, String> {
        Ok(GlobalResources {
            game_logo: raylib.load_texture_from_image(&thread, &Image::load_image("./assets/img/logos/game-logo.png")?)?
        })
    }
}
