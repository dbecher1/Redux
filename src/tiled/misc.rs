
use serde::Deserialize;
use super::layer::RawMapLayerData;

#[derive(Deserialize, Debug)]
pub(crate) struct TileSetReadData {
    pub(crate) source: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct RawTileMap {
    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) tilewidth: u8,
    pub(crate) tileheight: u8,
    pub(crate) infinite: bool,
    pub(crate) layers: Vec<RawMapLayerData>,
    pub(crate) tilesets: Vec<TileSetReadData>,
}
