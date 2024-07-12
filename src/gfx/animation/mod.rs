
use builder::AnimationBuilder;
use macroquad::{color::WHITE, math::Rect, texture::*};
use serde::{Deserialize, Serialize};

// pub use state_machine::AnimationStateMachine;

pub(super) mod state_machine;
mod from_atlas;
mod builder;

#[allow(dead_code)]
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Animation {

    #[serde(skip_serializing, skip_deserializing)]
    current_frame: u32,

    num_frames_x: u32,
    num_frames_y: u32,

    #[serde(skip_serializing, skip_deserializing)]
    x_offset: Option<u32>,

    #[serde(skip_serializing, skip_deserializing)]
    total_frames: u32,

    size: (u32, u32),

    #[serde(skip_serializing, skip_deserializing)]
    accumulator: f32,

    animation_speed: f32,

    texture_path: Option<String>, // only exists for easy serializing/deserializing

    #[serde(skip_serializing, skip_deserializing)]
    texture: Option<Texture2D>, // Option so serde can set a default value cleanly (None)
} 

impl Default for Animation {
    fn default() -> Self {
        Self {
            current_frame: 0,
            num_frames_x: 1,
            num_frames_y: 1, // right now assuming "strip" style - y will always = 1
            total_frames: 1,
            size: (0, 0),
            accumulator: 0.,
            animation_speed: 0.,
            texture_path: None,
            texture: None,
            x_offset: None,
        }
    } 
}

#[allow(dead_code)]
impl Animation {

    async fn new(builder: &AnimationBuilder) -> Result<Self, String> { 

        let (num_frames_x, num_frames_y) = builder.frames();
        let size = builder.size();
        let animation_speed = builder.animation_speed();
        let total_frames = num_frames_x * num_frames_y;

        let current_frame = 0;
        let accumulator = 0.;
        let x_offset = builder.x_offset();

        // going to assuming a root path of resources/sprites/
        let (texture, texture_path) = match builder.texture() {
            None => {
                if let Some(file_name) = builder.file_name() {
                    if file_name.is_empty() {
                        return Err("Must provide a valid file path!".to_string());
                    }
                    let mut tex_path = "resources/sprites/".to_owned();
                    tex_path.push_str(file_name);
                    println!("{}", tex_path);

                    match load_texture(&tex_path).await {
                        Ok(tex) => (Some(tex), Some(tex_path)),
                        Err(e) => {
                            return Err(e.to_string())
                        }
                    }
                }
                else {
                    return Err(String::from("Something went wrong loading textures!"));
                }
            },
            Some(tex) => (Some(tex.weak_clone()), None)
        };

        Ok(Self {
            current_frame,
            num_frames_x,
            num_frames_y,
            animation_speed,
            total_frames,
            size,
            accumulator,
            texture_path,
            texture,
            x_offset
        })
    }

    // TODO: only supporting 1 row animations right now
    pub fn update(&mut self, dt: f32) -> () {
        self.accumulator += dt;
        if self.accumulator >= self.animation_speed {
            self.accumulator = 0.;
            self.current_frame = (self.current_frame + 1) % self.num_frames_x;
        }
    }

    pub async fn draw(&mut self, position_x: f32, position_y: f32) -> () {

        if let Some(t) = &self.texture {
            let mut d = DrawTextureParams::default();

            let offset = match self.x_offset {
                Some(xoff) => xoff,
                None => 0
            };

            d.source = Some(Rect::new(
                ((self.current_frame % self.num_frames_x) * self.size.0) as f32,
                (self.size.1 * offset) as f32, 
                self.size.0 as f32,
                self.size.1 as f32,
            ));
    
            draw_texture_ex(t, position_x, position_y, WHITE, d);
        }
        else {
            // Workaround so I don't have to implement a custom deserialize impl
            // Adds slight overhead the literal first loop iteration for everything serialized
            // But should be fine beyond that
            // If we're in the situation where else executes, we don't have to worry about the overhead

            let path = match &self.texture_path {
                Some(p) => p.clone(),
                None => String::new(),
            };

            if let Ok(tex) = load_texture(&path).await  {
                self.texture = Some(tex);
                self.total_frames = self.num_frames_x * self.num_frames_y;

                // TODO: figure out the Box impl so we don't skip the first frame
                // self.draw(position_x, position_y).await; // recursive call after successful tex load
            }
            else {
                eprintln!("Attempted to draw a sprite with no texture data!");
            }
            
        }
    }

    pub fn size(&self) -> (u32, u32) {
        return self.size
    }
}
