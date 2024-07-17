
use macroquad::texture::{load_texture,FilterMode};
use tileset::TileSet;
use std::fs::File;
use serde_json::Result;
use ahash::AHashSet;

mod chunk;
mod layer;
mod misc;
mod tilemap;
mod tileset;
mod properties;

use misc::RawTileMap;

pub use tilemap::TileMap;
pub use misc::MapLayerDrawOptions;

/**
 * Biggest TODO: figure out how best to label the "player" layer for Y-sorting, and therefore to distinguish best drawing below/above player
*/

pub async fn load_map(path: &str, tilesets: &[(&str, &str)]) -> Result<TileMap> {
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
    // println!("{:?}", raw_map);

    let map = TileMap {
        texture,
        tileset,
        layers: raw_map.layers,
        width: raw_map.width,
        height: raw_map.height,
        tilewidth: raw_map.tilewidth as usize,
        tileheight: raw_map.tileheight as usize,
        draw_scale: 1.,
        player_layer: 1,
    };
    // println!("{:?}", map);
    Ok(map)
}