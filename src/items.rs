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
pub struct AirBag{
	extra_oxygen: u32,
}

impl AirBag {
    pub fn lvl1() -> Self {
        Self {
            extra_oxygen: 15,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            extra_oxygen: 30,
        }
    }
	pub fn lvl3() -> Self {
        Self {
            extra_oxygen: 45,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flashlight {
    pub radius: f32
}

impl Flashlight {
    pub fn lvl1() -> Self {
        Self {
            radius: 0.25
        }
    }
}

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
