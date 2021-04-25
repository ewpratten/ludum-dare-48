pub mod profiler;
pub mod triangles;
pub mod button;

pub fn calculate_linear_slide(playthrough_percent: f64) -> f64 {
    if playthrough_percent < 0.25 {
        return playthrough_percent / 0.25;
    } else if playthrough_percent > 0.75 {
        return 1.0 - ((playthrough_percent - 0.75) / 0.25);
    } else {
        return 1.0;
    }
}