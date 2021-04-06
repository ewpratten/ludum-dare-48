use raylib::{audio::{Music, RaylibAudio}, prelude::RaylibDrawHandle};

pub struct AudioWrapper {
    music: Music,
}

impl AudioWrapper {
    pub fn new(music: Music) -> Self {
        Self { music }
    }

    pub fn play(&mut self, audio_handle: &mut RaylibAudio) {
        audio_handle.play_music_stream(&mut self.music);
    }

    pub fn stop(&mut self, audio_handle: &mut RaylibAudio) {
        audio_handle.stop_music_stream(&mut self.music);
    }

    pub fn pause(&mut self, audio_handle: &mut RaylibAudio) {
        audio_handle.pause_music_stream(&mut self.music);
    }

    pub fn update(&mut self, audio_handle: &mut RaylibAudio) {
        audio_handle.update_music_stream(&mut self.music);
    }

    pub fn is_playing(&mut self, audio_handle: &mut RaylibAudio) -> bool {
        return audio_handle.is_music_playing(&mut self.music);
    }
}
