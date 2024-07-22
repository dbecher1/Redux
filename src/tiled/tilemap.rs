
use macroquad::{
    color::WHITE,
    math::Vec2,
    texture::{
        draw_texture_ex,
        DrawTextureParams,
        Texture2D
    }
};
use crate::gfx::{DrawCommand, SpriteBatch};

use super::{data::MapData::*, layer::MapLayer, tileset::TileSet};

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
}

//#[allow(dead_code)]
impl TileMap {

    pub fn set_draw_scale(&mut self, scale: f32) {
        self.draw_scale = scale;
    }

    fn idx_to_x_y(&self, idx: usize) -> (f32, f32) {
        // FIXME: localize all scaling to draw helper
        //let x = ((idx % self.width) * self.tilewidth) as f32 * self.draw_scale;
        //let y = ((idx / self.width) * self.tileheight) as f32 * self.draw_scale;
        let x = (idx % self.width) as f32;
        let y = (idx / self.width) as f32;
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

        // scale x/y to tile size
        let x = x * size.x;
        let y = y * size.y;

        match spritebatch {
            Some(sb) => {
                let dc = DrawCommand {
                    texture: Some(self.texture.weak_clone()),
                    x,
                    y,
                    z: Some(z),
                    params,
                    ..Default::default()
                };
                sb.add(dc);
            },
            None => {
                // TODO take a look at this... useless use case?
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

    #[allow(unused_variables)]
    fn draw_layers_inner(&self, layers: &[usize], mut spritebatch: Option<&mut SpriteBatch>) {

        let draw_all = layers.len() == 0;
        
        for (n, layer) in self.layers.iter().enumerate() {

            if !draw_all && !layers.contains(&n) {
                continue;
            }

            match layer.data() {

                RawData(data) => {
                    for (i, tile) in data.iter().enumerate().filter(|(_, t)| **t != 0) {
                        let (x, y) = self.idx_to_x_y(i);
                        self.draw_helper(*tile, x, y, layer.z(), &mut spritebatch);
                    }
                },
                
                Chunks(chunks) => {
                    for chunk in chunks {
                        for y in 0..chunk.height() {
                            for x in 0..chunk.width() {

                                let tid = chunk[(x, y)];

                                if tid == 0 {
                                    continue;
                                }

                                let real_x = (x as i32 + chunk.x()) as f32;
                                let real_y = (y as i32 + chunk.y()) as f32;

                                self.draw_helper(tid, real_x, real_y, layer.z(), &mut spritebatch);
                            }
                        }
                    }
                }
            }
        }
    }
}
