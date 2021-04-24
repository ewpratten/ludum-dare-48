use serde::{Deserialize, Serialize};

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// #[serde(tag = "t", content = "c")]
// pub enum ShopItems {
//     StunGun,
//     AirBag,
//     Flashlight { power: u8 },
//     Flippers { speed_increase: u8 },
// }


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StunGun {
    pub range: f32,
    pub duration: f64
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AirBag;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flashlight;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flippers {
    pub speed_increase: f32
}