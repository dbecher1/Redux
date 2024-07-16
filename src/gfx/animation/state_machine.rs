
use ahash::AHashMap;
use macroquad::texture::{load_texture, FilterMode, Texture2D};
use crate::{entity::Updateable, gfx::{spritebatch, SpriteBatch}};

use super::{builder::AnimationBuilder, loader::*, Animation};

#[derive(Debug)]
pub struct AnimationStateMachine {
    current_state: String,
    texture: Option<Texture2D>, // for atlas state machines
    animations: AHashMap<String, Animation>,
    is_playing: bool, // set kinda uniquely
}

#[allow(dead_code)]
impl AnimationStateMachine {

    pub async fn new_from_loader(loader: StateMachineLoader) -> Self {
        let current_state = loader.initial_state;

        let sm_texture = match loader.texture_name {
            Some(name) => match load_texture(&name).await {
                Ok(tex) => {
                    tex.set_filter(FilterMode::Nearest);
                    Some(tex)
                },
                Err(..) => None,
            },
            None => None,
        };

        let mut animations: AHashMap<String, Animation> = AHashMap::new();

        for anim_loader in loader.animations {
            let name = match anim_loader.name {
                Some(n) => n,
                None => {
                    println!("Error loading animation: name not provided");
                    continue;
                },
            };

            let texture = match sm_texture {
                Some(..) => None,
                None => {
                    match anim_loader.texture_name {
                        Some(name) => match load_texture(&name).await {
                            Ok(tex) => {
                                tex.set_filter(FilterMode::Nearest);
                                Some(tex)
                            },
                            Err(..) => {
                                println!("Error loading animation: couldn't load texture");
                                continue;
                            }
                        },
                        None => {
                            println!("Error loading animation: no texture data provided");
                            continue;
                        }
                    }
                }
            };

            let (num_frames_x, num_frames_y, animation_speed) = (
                anim_loader.num_frames_x,
                anim_loader.num_frames_y,
                anim_loader.animation_speed
            );

            let x_offset = match anim_loader.x_offset {
                Some(xoff) => xoff,
                None => 0
            };

            let y_offset = match anim_loader.y_offset {
                Some(yoff) => yoff,
                None => 0
            };

            let size = match anim_loader.size {
                Some(size_) => size_,
                None => match loader.size {
                    Some(size_) => size_,
                    None => {
                        println!("Error loading animation: no size data provided.");
                        continue;
                    }
                }
            };

            let flip = match anim_loader.flip {
                Some(flip_) => flip_,
                None => false
            };

            let scale = match anim_loader.scale {
                Some(scale_) => scale_,
                None => match loader.scale {
                    Some(scale_) => scale_,
                    None => (1., 1.),
                }
            };

            let default_frame = match anim_loader.default_frame {
                Some(df) => df,
                None => 0,
            };

            let builder = AnimationBuilder {
                texture,
                num_frames_x,
                num_frames_y,
                animation_speed,
                x_offset,
                y_offset,
                size,
                flip,
                scale,
                default_frame,
            };

            let animation = Animation::new_from_builder(builder);

            let _ = animations.insert(name, animation);
        }

        let is_playing = true;

        Self {
            current_state,
            texture: sm_texture,
            animations,
            is_playing,
        }
    }

    pub fn draw(&self, x: f32, y: f32, spritebatch: Option<&mut SpriteBatch>) {
        match self.animations.get(&self.current_state) {
            Some(state) => state.draw(x, y, &self.texture, spritebatch),
            None => {}
        }
    }

    pub fn set_state(&mut self, state: &str) {
        if !self.current_state.eq_ignore_ascii_case(state) {
            match self.animations.get_mut(&self.current_state) {
                Some(curr) => curr.reset(),
                None => {},
            }
            if self.animations.contains_key(state) {
                self.current_state = String::from(state);
            }
        }
    }

    pub fn stop(&mut self) {
        match self.animations.get_mut(&self.current_state) {
            Some(curr) => curr.stop(),
            None => {},
        }
        self.is_playing = false;
    }

    pub fn start(&mut self) {
        match self.animations.get_mut(&self.current_state) {
            Some(curr) => curr.start(),
            None => {},
        }
        self.is_playing = true;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}

impl Updateable for AnimationStateMachine {
    fn update(&mut self, dt: f32) {
        match self.animations.get_mut(&self.current_state) {
            Some(state) => state.update(dt),
            None => {}
        }
    }
}
