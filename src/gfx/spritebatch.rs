use macroquad::{color::WHITE, texture::draw_texture_ex};
use std::cmp::Ordering;
use super::DrawCommand;

static MANUAL_OFFSET: f32 = 13.;

// PLACEHOLDER
pub struct SpriteBatch {
    queue: Vec<DrawCommand>,
}

fn y_sort_compare(y1: f32, y2: f32) -> Ordering {
    let y1_ = y1 + MANUAL_OFFSET;
    y1_.partial_cmp(&y2).unwrap_or_else(|| Ordering::Equal)
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

        //self.queue.sort_by(|l, r| l.y.partial_cmp(&r.y).unwrap_or_else(|| Ordering::Equal));
        self.queue.sort_by(|l, r| y_sort_compare(l.y, r.y));

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

