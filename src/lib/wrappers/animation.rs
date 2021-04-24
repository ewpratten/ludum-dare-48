use raylib::{
    core::color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

/// A wrapper around an animation spritesheet
pub struct FrameAnimationWrapper {
    sprite_sheet: Texture2D,
    size: Vector2,
    frame_count: u32,
    frames_per_second: u8,
    start_time_seconds: f64,
}

impl FrameAnimationWrapper {

    /// Create a new animation from a texture containing all frames
    pub fn new(sprite_sheet: Texture2D, frame_size: Vector2, frame_count: u32, fps: u8) -> Self {
        Self {
            sprite_sheet,
            size: frame_size,
            frame_count,
            frames_per_second: fps,
            start_time_seconds: 0.0,
        }
    }

    /// Start the animation
    pub fn start(&mut self, handle: &RaylibDrawHandle) {
        self.start_time_seconds = handle.get_time();
    }

    /// Stop (and reset) the animation
    pub fn stop(&mut self) {
        self.start_time_seconds = 0.0;
    }

    /// Get the index of the currently displayed frame
    pub fn get_current_frame_id(&self, handle: &RaylibDrawHandle) -> u32 {
        // Get the time since start
        let time_since_start = handle.get_time() - self.start_time_seconds;

        // Determine the frame ID
        return ((time_since_start * self.frames_per_second as f64) % self.frame_count as f64)
            as u32;
    }

    /// Draw the next frame to the screen at `position`
    pub fn draw(&mut self, handle: &mut RaylibDrawHandle, position: Vector2, rotation: f32) {
        let frame_id = self.get_current_frame_id(handle);
        self.draw_frame(handle, position, rotation, frame_id);
    }

    /// Draw a specified frame to the screen at `position`
    pub fn draw_frame(
        &mut self,
        handle: &mut RaylibDrawHandle,
        position: Vector2,
        rotation: f32,
        frame_number: u32,
    ) {
        // Determine the col number
        let col = self.size.x * frame_number as f32;

        // Determine the row number
        let frames_per_row = self.sprite_sheet.width as u32 / self.size.x as u32;
        let row_number = frame_number / frames_per_row;
        let row = row_number as f32 * self.size.y;

        // Build a relative bounding box for this single frame
        let frame_box = Rectangle {
            x: col,
            y: row,
            width: self.size.x,
            height: self.size.y,
        };

        // Rotation origin
        let origin = Vector2 {
            x: self.size.x,
            y: self.size.y
        };

        // Render
        handle.draw_texture_pro(&mut self.sprite_sheet, frame_box, position, origin, rotation, Color::WHITE);
    }
}
