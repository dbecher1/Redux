
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct MapChunk {
    data: Vec<u32>,
    height: i32,
    width: i32,
    x: i32,
    y: i32,
}

impl MapChunk {

    pub(crate) fn x(&self) -> i32 {
        self.x
    }

    pub(crate) fn y(&self) -> i32 {
        self.y
    }

    pub(crate) fn width(&self) -> i32 {
        self.width
    }

    pub(crate) fn height(&self) -> i32 {
        self.height
    }

    pub(crate) fn data(&self) -> &Vec<u32> {
        &self.data
    }
}