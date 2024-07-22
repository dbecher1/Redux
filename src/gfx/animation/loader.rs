
use std::fs::File;
use serde::Deserialize;
use crate::gfx::AnimationStateMachine;

use super::ANIMATION_PATH;

#[derive(Deserialize, Debug)]
pub(crate) struct StateMachineLoader {
    pub(crate) initial_state: String,
    pub(crate) texture_name: Option<String>,
    pub(crate) size: Option<(u32, u32)>,
    pub(crate) scale: Option<(f32, f32)>,
    pub(crate) animations: Vec<AnimationLoader>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct AnimationLoader {
    pub(crate) name: Option<String>,
    pub(crate) texture_name: Option<String>,
    pub(crate) id: u32,
    pub(crate) num_frames_x: u32,
    pub(crate) num_frames_y: u32,
    pub(crate) x_offset: Option<u32>,
    pub(crate) y_offset: Option<u32>,
    pub(crate) animation_speed: f32,
    pub(crate) size: Option<(u32, u32)>,
    pub(crate) flip: Option<bool>,
    pub(crate) scale: Option<(f32, f32)>,
    pub(crate) default_frame: Option<u32>,
}

/**
 * Expected input: the name of the animation folder in the given path
 * There is expected to be a json file of the same name in this folder
 */
pub async fn load_animations(path: &str) -> Result<AnimationStateMachine, ()> {

    let mut full_path = String::from(ANIMATION_PATH);
    full_path.push_str(path);
    full_path.push('/');
    let mut full_path_ = full_path.clone();
    full_path.push_str(path);
    full_path.push_str(".json");

    let file = match File::open(full_path) {
        Ok(f) => f,
        Err(..) => return Err(())
    };

    let mut sm_raw: StateMachineLoader = match serde_json::from_reader(file) {
        Ok(js) => js,
        Err(..) => return Err(()),
    };
    // println!("{:?}", sm_raw);

    sm_raw.texture_name = match sm_raw.texture_name {
        Some(n) => {
            full_path_.push_str(&n);
            Some(full_path_)
        },
        None => None,
    };


    Ok(AnimationStateMachine::new_from_loader(sm_raw).await)
}