use crate::tiled::TileMap;

use super::SceneObject;


pub struct OverworldScene {
    id: u32,
    map: TileMap,

}

impl SceneObject for OverworldScene {
    fn is_active(&self) -> bool {
        todo!()
    }
}