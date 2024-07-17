use macroquad::{color::{Color, WHITE}, math::Vec2, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use crate::gfx::{DrawCommand, SpriteBatch};
use super::MapLayerDrawOptions;

use super::{layer::MapLayer, tileset::TileSet};


#[allow(dead_code)]
#[derive(Debug)]
pub struct TileMap {
    pub(super) texture: Texture2D,
    pub(super) tileset: TileSet,
    pub(super) layers: Vec<MapLayer>,
    pub(super) height: usize,
    pub(super) width: usize,
    pub(super) tileheight: usize,
    pub(super) tilewidth: usize,
    pub(super) draw_scale: f32,
    pub(super) player_layer: usize, // TODO: delete?
}

#[allow(dead_code)]
impl TileMap {

    pub fn texture(&self) -> &Texture2D {
        &self.texture
    }

    pub fn player_layer(&self) -> usize {
        self.player_layer
    }

    pub fn set_player_layer(&mut self, layer: usize) {
        self.player_layer = layer;
    }

    pub fn set_draw_scale(&mut self, scale: f32) {
        self.draw_scale = scale;
    }

    pub fn set_layer_draw_type_by_idx(&mut self, idx: &[usize], draw_type: MapLayerDrawOptions) {
        for lyr in self.layers.iter_mut().enumerate().filter(|(i, _)| idx.contains(i)).map(|(_, lyr)| lyr) {
            lyr.set_draw_type(draw_type);
        }
    }

    fn idx_to_x_y(&self, idx: usize) -> (f32, f32) {
        let x = ((idx % self.width) * self.tilewidth) as f32 * self.draw_scale;
        let y = ((idx / self.width) * self.tileheight) as f32 * self.draw_scale;
        (x, y)
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch) {
        self.draw_layers_inner(&[], Some(spritebatch));
    }

    fn draw_helper(&self, tid: u32, x: f32, y: f32, z: usize, spritebatch: &mut Option<&mut SpriteBatch>) {
        let rect = self.tileset.to_rect(tid);
        let size = Vec2::new(self.tilewidth as f32 * self.draw_scale, self.tileheight as f32 * self.draw_scale);
        let params = DrawTextureParams {
            source: Some(rect),
            dest_size: Some(size),
            ..Default::default()
        };

        match spritebatch {
            Some(sb) => {
                let dc = DrawCommand {
                    texture: self.texture.weak_clone(),
                    x,
                    y,
                    z: Some(z),
                    params,
                    ..Default::default()
                };
                sb.add(dc);
            },
            None => {
                draw_texture_ex(
                    &self.texture,
                    x,
                    y,
                    WHITE,
                    params
                    );
            }
        }
    }

    pub fn draw_by_type(&self, draw_type: MapLayerDrawOptions, spritebatch: Option<&mut SpriteBatch>) {
        let idx = self.layers
            .iter()
            .enumerate()
            .filter(|(_, lyr)| lyr.draw_type() == draw_type)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        //println!("{:?}", idx);

        // In case this is called incorrectly we'll defer the error handling to the inner function that already deals with it
        match draw_type {
            MapLayerDrawOptions::PlayerSorted => self.draw_layers_inner(&idx, spritebatch), 
            _ => self.draw_layers_inner(&idx, None),
        }
    }

    pub fn draw_all_immediate(&self) {
        let v: Vec<usize> = (0..self.layers.len()).into_iter().collect();
        self.draw_layers_inner(&v, None);
    }

    pub fn draw_below_player(&self) {
        if self.player_layer == 0 {
            self.draw_layers_inner(&[0], None);
        }
        else {
            let v: Vec<usize> = (0..self.player_layer - 1).into_iter().collect();
            self.draw_layers_inner(&v, None);
        }
    }

    pub fn draw_layers(&self, layers: &[usize]) {
        self.draw_layers_inner(layers, None);
    }

    pub fn draw_layers_deferred(&self, layers: &[usize], spritebatch: &mut SpriteBatch) {
        self.draw_layers_inner(layers, Some(spritebatch));
    }

    #[allow(unused_variables)]
    fn draw_layers_inner(&self, layers: &[usize], mut spritebatch: Option<&mut SpriteBatch>) {

        let draw_all = layers.len() == 0;
        
        for (n, layer) in self.layers.iter().enumerate() {

            if !draw_all && !layers.contains(&n) {
                continue;
            }

            if let Some(data) = layer.data() {
                for (i, tile) in data.iter().enumerate().filter(|(_, t)| **t != 0) {
                    let (x, y) = self.idx_to_x_y(i);

                    self.draw_helper(*tile, x, y, layer.z(), &mut spritebatch);
                }
            }
            else {
                // chunkzz
                // check option here too in case something went wrong... it's zero cost so no reason not to
                if let Some(chunks) = layer.chunks() {
                    // let w = chunks.first().unwrap().width;
                    let w = layer.chunk_width().unwrap();
                    let num_chunks_x = layer.width() as i32 / w;

                    for (chunk, i) in chunks.iter().zip((0 as i32)..) {
                        for (yidx, y) in (chunk.y()..chunk.y() + chunk.height()).enumerate() {
                            for (xidx, x) in (chunk.x()..chunk.x() + chunk.width()).enumerate() {
                                
                                let tile_id = chunk.data()[((yidx * chunk.width() as usize) + xidx) as usize];

                                if tile_id == 0 {
                                    continue;
                                }

                                let real_x = (((i % num_chunks_x) * chunk.width() * self.tilewidth as i32) + (x * self.tilewidth as i32)) as f32 * self.draw_scale;
                                let real_y = (((i / num_chunks_x) * chunk.height() * self.tileheight as i32) + (y * self.tileheight as i32)) as f32 * self.draw_scale;
                                
                                self.draw_helper(tile_id, real_x, real_y, layer.z(), &mut spritebatch);
                            }
                        }
                    }
                }
            }
        }
    }
}
