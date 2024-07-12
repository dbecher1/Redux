
use std::fs::File;
use serde_json::Result as SerdeJsonResult;

use macroquad::texture::load_texture;
use serde::Deserialize;

use crate::gfx::AnimationStateMachine;

use super::{Animation, AnimationBuilder};

#[derive(Deserialize)]
#[allow(dead_code)]
struct AtlasLoaderSprite {
    name: String,
    num_frames_x: u32,
    num_frames_y: u32,
    animation_speed: f32,
    x_offset: u32,
    flip: Option<bool>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct AtlasLoader {
    initial_state: String,
    texture_path: String,
    size: (u32, u32),
    animations: Vec<AtlasLoaderSprite>,
}

impl AtlasLoader {
    pub async fn new_from_file(path: &str) -> Result<AnimationStateMachine, String> {
        let file = File::open(path);

        let file = match file {
            Ok(file) => file,
            Err(_) => return Err(String::from("Error loading file!")),
        };

        let try_parse: SerdeJsonResult<AtlasLoader> = serde_json::from_reader(file);

        let loaded_atlas = match try_parse {
            Ok(p) => p,
            Err(_) => return Err(String::from("Error parsing JSON file for sprite atlas!")),
        };

        let mut asm = AnimationStateMachine::new();

        let texture = match load_texture(&loaded_atlas.texture_path).await {
            Ok(tex) => tex,
            Err(..) => return Err("AHH".to_string())
        };

        for anim in loaded_atlas.animations {
            let ab = AnimationBuilder::new_with_texture(texture.weak_clone())
                .with_frames_x(anim.num_frames_x)
                .with_frames_y(anim.num_frames_y)
                .with_dimensions(loaded_atlas.size.0, loaded_atlas.size.1)
                .with_animation_speed(anim.animation_speed)
                .with_x_offset(anim.x_offset)
                .with_flip(anim.flip);

            asm.add_animation(&anim.name, Animation::new(&ab).await.unwrap());
            
        }

        match asm.set_state(&loaded_atlas.initial_state) {
            Ok(_) => {},
            Err(_) => return Err(String::from("Error - given initial state does not exist!")),
        }

        Ok(asm)
    }

    
}