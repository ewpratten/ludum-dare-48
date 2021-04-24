use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ShopItems {
    StunGun,
    AirBag,
    Flashlight { power: u8 },
    Flippers { speed_increase: u8 },
}
