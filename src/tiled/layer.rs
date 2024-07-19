
use ahash::AHashMap;
use super::{data::MapData::{self, *}, misc::MapLayerLoader, properties::TileMapProperty};

#[derive(Debug)]
//#[allow(dead_code)]
// Parallax not supported (not yet needed in the scope of what I'm doing)
pub(crate) struct MapLayer {
    data: MapData,
    width: usize,
    height: usize,
    x: u8,
    y: u8,
    z: usize,
    name: String,
    visible: bool,
    properties: Option<AHashMap<String, TileMapProperty>>,
}

impl MapLayer {

    pub(crate) fn new_from_raw(raw: MapLayerLoader) -> Self {
        let data = match raw.data {
            Some(dat) => MapData::RawData(dat),
            None => match raw.chunks {
                Some(chunk) => MapData::Chunks(chunk),
                None => panic!("Very invalid map data provided!"),
            }
        };

        let width = raw.width;
        let height = raw.height;
        let x = raw.x;
        let y = raw.y;
        let z = raw.z.unwrap_or_default();
        let name = raw.name;
        let visible = raw.visible;

        let properties = match raw.properties {
            Some(prop) => {
                let mut hm = AHashMap::new();
                for p in prop {
                    let k = p.name.clone();
                    let v = TileMapProperty::from_json_value(p.value, &p.name);
                    hm.insert(k, v);
                }
                Some(hm)
            },
            None => None,
        };

        Self {
            data,
            width,
            height,
            x,
            y,
            z,
            name,
            visible,
            properties,
        }
    }

    pub(crate) fn z(&self) -> usize {
        self.z
    }

    pub(crate) fn data(&self) -> &MapData {
        &self.data
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    // This assumes uniform chunk width.... careful!
    pub(crate) fn chunk_width(&self) -> Option<i32> {
        return match &self.data {
            Chunks(chunk) => match chunk.first() {
                Some(first) => Some(first.width()),
                _ => None,
            },
            _ => None,
        }
    }
}