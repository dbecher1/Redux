
use serde::Deserialize;
use super::{chunk::MapChunk, properties::PropertyLoader};

#[derive(Deserialize, Debug)]
pub(crate) struct TileSetReadData {
    pub(crate) source: String,
    
    // Currently unused
    pub(crate) firstgid: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RawTileMap {
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) tilewidth: u8,
    pub(crate) tileheight: u8,
    pub(crate) infinite: bool,
    pub(crate) layers: Vec<MapLayerLoader>,
    pub(crate) tilesets: Vec<TileSetReadData>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MapLayerLoader {
    pub(crate) data: Option<Vec<u32>>,
    pub(crate) chunks: Option<Vec<MapChunk>>,
    pub(crate) properties: Option<Vec<PropertyLoader>>,
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) z: Option<usize>,
    pub(crate) id: u8,
    pub(crate) name: String,
    pub(crate) parallaxx: Option<f32>,
    pub(crate) visible: bool,
}