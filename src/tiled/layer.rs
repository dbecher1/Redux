
use ahash::AHashMap;
use super::{data::MapData, loaders::MapLayerLoader, properties::TileMapProperty};

static DEPTH_PROPERTY_NAME: &str = "depth";

#[derive(Debug)]
#[allow(dead_code)]
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
    properties: AHashMap<String, TileMapProperty>,
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

        let mut properties = AHashMap::new();
        match raw.properties {
            Some(prop) => {
                for p in prop {
                    let k = p.name;
                    let v = TileMapProperty::from_json_value(p.value, &p.prop_type);
                    properties.insert(k, v);
                }
            },
            None => {},
        };

        // println!("{:?}", &properties);
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

    // Checks the depth value first since that's what I want to use
    // Leaving in the code for the old z value though
    pub(crate) fn z(&self) -> usize {
        match self.properties.get(DEPTH_PROPERTY_NAME) {
            Some(z) => z.get_number_value(),
            None => self.z,
        }
    }

    pub(crate) fn data(&self) -> &MapData {
        &self.data
    }

    #[allow(dead_code)]
    pub(crate) fn width(&self) -> usize {
        self.width
    }
}