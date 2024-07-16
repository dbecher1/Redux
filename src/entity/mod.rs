use ahash::AHashMap;
use traits::Moveable;
use macroquad::math::Vec2;
use crate::gfx::{load_animations, AnimationStateMachine, SpriteBatch};
use loader::EntityLoader;
use std::fs::File;

mod player;
mod traits;
mod loader;

pub use traits::Updateable;
pub use player::Player;

static ENTITY_PATH: &str = "resources/entity/";

pub struct Entity {
    pub(crate) animations: AnimationStateMachine,
    pub(crate) position: Vec2,
    pub(crate) delta: Vec2,
    pub(crate) move_speed: f32,

    #[allow(unused)]
    states: AHashMap<u32, String>,

    current_state: u32,
}

impl Entity {

    // TODO: load from file

    // Path is expected to be a json file in resources/entity/
    pub async fn new(path: &str) -> Result<Self, String> {

        let mut full_path = String::from(ENTITY_PATH);
        full_path.push_str(path);
        full_path.push_str(".json");

        let file = match File::open(full_path) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        let loader: EntityLoader = match serde_json::from_reader(file) {
            Ok(l) => l,
            Err(e) => return Err(e.to_string()),
        };

        let animations = match load_animations(&loader.animation_data).await {
            Ok(anim) => anim,
            Err(..) => return Err(String::from("Error loading animation data for entity")),
        };

        let position = match loader.default_position {
            Some(pos) => Vec2::new(pos.0, pos.1),
            None => Vec2::new(100., 100.),
        };

        let move_speed = match loader.move_speed {
            Some(ms) => ms,
            None => 100.,
        };
        
        let delta = Vec2::ZERO;

        let current_state = 0;
        
        // TODO: load from file? idk...
        let mut states = AHashMap::new();
        states.insert(0, String::from("Left"));
        states.insert(1, String::from("Right"));
        states.insert(2, String::from("Down"));
        states.insert(3, String::from("Up"));

        Ok(Self {
            animations,
            position,
            delta,
            move_speed,
            states,
            current_state,
        })
    }

    pub fn draw(&self, spritebatch: Option<&mut SpriteBatch>) {
        self.animations.draw(self.position.x, self.position.y, spritebatch);
    }
}

impl Moveable for Entity {
    fn position(&self) -> Vec2 {
        self.position
    }

    fn move_speed(&self) -> f32 {
        self.move_speed
    }

    fn move_entity(&mut self, dt: f32) -> () {

        let len = self.delta.length();

        if len > 1. {
            self.delta = self.delta.normalize_or_zero();
        }

        self.position += self.delta * self.move_speed * dt;

        if len > 0. && !self.animations.is_playing() {
            self.animations.start();
        }
        else if len == 0. && self.animations.is_playing() {
            self.animations.stop();
        }
    }
    
    fn delta(&self) -> Vec2 {
        self.delta
    }
    
    fn is_moving(&self) -> bool {
        self.delta.length() == 0.
    }
}

impl Updateable for Entity {
    fn update(&mut self, dt: f32) {
        self.animations.update(dt);

        // animation states for entities are expected to be generalized
        // 0 - left
        // 1 - right
        // 2 - down
        // 3 - up

        let mut state = self.current_state;

        if self.delta.x < 0. {
            state = 0;
        }
        else if self.delta.x > 0. {
            state = 1;
        }

        if self.delta.y < 0. {
            state = 3;
        }
        else if self.delta.y > 0. {
            state = 2;
        }

        if state != self.current_state {
            let new_state = self.states.get(&state).unwrap();
            self.animations.set_state(new_state);
            self.current_state = state;
        }
    }
}