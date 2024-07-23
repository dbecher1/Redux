
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
use super::loaders::RawTileMap;
use macroquad::texture::{load_texture, FilterMode};
use std::fs::File;
use serde_json::Result;
use ahash::AHashSet;
use super::{data::MapData::*, layer::MapLayer, tileset::TileSet};

static TILEMAP_PATH: &str = "resources/maps/";


#[derive(Debug)]
pub struct TileMap {
    pub(super) texture: Texture2D,
    pub(super) tileset: TileSet,
    pub(super) layers: Vec<MapLayer>,
    pub(super) width: usize,
    pub(super) tileheight: usize,
    pub(super) tilewidth: usize,
    pub(super) draw_scale: f32,
}


impl TileMap {

    #[allow(dead_code)]
    pub async fn load_map_from_name(name: &str) -> Result<Self> {
        let mut path = String::from(TILEMAP_PATH);
        path.push_str(name);
        path.push_str("/");
        path.push_str(name);
        path.push_str(".tmj");

        let map = Self::load_map(&path, &[]).await?;
        Ok(map)
    }

    pub async fn load_map(path: &str, tilesets: &[(&str, &str)]) -> Result<Self> {
        // Load the tile map
        let file = File::open(path).unwrap();
        let raw_map: RawTileMap = serde_json::from_reader(file)?;
    
        let file_path_str = String::from(path);
        let fp_split = file_path_str.rsplit_once('/').unwrap();
        let file_path = fp_split.0;
    
        // Load the tile set/s
        let remap = tilesets.len() > 0;
        let mut set_file_names = Vec::new();
    
        // Iterate through the loaded data, and if we have remapped names
        // find them in the inner loop
        for set in &raw_map.tilesets {
            if remap {
                for ts in tilesets {
                    if ts.0.eq_ignore_ascii_case(set.source.as_str()) {
                        set_file_names.push(ts.1);
                    }
                }
            }
            else {
                // If not remapping just add to the vec
                set_file_names.push(set.source.as_str());
            }
        }
    
        let mut tilesets = Vec::new();
        let mut images = AHashSet::new();
    
        for set in &set_file_names {
            let mut name = String::new();
            name += file_path;
            name += "/";
            name += set;
            // println!("TILE SET NAME: {}", name);
            let ts_file = File::open(name).unwrap();
            let ts: TileSet = serde_json::from_reader(ts_file)?;
    
            images.insert(ts.image().clone());
    
            tilesets.push(ts);
        }
        let tileset = tilesets[0].clone();
    
        // for ts in &tilesets { println!("TS READ: {:?}", ts); }
    
        // Load images
    
        let mut textures = Vec::new();
    
        for img in &images {
            let mut name = String::new();
            name += file_path;
            name += "/";
            name += img;
    
            let texture = load_texture(&name).await.unwrap();
            texture.set_filter(FilterMode::Nearest);
            textures.push(texture);
        }
        let texture = textures[0].clone();
    
        let layers = raw_map.layers
            .into_iter()
            .map(|l| MapLayer::new_from_raw(l))
            .collect::<Vec<MapLayer>>();
        // println!("{:?}", raw_map);
    
        let map = TileMap {
            texture,
            tileset,
            layers,
            width: raw_map.width,
            tilewidth: raw_map.tilewidth as usize,
            tileheight: raw_map.tileheight as usize,
            draw_scale: 1.,
        };
        // println!("{:?}", map);
        Ok(map)
    }

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
