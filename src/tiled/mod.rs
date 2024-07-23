
mod chunk;
mod data;
mod layer;

// Allow dead code for the loaders mod
// These are serde structs and without doing that, we'll get millions of warnings
#[allow(dead_code)]
mod loaders;

mod tilemap;
mod tileset;
mod properties;

pub use tilemap::TileMap;
