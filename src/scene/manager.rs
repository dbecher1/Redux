
use macroquad::logging::*;
use super::Scene;

// General game state manager
pub struct SceneManager {
    scene_stack: Vec<Scene>,

    timer_accumulator: f32,
    timer: u64,
}

impl SceneManager {

    // Assuming dt is milliseconds
    pub fn update(&mut self, dt: f32) {
        self.timer_accumulator += dt;
        while self.timer_accumulator >= 1. {
            self.timer += 1;
            self.timer_accumulator -= 1.;
        }

        // Only update the top of the stack
        match self.scene_stack.last_mut() {
            Some(last) => {
                last.update(dt);
            },
            None => {
                error!("Scene stack is empty! Something went wrong");
            }
        }
    }
}