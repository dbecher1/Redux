use macroquad::{input::{is_key_down, KeyCode}, math::Vec2};

pub struct InputManager;

impl InputManager {

    pub fn get_input() -> Vec2 {

        let mut delta = Vec2::ZERO;

        if InputManager::input_left() {
            delta.x -= 1.0;
        }
        if InputManager::input_right() {
            delta.x += 1.0;
        }
        if InputManager::input_up() {
            delta.y -= 1.0;
        }
        if InputManager::input_down() {
            delta.y += 1.0;
        }

        delta
    }

    fn input_left() -> bool {
        is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)
    }

    fn input_right() -> bool {
        is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)
    }

    fn input_up() -> bool {
        is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)
    }

    fn input_down() -> bool {
        is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)
    }
}