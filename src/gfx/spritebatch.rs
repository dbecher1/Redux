use macroquad::{color::WHITE, texture::draw_texture_ex};
use std::cmp::Ordering;
use super::DrawCommand;

// PLACEHOLDER
pub struct SpriteBatch {
    queue: Vec<DrawCommand>,
}

#[allow(dead_code)]
impl SpriteBatch {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }
    pub fn add(&mut self, dc: DrawCommand) {
        self.queue.push(dc);
    }

    pub fn draw(&mut self) {

        self.queue.sort_by(|l, r| l.y.partial_cmp(&r.y).unwrap_or_else(|| Ordering::Equal));

        for dc in self.queue.drain(0..) {
            draw_texture_ex(
                &dc.texture,
                dc.x,
                dc.y,
                WHITE,
                dc.params
            )
        }
    }
}

