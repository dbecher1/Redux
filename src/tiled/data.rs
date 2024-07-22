
use super::chunk::MapChunk;

#[derive(Debug)]
pub(crate) enum MapData {
    RawData(Vec<u32>),
    Chunks(Vec<MapChunk>),
}
