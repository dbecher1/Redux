
use macroquad::{color::WHITE, math::{Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};
use builder::AnimationBuilder;
use crate::entity::Updateable;

use super::{DrawCommand, SpriteBatch};

pub(crate) mod loader;
mod builder;
pub(crate) mod state_machine;

pub(crate) static ANIMATION_PATH: &str = "resources/animations/";

// This assumes comic book style
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct Animation {
    pub(crate) texture: Option<Texture2D>,
    pub(crate) num_frames_x: u32,
    pub(crate) num_frames_y: u32, // will always be 1 right now

    
    pub(crate) animation_speed: f32,
    pub(crate) x_offset: u32,
    pub(crate) y_offset: u32,
    pub(crate) size: (u32, u32),
    pub(crate) flip: bool,
    pub(crate) scale: (f32, f32),

    accumulator: f32,
    current_frame: u32,
    is_playing: bool,
    default_frame: u32,
}

#[allow(dead_code)]
impl Animation {

    pub fn new_from_builder(builder: AnimationBuilder) -> Self {
        let texture = builder.texture;
        let num_frames_x = builder.num_frames_x;
        let num_frames_y = builder.num_frames_y;
        let animation_speed = builder.animation_speed / 1000.;
        let x_offset = builder.x_offset;
        let y_offset = builder.y_offset;
        let size = builder.size;
        let flip = builder.flip;
        let accumulator = 0.;
        let current_frame = 0;
        let scale = builder.scale;
        let is_playing = true;
        let default_frame = builder.default_frame;

        Self {
            texture,
            num_frames_x,
            num_frames_y,
            animation_speed,
            x_offset,
            y_offset,
            size,
            flip,
            accumulator,
            current_frame,
            scale,
            is_playing,
            default_frame,
        }
    }

    fn increment_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.num_frames_x;
    }

    pub fn reset(&mut self) {
        self.accumulator = 0.;
        self.current_frame = self.default_frame;
    }

    pub fn stop(&mut self) {
        self.reset(); // TODO see if I want this
        self.is_playing = false;
    }

    pub fn start(&mut self) {
        self.is_playing = true;
        self.increment_frame();   
    }

    pub fn draw(&self, x: f32, y: f32, texture: &Option<Texture2D>, spritebatch: Option<&mut SpriteBatch>) {
        let tex = match texture {
            Some(t) => t,
            None => match &self.texture {
                Some(t) => t,
                None => return,
            }
        };

        let source = Rect::new(
            ((self.current_frame * self.size.0) + (self.x_offset * self.size.0)) as f32,
            (self.y_offset * self.size.1) as f32,
            self.size.0 as f32,
            self.size.1 as f32,
        );

        let dest_size = Vec2::new(
            self.scale.0 * (self.size.0 as f32),
            self.scale.1 * (self.size.1 as f32),
        );

        let params = DrawTextureParams{
            source: Some(source),
            dest_size: Some(dest_size),
            flip_x: self.flip,
            ..Default::default()
        };

        match spritebatch {
            Some(sb) => {
                let z = Some(sb.sort_layer());
                let dc = DrawCommand {
                    texture: Some(tex.weak_clone()),
                    x,
                    y,
                    z,
                    params,
                    ..Default::default()
                };
                sb.add(dc);
            },
            None => {}, //draw_texture_ex(tex, x, y, WHITE, params),
        }
        // 
    }
}

impl Updateable for Animation {
    fn update(&mut self, dt: f32) {
        if !self.is_playing {
            return;
        }
        self.accumulator += dt;
        if self.accumulator >= self.animation_speed {
            self.accumulator = 0.;
            self.increment_frame();
        }
    }
}