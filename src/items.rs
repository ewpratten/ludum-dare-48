use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StunGun {
    pub range: f32,
    pub duration: f64,
}

impl StunGun {
    pub fn lvl1() -> Self {
        Self {
            range: 30.0,
            duration: 0.75,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            range: 60.0,
            duration: 1.25,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AirBag;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flashlight;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flippers {
    pub speed_increase: f32,
}

impl Flippers {
    pub fn lvl1() -> Self {
        Self {
            speed_increase: 1.2
        }
    }
    pub fn lvl2() -> Self {
        Self {
            speed_increase: 1.5
        }
    }
}
