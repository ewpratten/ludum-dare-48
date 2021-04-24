use serde_json::json;
use serialstudio::{
    data::{DataGroup, DataSet, TelemetryFrame},
    SerialStudioSource,
};

#[derive(Default)]
pub struct ProfilerData {
    // Rendering
    pub frames_per_second: u32,
    pub seconds_per_frame: f32,
    pub monitor_count: i32,

    // Audio
    pub audio_volume: f32,
    pub active_sounds: i32,

    // Game core
    pub game_state: String,

    // Player 
    pub player_coins: u32,
    pub player_boost_percent: f32,
    pub player_breath_percent: f32
}

/// The development profiler
#[derive(Default)]
pub struct GameProfiler {
    /// The SerialStudio server
    server: Option<SerialStudioSource>,

    /// The data
    pub data: ProfilerData,
}

/// Dev mode
#[cfg(debug_assertions)]
impl GameProfiler {
    pub fn new() -> Self {
        Self {
            server: Some(SerialStudioSource::new()),
            ..Default::default()
        }
    }

    pub fn start(&mut self) {
        println!("Starting debug server on: tcp://localhost:8019");
        self.server
            .as_mut()
            .unwrap()
            .start("localhost:8019".to_string());
    }

    pub fn stop(&mut self) {
        println!("Stopping debug server");
        self.server.as_mut().unwrap().stop();
    }

    pub fn update(&mut self) {
        // Build telemetry frame
        let frame = TelemetryFrame {
            title: "Game status".to_string(),
            groups: vec![
                DataGroup {
                    title: "Rendering engine".to_string(),
                    widget_type: None,
                    datasets: vec![
                        DataSet {
                            title: Some("Frames per Second".to_string()),
                            value: json!(self.data.frames_per_second),
                            graph: Some(true),
                            unit: Some("fps".to_string()),
                            w_type: None,
                        },
                        DataSet {
                            title: Some("Seconds per Frame".to_string()),
                            value: json!(self.data.seconds_per_frame),
                            graph: Some(true),
                            unit: Some("seconds".to_string()),
                            w_type: None,
                        },
                        DataSet {
                            title: Some("Monitor Count".to_string()),
                            value: json!(self.data.monitor_count),
                            graph: Some(false),
                            unit: None,
                            w_type: None,
                        },
                    ],
                },
                DataGroup {
                    title: "Audio engine".to_string(),
                    widget_type: None,
                    datasets: vec![
                        DataSet {
                            title: Some("Master Volume".to_string()),
                            value: json!(self.data.audio_volume),
                            graph: Some(false),
                            unit: None,
                            w_type: None,
                        },
                        DataSet {
                            title: Some("Active Sounds".to_string()),
                            value: json!(self.data.active_sounds),
                            graph: Some(true),
                            unit: None,
                            w_type: None,
                        },
                    ],
                },
                DataGroup {
                    title: "Game".to_string(),
                    widget_type: None,
                    datasets: vec![
                        DataSet {
                            title: Some("Global State".to_string()),
                            value: json!(self.data.game_state),
                            graph: Some(false),
                            unit: None,
                            w_type: None,
                        },
                    ],
                },
                DataGroup {
                    title: "Player".to_string(),
                    widget_type: None,
                    datasets: vec![
                        DataSet {
                            title: Some("Coins".to_string()),
                            value: json!(self.data.player_coins),
                            graph: Some(false),
                            unit: Some("coins".to_string()),
                            w_type: None,
                        },
                        DataSet {
                            title: Some("Boost".to_string()),
                            value: json!(self.data.player_boost_percent),
                            graph: Some(false),
                            unit: Some("%".to_string()),
                            w_type: None,
                        },
                        DataSet {
                            title: Some("Breath".to_string()),
                            value: json!(self.data.player_breath_percent),
                            graph: Some(false),
                            unit: Some("%".to_string()),
                            w_type: None,
                        },
                    ],
                },
            ],
        };

        // Send the frame
        self.server.as_mut().unwrap().publish(frame);
    }
}

/// Release mode: We do nothing here
#[cfg(not(debug_assertions))]
impl GameProfiler {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn start(&mut self) {}
    pub fn stop(&mut self) {}
    pub fn update(&mut self) {}
}
