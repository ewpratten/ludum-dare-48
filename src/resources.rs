use failure::Error;

/// This struct contains all textures and sounds that must be loaded into (V)RAM at the start of the game
pub struct GlobalResources {}

impl GlobalResources {
    /// Load all resources. **THIS WILL HANG!**
    pub fn load_all(&mut self) -> Result<GlobalResources, Error> {
        Ok(GlobalResources {})
    }
}
