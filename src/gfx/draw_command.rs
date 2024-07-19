use std::rc::Rc;

use macroquad::texture::{DrawTextureParams, Texture2D};

#[derive(Debug)]
pub struct DrawCommand {
    pub texture: Option<Texture2D>,
    pub texture_experiment: Option<Rc<Texture2D>>,
    pub x: f32,
    pub y: f32,
    pub z: Option<usize>,
    pub params: DrawTextureParams,
}

impl Default for DrawCommand {
    fn default() -> Self {
        Self {
            texture: None,
            texture_experiment: None,
            x: 0.,
            y: 0.,
            z: None,
            params: DrawTextureParams::default(),
        }
    }
}
