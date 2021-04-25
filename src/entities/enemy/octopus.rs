use crate::{
    lib::utils::calculate_linear_slide,
    pallette::{TRANSLUCENT_RED_64, TRANSLUCENT_WHITE_64},
    player::Player,
};

use super::base::EnemyBase;
use rand::{prelude::ThreadRng, Rng};
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

const OCTOPUS_SUCK_AIR_DELAY: f64 = 3.5;
const OCTOPUS_SUCK_AIR_RANGE: f32 = 70.0;
const OCTOPUS_SUCK_AIR_DURATION: f64 = 1.0;
// const RNG: ThreadRng = rand::thread_rng();

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct OctopusAirBubble {
    position: Vector2,
    speed: f32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Octopus {
    pub position_a: Vector2,
    pub position_b: Vector2,

    #[serde(skip)]
    pub current_position: Vector2,

    #[serde(skip)]
    pub stunned_timer: f64,
    #[serde(skip)]
    pub max_stunned_time: f64,

    #[serde(skip)]
    pub suck_air_time_remaining: f64,
    #[serde(skip)]
    suck_air_bubbles: Vec<OctopusAirBubble>,
}

impl Octopus {}

impl EnemyBase for Octopus {
    fn render(
        &mut self,
        context_2d: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
        player: &mut Player,
        resources: &mut crate::resources::GlobalResources,
        dt: f64,
    ) {
        let is_octopus_stunned = self.stunned_timer > 0.0;

        // Simple sine position
        let h_trans = (context_2d.get_time() / 8.0).sin().abs() as f32;

        // Modify the current pose
        let dist_a_to_b = self.position_b - self.position_a;
        self.current_position = (dist_a_to_b * h_trans) + self.position_a;

        // Render the stun ring
        if self.max_stunned_time > 0.0 && self.stunned_timer > 0.0 {
            let stun_ring_alpha =
                calculate_linear_slide(self.stunned_timer / self.max_stunned_time);
            context_2d.draw_circle_v(
                self.current_position,
                20.0,
                TRANSLUCENT_RED_64.fade(0.55 * stun_ring_alpha as f32),
            );
            self.stunned_timer -= dt;
        }

        // Every once in a while, start sucking air
        if (context_2d.get_time() % OCTOPUS_SUCK_AIR_DELAY) < 0.1 {
            self.suck_air_time_remaining = OCTOPUS_SUCK_AIR_DURATION;

            // Spawn a few air bubbles if the player is in range
            if player.position.distance_to(self.current_position).abs() <= OCTOPUS_SUCK_AIR_RANGE {
                for _ in 0..3 {
                    self.suck_air_bubbles.push(OctopusAirBubble {
                        position: player.position,
                        speed: rand::thread_rng().gen_range(0.8..1.3),
                    });
                }
            }
        }

        // Handle sucking air bubble animation
        if self.suck_air_time_remaining > 0.0 {
            // Render and update all bubbles
            for bubble in self.suck_air_bubbles.iter_mut() {
                // Get the direction from the bubble to the octopus
                let direction = (self.current_position - bubble.position).normalized();

                // Render the bubble
                context_2d.draw_circle_v(bubble.position, 2.0, TRANSLUCENT_WHITE_64);

                // Move the bubble
                bubble.position += direction * bubble.speed;
            }

            // Reduce time
            self.suck_air_time_remaining -= dt;
        } else {
            self.suck_air_bubbles.clear();
        }

        // TODO: TMP
        context_2d.draw_circle_v(self.current_position, 10.0, Color::RED);
    }

    fn handle_logic(&mut self, player: &mut crate::player::Player, dt: f64) {
        if self.suck_air_time_remaining > 0.0 {}
    }

    fn handle_getting_attacked(&mut self, stun_duration: f64, current_time: f64) {
        self.stunned_timer = stun_duration;
        self.max_stunned_time = stun_duration;
    }
}
