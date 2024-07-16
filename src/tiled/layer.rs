
use serde::Deserialize;
use super::chunk::MapChunk;
use super::MapLayerDrawOptions;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct MapLayer {
    data: Option<Vec<u32>>,
    chunks: Option<Vec<MapChunk>>,
    draw_type: Option<MapLayerDrawOptions>, // ugly but necessary for serde
    height: usize,
    width: usize,
    x: u8,
    y: u8,
    id: u8,
    name: String,
    parallaxx: Option<f32>,
    visible: bool,
}

impl MapLayer {

    pub(crate) fn draw_type(&self) -> MapLayerDrawOptions {
        if let Some(dt) = self.draw_type {
            dt
        }
        else {
            MapLayerDrawOptions::NotSpecified
        }
    }

    pub(crate) fn set_draw_type(&mut self, draw: MapLayerDrawOptions) {
        self.draw_type = Some(draw);
    }

    pub(crate) fn data(&self) -> &Option<Vec<u32>> {
        &self.data
    }

    pub(crate) fn chunks(&self) -> &Option<Vec<MapChunk>> {
        &self.chunks
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    pub(crate) fn chunk_width(&self) -> Option<usize> {
        return match &self.chunks {
            Some(chunk) => match chunk.first() {
                Some(first) => Some(first.width()),
                _ => None,
            },
            _ => None,
        }
    }
}