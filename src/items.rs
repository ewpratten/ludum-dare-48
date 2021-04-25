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
	pub fn lvl3() -> Self {
        Self {
            range: 80.0,
            duration: 1.0,
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
pub struct Flashlight{
	power_level: f32,
}

impl Flashlight{
    pub fn lvl1() -> Self {
        Self {
            power_level: 0.25,
        }
    }
    pub fn lvl2() -> Self {
        Self {
            power_level: 0.5,
        }
    }
	pub fn lvl3() -> Self {
        Self {
            power_level: 1.0,
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
	pub fn lvl3() -> Self {
        Self {
            speed_increase: 1.8
        }
    }
}

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
	
		[ShopItems::StunGun(0, 5, String::from("Stun Gun")), ShopItems::AirBag(0, 5, String::from("Air Bag")), 
			ShopItems::Flashlight(0, 5, String::from("Flash Light")), ShopItems::Flippers(0, 5, String::from("Flippers"))]


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
		    ShopItems::StunGun(_, x, _) => *x,
		    ShopItems::AirBag(_, x, _) => *x,
		    ShopItems::Flashlight(_, x, _) => *x,
		    ShopItems::Flippers(_, x, _) => *x
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





