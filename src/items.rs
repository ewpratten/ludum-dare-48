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
            duration: 0.5,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            range: 60.0,
            duration: 0.75,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AirBag;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flashlight;



#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ShopItems {
    StunGun(u8, u8, String),
    AirBag(u8, u8, String),
    Flashlight(u8, u8, String),
    Flippers(u8, u8, String)
}

impl ShopItems{

	pub fn get_inital_items() -> [ShopItems; 4]{
	
		[ShopItems::StunGun(1, 10, String::from("Stun Gun")), ShopItems::AirBag(1, 10, String::from("Air Bag")), 
			ShopItems::Flashlight(1, 12, String::from("Flash Light")), ShopItems::Flippers(1, 10, String::from("Flippers"))]


	}

	pub fn get_level(item: &ShopItems) -> u8{


		match item {
		    ShopItems::StunGun(x, _, _) => *x,
		    ShopItems::AirBag(x, _, _) => *x,
		    ShopItems::Flashlight(x, _, _) => *x,
		    ShopItems::Flippers(x, _, _) => *x
		}

	}


	pub fn get_cost(item: &ShopItems) -> u8{

		match item {
		    ShopItems::StunGun(x, _, _) => *x,
		    ShopItems::AirBag(x, _, _) => *x,
		    ShopItems::Flashlight(x, _, _) => *x,
		    ShopItems::Flippers(x, _, _) => *x
		}

	}

	pub fn get_name(item: &ShopItems) -> String{

		match item {
		    ShopItems::StunGun(_, _, x) => x.to_string(),
		    ShopItems::AirBag(_, _, x) => x.to_string(),
		    ShopItems::Flashlight(_, _, x) => x.to_string(),
		    ShopItems::Flippers(_, _, x) => x.to_string()
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
