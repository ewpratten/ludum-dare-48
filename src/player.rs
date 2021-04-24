use raylib::math::Vector2;

#[derive(Debug, Default)]
pub struct Player {
    pub position: Vector2,
    pub direction: Vector2,
    pub size: Vector2,
    pub coins: u32,
    pub boost_percent: f32,
    pub breath_percent: f32,
    pub is_moving: bool,
    pub is_boosting: bool,
    pub is_boost_charging: bool,
}

impl Player {
    pub fn new(spawn: &Vector2) -> Self {
        Self {
            boost_percent: 1.0,
            size: Vector2 { x: 11.0, y: 21.0 },
            breath_percent: 1.0,
            position: spawn.clone(),
            ..Default::default()
        }
    }
}
