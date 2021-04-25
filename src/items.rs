use raylib::texture::Texture2D;
use serde::{Deserialize, Serialize};

pub trait ItemBase {
    fn get_cost(&self) -> u32;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_texture(&self) -> &Texture2D;
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StunGun {
    pub range: f32,
    pub duration: f64,
    pub level: u8,
    cost: u32,
}

impl StunGun {
    pub fn lvl1() -> Self {
        Self {
            range: 30.0,
            duration: 0.75,
            level: 1,
            cost: 30,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            range: 60.0,
            duration: 1.25,
            level: 2,
            cost: 40,
        }
    }
    pub fn lvl3() -> Self {
        Self {
            range: 80.0,
            duration: 1.0,
            level: 3,
            cost: 50,
        }
    }
}

impl ItemBase for StunGun {
    fn get_cost(&self) -> u32 {
        self.cost
    }

    fn get_name(&self) -> String {
        return "Stun Gun".to_string();
    }

    fn get_description(&self) -> String {
        return "Stun your enemies! Just don't point it at yourself.".to_string();
    }

    fn get_texture(&self) -> &Texture2D {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AirBag {
    extra_oxygen: u32,
    pub level: u8,
    cost: u32,
}

impl AirBag {
    pub fn lvl1() -> Self {
        Self {
            extra_oxygen: 15,
            level: 1,
            cost: 30,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            extra_oxygen: 30,
            level: 2,
            cost: 40,
        }
    }
    pub fn lvl3() -> Self {
        Self {
            extra_oxygen: 45,
            level: 3,
            cost: 50,
        }
    }
}

impl ItemBase for AirBag {
    fn get_cost(&self) -> u32 {
        self.cost
    }

    fn get_name(&self) -> String {
        return "Bag of Air".to_string();
    }

    fn get_description(&self) -> String {
        return "Its.. a bag. Filled with air. Duh".to_string();
    }

    fn get_texture(&self) -> &Texture2D {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flashlight {
    pub radius: f32,
    pub level: u8,
    cost: u32,
}

impl Flashlight {
    pub fn lvl1() -> Self {
        Self {
            radius: 0.25,
            level: 1,
            cost: 40,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            radius: 0.5,
            level: 2,
            cost: 50,
        }
    }
    pub fn lvl3() -> Self {
        Self {
            radius: 1.0,
            level: 3,
            cost: 60,
        }
    }
}

impl ItemBase for Flashlight {
    fn get_cost(&self) -> u32 {
        self.cost
    }

    fn get_name(&self) -> String {
        return "Flashlight".to_string();
    }

    fn get_description(&self) -> String {
        return "See better for longer".to_string();
    }

    fn get_texture(&self) -> &Texture2D {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flippers {
    pub speed_increase: f32,
    pub level: u8,
    cost: u32,
}

impl Flippers {
    pub fn lvl1() -> Self {
        Self {
            speed_increase: 1.2,
            level: 1,
            cost: 30,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            speed_increase: 1.5,
            level: 2,
            cost: 40,
        }
    }
    pub fn lvl3() -> Self {
        Self {
            speed_increase: 1.8,
            level: 3,
            cost: 50,
        }
    }
}

impl ItemBase for Flippers {
    fn get_cost(&self) -> u32 {
        self.cost
    }

    fn get_name(&self) -> String {
        return "Flippers".to_string();
    }

    fn get_description(&self) -> String {
        return "Swim faster, and look stupid at the same time!".to_string();
    }

    fn get_texture(&self) -> &Texture2D {
        todo!()
    }
}
