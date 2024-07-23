
mod manager;
mod battle;
mod overworld;
mod loader;

use loader::SceneLoader;
use overworld::OverworldScene;
use battle::BattleScene;

// pub use manager::SceneManager;
//use crate::tiled::TileMap;

pub enum Scene {
    Overworld(OverworldScene),
    Battle(BattleScene),
    Menu,
    Pause,
}

pub(crate) trait SceneObject {
    fn is_active(&self) -> bool;
}

#[allow(unreachable_code, unused_variables)]
impl Scene {

    pub async fn new_from_name(name: &str) -> Result<Self, String> {
        todo!();
        let loader = SceneLoader::load_from_file(name)?;

        let map = if let Some(m) = loader.map {
            
        };

        match loader.scene_type.as_str() {
            "overworld" => {
                // let map = TileMap::load_map_from_name(loader.map.unwrap_or_default()).await;
            },
            _ => {},
        }
        Err(String::new())
    }

    pub fn is_active(&self) -> bool {
        todo!();
        match self {
            Self::Menu | Self::Pause => false,
            _ => false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self {
            _ => {}
        }
    }
}