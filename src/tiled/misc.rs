
use serde::Deserialize;
use super::layer::MapLayer;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
pub enum MapLayerDrawOptions {
    NotSpecified,
    BelowPlayer,
    PlayerUnsorted,
    PlayerSorted,
    AbovePlayer
}

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
    pub(crate) layers: Vec<MapLayer>,
    pub(crate) tilesets: Vec<TileSetReadData>,
}
