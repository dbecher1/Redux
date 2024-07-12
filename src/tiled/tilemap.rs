use macroquad::{color::Color, math::Vec2, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use crate::gfx::{DrawCommand, SpriteBatch};

use super::{layer::RawMapLayerData, tileset::TileSet};


#[allow(dead_code)]
#[derive(Debug)]
pub struct TileMap {
    pub(super) texture: Texture2D,
    pub(super) tileset: TileSet,
    pub(super) layers: Vec<RawMapLayerData>,
    pub(super) height: usize,
    pub(super) width: usize,
    pub(super) tileheight: usize,
    pub(super) tilewidth: usize,
    pub(super) draw_scale: f32,
    pub(super) player_layer: usize,
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

    fn idx_to_x_y(&self, idx: usize) -> (f32, f32) {
        let x = ((idx % self.width) * self.tilewidth) as f32 * self.draw_scale;
        let y = ((idx / self.width) * self.tileheight) as f32 * self.draw_scale;
        (x, y)
    }

    fn draw_helper_immediate(&self, tid: u32, x: f32, y: f32) {
        let rect = self.tileset.to_rect(tid);
        let size = Vec2::new(self.tilewidth as f32 * self.draw_scale, self.tileheight as f32 * self.draw_scale);

        draw_texture_ex(
            &self.texture,
            x,
            y,
            Color::from_hex(0xFFFFFF),
            DrawTextureParams {
                source: Some(rect),
                dest_size: Some(size),
                ..Default::default()
        });
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

                    if let Some(ref mut sb) = spritebatch {
                        // TODO - this is where we'd call a function to defer drawing to a sprite batch for y-sorting
                        let dc = DrawCommand {
                            texture: self.texture.weak_clone(),
                            x,
                            y,
                            params: DrawTextureParams::default(),
                        };

                        sb.add(dc);
                    }
                    else {
                        self.draw_helper_immediate(*tile, x, y);
                    }
                }
            }
            else {
                // chunkzz
                // check option here too in case something went wrong... it's zero cost so no reason not to
                if let Some(chunks) = layer.chunks() {
                    // let w = chunks.first().unwrap().width;
                    let w = layer.chunk_width().unwrap();
                    let num_chunks_x = layer.width() / w;

                    for (i, chunk) in chunks.iter().enumerate() {
                        for y in 0..chunk.height() {
                            for x in 0..chunk.width() {
                                let tile_id = chunk.data()[(y * chunk.width()) + x];

                                if tile_id == 0 {
                                    continue;
                                }

                                let real_x = (((i % num_chunks_x) * chunk.width() * self.tilewidth) + (x * self.tilewidth)) as f32 * self.draw_scale;
                                let real_y = (((i / num_chunks_x) * chunk.height() * self.tileheight) + (y * self.tileheight)) as f32 * self.draw_scale;
                                
                                if let Some(sb) = &spritebatch {
                                    // TODO - see above
                                }
                                else {
                                    self.draw_helper_immediate(tile_id, real_x, real_y);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
