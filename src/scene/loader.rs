
use serde::Deserialize;
use serde_json::{
    Result as JsonResult,
    Error as JsonError
};
use crate::entity::EntityLoader;
use std::fs::File;

static SCENE_PATH: &str = "resources/scenes/";

#[derive(Debug, Deserialize)]
pub struct SceneLoader {
    pub(crate) scene_type: String, // Required
    pub(crate) map: Option<String>,
    pub(crate) entities: Option<Vec<EntityLoader>>,
}

impl SceneLoader {
    pub fn load_from_file(file_name: &str) -> Result<Self, String> {
        let mut path = String::from(SCENE_PATH);
        path.push_str(file_name);
        path.push_str(".json");

        let file = match File::open(path) {
            Ok(f) => f,
            Err(..) => {
                return Err(String::from("Error opening file!"))
            }
        };

        let scene: Self = match serde_json::from_reader(file) {
            Ok(scn) => scn,
            Err(..) => {
                return Err(String::from("Error parsing scene JSON data!"))
            }
        };
        Ok(scene)
    }
}