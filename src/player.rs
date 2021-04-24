use raylib::math::Vector2;



#[derive(Debug, Default)]
pub struct Player {
    pub position: Vector2,
    pub direction: Vector2,
    pub size: Vector2,
    pub coins: u32,
    pub boost_percent: f32,
    pub breath_percent: f32
}

impl Player {
    pub fn new() -> Self {
        Self {
            boost_percent: 1.0,
            size: Vector2 {
                x: 11.0 * 4.0,
                y: 21.0 * 4.0
            },
            breath_percent: 0.5,
            ..Default::default()
			
        }
    }
}