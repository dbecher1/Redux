use macroquad::{
    color::{
        RED,
        WHITE,
    },
    math::{
        Rect,
        Vec2,
    },
    shapes::draw_rectangle_lines,
    texture::{
        DrawTextureParams,
        draw_texture_ex,
    },
    window::{
        screen_height,
        screen_width,
    },
};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
};
use super::DrawCommand;
use ahash::AHashSet;

static MANUAL_OFFSET: f32 = 13.;
static DEFAULT_SORT_LAYER: usize = 2;

pub struct SpriteBatch {
    camera: Rect,
    sorted_queue: BTreeMap<usize, Vec<DrawCommand>>,

    // Changing this from just one set layer to be able to set any number of layers
    sort_layers: AHashSet<usize>,
    sort_layer: Option<usize>, // Keeping a field of this here though so old code doesn't break

    draw_scale: Option<f32>,
}

fn y_sort_compare(y1: f32, y2: f32) -> Ordering {
    let y1_ = y1 + MANUAL_OFFSET;
    y1_.partial_cmp(&y2).unwrap_or_else(|| Ordering::Equal)
}

#[allow(dead_code)]
impl SpriteBatch {


    pub fn sort_layer(&self) -> usize {
        self.sort_layer.unwrap_or(DEFAULT_SORT_LAYER)
    }

    pub fn add_sort_layer(&mut self, layer: usize) {
        self.sort_layers.insert(layer);
        if self.sort_layer.is_none() {
            self.sort_layer = Some(layer);
        }
    }

    pub fn remove_sort_layer(&mut self, layer: usize) {
        self.sort_layers.remove(&layer);
        if let Some(l) = self.sort_layer {
            if l == layer {
                self.sort_layer = None;
            }
        }
    }

    pub fn new() -> Self {

        let camera = Rect::new(0., 0., screen_width(), screen_height());
        let draw_scale = None;
        let sorted_queue = BTreeMap::new();
        let mut sort_layers = AHashSet::new();
        sort_layers.insert(DEFAULT_SORT_LAYER);

        let sort_layer = Some(DEFAULT_SORT_LAYER);

        Self {
            camera,
            sorted_queue,
            sort_layers,
            sort_layer,
            draw_scale,
        }
    }

    pub fn set_draw_scale(&mut self, scale: f32) {
        self.draw_scale = match scale {
            1. => None,
            _ => Some(scale),
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
                d.z = Some(self.sort_layer.unwrap_or(DEFAULT_SORT_LAYER));
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
            if self.sort_layers.contains(k) {
                v.sort_by(|l, r| y_sort_compare(l.y, r.y));
            }
            for dc in v.drain(0..) {

                // floor to prevent sub-pixel tearing
                let x = (dc.x - x_offset).floor();
                let y = (dc.y - y_offset).floor();

                let params = if let Some(scale) = self.draw_scale {
                    match dc.params.dest_size {
                        Some(size) => DrawTextureParams {
                            dest_size: Some(size * scale),
                            ..dc.params
                        },
                        None => dc.params
                    }
                } else {
                    dc.params
                };

                match &dc.texture {
                    Some(texture) => {
                        draw_texture_ex(
                            &texture,
                            x,
                            y,
                            WHITE,
                            params
                        );
                    },
                    None => {
                        //let size = dc.params.dest_size.unwrap_or_default();
                        // THIS DOESN'T WORK
                        let size = Vec2::ZERO;
                        draw_rectangle_lines(x, y, size.x, size.y, 1., RED);
                    },
                }

                
            }
        }
    }
}

