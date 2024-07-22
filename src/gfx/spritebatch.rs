use macroquad::color::RED;
use macroquad::math::{Rect, Vec2};
use macroquad::shapes::draw_rectangle_lines;
use macroquad::window::{screen_height, screen_width};
use macroquad::{color::WHITE, texture::draw_texture_ex};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use super::DrawCommand;

static MANUAL_OFFSET: f32 = 13.;
static DEFAULT_SORT_LAYER: usize = 2;

// PLACEHOLDER
pub struct SpriteBatch {
    camera: Rect,
    sorted_queue: BTreeMap<usize, Vec<DrawCommand>>,
    sort_layer: usize,
}

fn y_sort_compare(y1: f32, y2: f32) -> Ordering {
    let y1_ = y1 + MANUAL_OFFSET;
    y1_.partial_cmp(&y2).unwrap_or_else(|| Ordering::Equal)
}

#[allow(dead_code)]
impl SpriteBatch {

    pub fn sort_layer(&self) -> usize {
        self.sort_layer
    }

    pub fn new() -> Self {

        let camera = Rect::new(0., 0., screen_width(), screen_height());

        Self {
            camera,
            sorted_queue: BTreeMap::new(),
            sort_layer: DEFAULT_SORT_LAYER,
        }
    }

    pub fn update(&mut self, position: Vec2) {
        // TODO!! size offset
        let (half_w, half_h) = (0., 0.);
        self.camera.x = position.x - half_w;
        self.camera.y = position.y - half_h;
    }

    pub fn add(&mut self, dc: DrawCommand) {

        match dc.z {
            Some(z) => {
                match self.sorted_queue.get_mut(&z) {
                    Some(queue) => queue.push(dc),
                    None => {
                        let mut v = Vec::new();
                        v.push(dc);
                        self.sorted_queue.insert(z, v);
                    }
                }
            },
            None => {
                let mut d = dc;
                d.z = Some(self.sort_layer);
                self.add(d);
            },
        }
    }

    fn half_size(&self) -> (f32, f32) {
        let half_w = self.camera.w * 0.5;
        let half_h = self.camera.h * 0.5;
        (half_w, half_h)
    }

    pub fn draw(&mut self) {

        let half_size = self.half_size();

        let x_offset = if self.camera.x - half_size.0 < 0. {
            // 0. // SHOULD BE THIS
            self.camera.x - half_size.0
        } else {
            self.camera.x - half_size.0
        };

        let y_offset = if self.camera.y - half_size.1 < 0. {
            // 0. // SHOULD BE THIS
            self.camera.y - half_size.1
        } else {
            self.camera.y - half_size.1
        };

        for (k, v) in self.sorted_queue.iter_mut() {
            if k == &self.sort_layer {
                v.sort_by(|l, r| y_sort_compare(l.y, r.y));
            }
            for dc in v.drain(0..) {

                // floor to prevent sub-pixel tearing
                let x = (dc.x - x_offset).floor();
                let y = (dc.y - y_offset).floor();

                match &dc.texture {
                    Some(texture) => {
                        draw_texture_ex(
                            &texture,
                            x,
                            y,
                            WHITE,
                            dc.params
                        );
                    },
                    None => {
                        let size = dc.params.dest_size.unwrap_or_default();
                        draw_rectangle_lines(x, y, size.x, size.y, 1., RED);
                    },
                }

                
            }
        }
    }
}

