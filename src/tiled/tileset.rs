
use macroquad::math::Rect;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct TileSet {
    columns: u16,
    image: String,
    imageheight: u32,
    imagewidth: u32,
    margin: u8,
    name: String,
    spacing: u8,
    tilecount: u32,
    tileheight: u32,
    tilewidth: u32
}

impl TileSet {
    pub(crate) fn to_rect(&self, gid: u32) -> Rect {
        let tile_count_x = self.imagewidth / self.tilewidth;

        let x = (((gid % tile_count_x) - 1) * self.tilewidth) as f32;
        let y = ((gid / tile_count_x) * self.tileheight) as f32;
        let w = self.tilewidth as f32;
        let h = self.tileheight as f32;

        Rect {x, y, w, h}
    }

    pub(crate) fn image(&self) -> &String {
        &self.image
    }
}
