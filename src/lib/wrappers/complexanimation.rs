use std::usize;

use raylib::prelude::*;

pub struct FrameRange {
    pub min: usize,
    pub max: usize,
}

pub struct ComplexAnimationTool {
    sprite_sheet: Texture2D,
    frames_per_second: f32,
    frame_size: Vector2,
    sprite_sheet_size_frames: Vector2
}

impl ComplexAnimationTool {
    pub fn render_loop(&self, context_2d: &mut RaylibMode2D<RaylibDrawHandle>, bounds: Rectangle, rotation: f32, range: &FrameRange) {

    }

    pub fn render_frame(&self, context_2d: &mut RaylibMode2D<RaylibDrawHandle>, bounds: Rectangle, rotation: f32, id: usize) {

        // Convert the ID to an xy
        let col_id = id % self.sprite_sheet_size_frames.x as usize;
        let row_id = id / self.sprite_sheet_size_frames.y as usize;

        


    }
}
