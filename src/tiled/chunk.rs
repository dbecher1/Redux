
use std::ops::Index;
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

#[allow(dead_code)]
impl MapChunk {

    pub(crate) fn x(&self) -> i32 {
        self.x
    }

    pub(crate) fn y(&self) -> i32 {
        self.y
    }

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

impl Index<(usize, usize)> for MapChunk {
    type Output = u32;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let idx = (y * self.width) + x;
        return match self.data.get(idx) {
            Some(n) => n,
            None => {
                // Log that there was an error and return 0
                println!("Indexing error in MapChunk!");
                &0
            }
        }
    }
}

impl Index<(i32, i32)> for MapChunk {
    type Output = u32;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        // Check for negative index
        if x < 0 || y < 0 {
            return &0;
        }
        // If index is good use the usize impl
        &self[(x as usize, y as usize)]
    }
}