
use std::sync::OnceLock;
use ahash::AHashMap;
use macroquad::texture::Texture2D;

use super::ImagePacker;

static RESOURCE_MANAGER: OnceLock<ResourceManager> = OnceLock::new();

pub struct ResourceManager {

}


impl ResourceManager {

    pub fn get_manager() -> &'static Self {
        RESOURCE_MANAGER.get_or_init(|| ResourceManager::new())
    }

    fn new() -> Self {
        Self{
            
        }
    }

}
