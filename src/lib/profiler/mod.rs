use serde_json::json;
use serialstudio::{
    data::{DataGroup, DataSet, TelemetryFrame},
    SerialStudioSource,
};

/// The development profiler
#[derive(Default)]
pub struct GameProfiler {
    /// The SerialStudio server
    server: Option<SerialStudioSource>,

    /// The data
    pub frames_per_second: u32,
    pub seconds_per_frame: f32,
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
            groups: vec![DataGroup {
                title: "Rendering engine".to_string(),
                widget_type: None,
                datasets: vec![
                    DataSet {
                        title: Some("Frames per Second".to_string()),
                        value: json!(self.frames_per_second),
                        graph: Some(true),
                        unit: Some("fps".to_string()),
                        w_type: None,
                    },
                    DataSet {
                        title: Some("Seconds per Frame".to_string()),
                        value: json!(self.seconds_per_frame),
                        graph: Some(true),
                        unit: Some("seconds".to_string()),
                        w_type: None,
                    },
                ],
            }],
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
