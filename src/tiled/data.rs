
use super::chunk::MapChunk;

pub type TileID = u32;

#[derive(Debug)]
pub(crate) enum MapData {
    RawData(Vec<TileID>),
    Chunks(Vec<MapChunk>),
}
