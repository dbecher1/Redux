
use serde::Deserialize;
use serde_json::Value;

// only bool, float, int and string supported at the moment
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum TileMapProperty {
    IntProperty(i32),
    FloatProperty(f32),
    BoolProperty(bool),
    StringProperty(String),
    Null,
}

impl TileMapProperty {
    pub fn from_json_value(value: Value, hint: &str) -> Self {
        match hint {
            "int" => Self::IntProperty(value.as_i64().unwrap_or(0) as i32),
            "float" => Self::FloatProperty(value.as_f64().unwrap_or(0.) as f32),
            "bool" => Self::BoolProperty(value.as_bool().unwrap_or(false)),
            "string" => Self::StringProperty(value.as_str().unwrap_or("").to_string()),
            _ => Self::Null,
        }
    }

    pub fn get_number_value(&self) -> usize {
        match self {
            Self::IntProperty(n) => *n as usize,
            Self::FloatProperty(f) => {
                println!("Trying to unwrap a float property as a number!");
                *f as usize
            }
            _ => {
                println!("Trying to unwrap a property that is not a number!");
                0
            },
        }
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct PropertyLoader {
    pub(crate) name: String,
    
    #[serde(alias = "type")]
    pub(crate) prop_type: String,

    pub(crate) value: Value,
}