
use serde::Deserialize;
use serde_json::Value;

// only bool, float, int and string supported at the momeny
enum TileMapProperty {
    IntProperty(i32),
    FloatProperty(f32),
    BoolProperty(bool),
    StringProperty(String),
}

#[derive(Deserialize)]
struct PropertyLoader {
    name: String,
    
    #[serde(alias = "type")]
    prop_type: String,

    value: Value,
}