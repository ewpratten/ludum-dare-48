use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StunGun {
    pub level: u32,
    pub cost: u32,
    pub range: f32,
    pub duration: f64,
	pub name: String,
}

impl StunGun {
    pub fn level_up(new_level: u32) -> Self {
        Self {
            level: new_level,
            range: (new_level * 30) as f32,
            duration: new_level as f64 * 0.25,
            cost: 5 * new_level,
			name: String::from("Stun Gun"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AirBag {
    pub level: u32,
    pub cost: u32,
    pub additonal_oxegen: u32,
	pub name: String,
}

impl AirBag {
    pub fn level_up(new_level: u32) -> Self {
        Self {
            level: new_level,
            cost: new_level * 5,
            additonal_oxegen: 15 * new_level,
			name: String::from("Air Bag"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flashlight {
    pub level: u32,
    pub cost: u32,
    pub brightness: u32,
    pub battery_life: u32,
	pub name: String,
}

impl Flashlight {
    pub fn level_up(new_level: u32) -> Self {
        Self {
            level: new_level,
            cost: new_level * 5,
            brightness: (0.5 * new_level as f64) as u32,
            battery_life: 5 * new_level,
			name: String::from("Flash Light"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flippers {
    pub level: u32,
    pub cost: u32,
    pub speed_increase: f32,
	pub name: String,
}

impl Flippers {
    pub fn level_up(new_level: u32) -> Self {
        Self {
            level: new_level,
            cost: new_level * 5,
            speed_increase: (if new_level > 1 {
                1.2 + (new_level as f32) * 0.3
            } else {
                1.2 * new_level as f32
            }),
			name: String::from("Flippers"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ShopItems {
    StunGun(u8, u8, String),
    AirBag(u8, u8, String),
    Flashlight(u8, u8, String),
    Flippers(u8, u8, String),
}

impl ShopItems {
    pub fn get_inital_items() -> [ShopItems; 4] {
        [
            ShopItems::StunGun(1, 10, String::from("Stun Gun")),
            ShopItems::AirBag(1, 10, String::from("Air Bag")),
            ShopItems::Flashlight(1, 12, String::from("Flash Light")),
            ShopItems::Flippers(1, 10, String::from("Flippers")),
        ]
    }

    pub fn get_level(item: &ShopItems) -> u8 {
        match item {
            ShopItems::StunGun(x, _, _) => *x,
            ShopItems::AirBag(x, _, _) => *x,
            ShopItems::Flashlight(x, _, _) => *x,
            ShopItems::Flippers(x, _, _) => *x,
        }
    }

    pub fn get_cost(item: &ShopItems) -> u8 {
        match item {
            ShopItems::StunGun(x, _, _) => *x,
            ShopItems::AirBag(x, _, _) => *x,
            ShopItems::Flashlight(x, _, _) => *x,
            ShopItems::Flippers(x, _, _) => *x,
        }
    }

    pub fn get_name(item: &ShopItems) -> String {
        match item {
            ShopItems::StunGun(_, _, x) => x.to_string(),
            ShopItems::AirBag(_, _, x) => x.to_string(),
            ShopItems::Flashlight(_, _, x) => x.to_string(),
            ShopItems::Flippers(_, _, x) => x.to_string(),
        }
    }
}
