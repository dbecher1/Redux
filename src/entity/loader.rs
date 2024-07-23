
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct EntityLoader {
    pub(crate) name: String,
    pub(crate) animation_data: String,
    pub(crate) move_speed: Option<f32>,
    pub(crate) default_position: Option<(f32, f32)>,
}