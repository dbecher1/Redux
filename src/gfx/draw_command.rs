use macroquad::texture::{DrawTextureParams, Texture2D};


pub struct DrawCommand {
    pub texture: Texture2D,
    pub x: f32,
    pub y: f32,
    pub params: DrawTextureParams
}
