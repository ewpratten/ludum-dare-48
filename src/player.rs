use raylib::math::Vector2;



#[derive(Debug, Default)]
pub struct Player {
    pub position: Vector2,
    pub direction: Vector2,
    pub coins: u32
}

impl Player {
    pub fn new() -> Self {
        Self {
            ..Default::default()
			
        }
    }
}