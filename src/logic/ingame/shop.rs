use std::u8;

use raylib::prelude::*;

use crate::{gamecore::GameCore, items::ShopItems};

use super::{InGameScreen, InGameState};


pub struct Item{

	x_pose: i32,
	y_pose: i32,
	width: i32,
	height: i32,
	cost: u8,
	level: u8,
	name: String,
	

}


pub struct Shop{

	shop_items: Vec<Item>,

}

impl Shop {

	pub fn new() -> Self {
        Self {
            shop_items: Vec::new(),
			
		}
	}

	// Creates all the items
	pub fn create_items(&mut self, screen_dimension: Vector2){

		// gets every item.. hacky
		let items = ShopItems::get_inital_items();

		// sets sizes any random number is just a number I think looks good
		let screen_width = screen_dimension.x as f32;
		let screen_height = screen_dimension.y as f32;

		let box_height = screen_height * 0.15;
		let box_width = screen_width * 0.1;

		let start_width = screen_width - (box_width * 4.0) - 40.0;
		let draw_height = screen_height - 20.0 - box_height;

		let mut item_vec = Vec::new();
		
		for box_num in 0..4 {
			let x_pose = start_width + box_width * box_num as f32;

			// adds an item struct to the item list
			item_vec.push(Item{
				x_pose: (x_pose as i32),
				y_pose: (draw_height as i32),
				width: (box_width as i32),
				height: (box_height as i32),
				// Crazy hacky but this gets the data from the enum
				cost: (ShopItems::get_cost(&items.get(box_num).unwrap())),
				level: (ShopItems::get_level(&items.get(box_num).unwrap())),
				name: (ShopItems::get_name(&items.get(box_num).unwrap())),				
			});
	
		}

		self.shop_items = item_vec;
	}
}


pub fn render_shop(
	draw_handle: &mut RaylibDrawHandle, 
	game_core: &mut GameCore,
	inGameScreen: &mut InGameScreen,
	) {
	
	

	// Pressing F exits from buying
	if draw_handle.is_key_pressed(KeyboardKey::KEY_F){
		inGameScreen.current_state = InGameState::SWIMMING;
	}


	// Draws shop boxes
	for item in inGameScreen.shop.shop_items.iter() {
		// Draws in accordance to the struct
		draw_handle.draw_rectangle(item.x_pose, item.y_pose, item.width, item.height, Color::BLACK);


	}




}

pub fn shop_logic() {

	






}
