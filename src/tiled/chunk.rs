
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct MapChunk {
    data: Vec<u32>,
    height: usize,
    width: usize,
    x: i32,
    y: i32,
}

impl MapChunk {
    pub(crate) fn width(&self) -> usize {
        self.width
    }

    pub(crate) fn height(&self) -> usize {
        self.height
    }

    pub(crate) fn data(&self) -> &Vec<u32> {
        &self.data
    }
}