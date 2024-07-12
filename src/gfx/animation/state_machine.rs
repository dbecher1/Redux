
use serde::{Deserialize, Serialize};
use ahash::AHashMap;
use crate::gfx::animation::Animation;

use crate::gfx::animation::from_atlas::AtlasLoader;

#[derive(Deserialize, Serialize, Debug)]
pub struct AnimationStateMachine {
    pub(crate) animations: AHashMap<String, Animation>,
    pub(crate) current_state: String
    // maybe name?
}

impl Default for AnimationStateMachine {
    fn default() -> Self {
        Self {
            animations: AHashMap::new(),
            current_state: String::new(),
        }
    }
}

#[allow(dead_code)]
impl AnimationStateMachine {

    // TODO!! Make this consistent for loading things...

    pub fn new() -> Self {
        Default::default()
    }

    pub async fn new_with_atlas(path: &str) -> Result<Self, String> {
        AtlasLoader::new_from_file(path).await
    }

    pub fn new_from_file(path: &str) -> Result<Self, String> {
        return if let Ok(file) = std::fs::read(path) {
            if let Ok(f_) = String::from_utf8(file) {
                if let Ok(astate) = toml::from_str::<AnimationStateMachine>(f_.as_str()) {
                    Ok(astate)
                } else {
                    Err(String::from("Error parsing TOML file!"))
                }
            } else {
                Err(String::from("Data read is not a valid UTF-8 string!"))
            }
        } else {
            Err(String::from("Error loading file!"))
        };
    }

    pub fn add_animation(&mut self, name: &str, animation: Animation) -> Option<()> {
        return if !self.animations.contains_key(name) {
            self.animations.insert(String::from(name), animation);
            if self.current_state.is_empty() {
                self.current_state = String::from(name);
            }
            Some(())
        } else {
            None
        }
    }

    pub fn update(&mut self, dt: f32) -> Result<(), String> {
        return if let Some(curr_anim) = self.animations.get_mut(&self.current_state) {
            curr_anim.update(dt);
            Ok(())
        } else {            
            Err(String::from("No state currently set for this animation!"))
        }
    }

    pub fn set_state(&mut self, state: &String) -> Result<(), String> {
        return if self.animations.contains_key(state) {
            self.current_state = state.to_owned();
            Ok(())
        } else {
            Err(String::from("Attempted to set a state that doesn't exist!"))
        }
    }

    pub async fn draw(&mut self, x: f32, y: f32) -> Result<(), String> {
        return if let Some(curr_anim) = self.animations.get_mut(&self.current_state) {
            curr_anim.draw(x, y).await;
            Ok(())
        } else {
            Err(String::from(""))
        }
    }
}
