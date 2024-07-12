use macroquad::texture::Texture2D;

use super::Animation;



#[allow(dead_code)]
pub struct AnimationBuilder {
    file_name: Option<String>, // relative to the resources folder
    size: (u32, u32),
    frames: (u32, u32),
    animation_speed: f32,

    // Ugly struct because of retrofitted code, yeehaw!
    x_offset: Option<u32>,
    texture: Option<Texture2D>,
    flip: Option<bool>,
}

impl Default for AnimationBuilder {
    fn default() -> Self {
        Self {
            file_name: Some(String::new()),
            size: (0, 0),
            frames: (1, 1),
            animation_speed: 0.,
            texture: None,
            x_offset: None,
            flip: None,
        }
    }
}

#[allow(dead_code)]
impl AnimationBuilder {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: Some(file_name.to_string()),
            ..Default::default()
        }
    }

    pub fn new_with_texture(texture: Texture2D) -> Self {
        let texture = Some(texture.weak_clone());
        Self {
            texture,
            ..Default::default()
        }
    }

    pub fn with_flip(self, flip: Option<bool>) -> Self {
        Self {
            flip,
            ..self
        }
    }

    pub fn with_file_name(self, fname: &str) -> Self {
        Self {
            file_name: Some(fname.to_string()),
            ..self
        }
    }

    pub fn with_dimensions(self, x: u32, y: u32) -> Self {
        Self {
            size: (x, y),
            ..self
        }
    }

    pub fn with_frames_x(self, num_frames_x: u32) -> Self {
        Self {
            frames: (num_frames_x, self.frames.1),
            ..self
        }
    }

    pub fn with_frames_y(self, num_frames_y: u32) -> Self {
        Self {
            frames: (self.frames.0, num_frames_y),
            ..self
        }
    }

    pub fn with_animation_speed(self, speed: f32) -> Self {
        Self {
            animation_speed: speed,
            ..self
        }
    }

    pub fn with_x_offset(self, x_offset: u32) -> Self {
        Self {
            x_offset: Some(x_offset),
            ..self
        }
    }

    pub async fn build(&self) -> Result<Animation, String> {
        Animation::new(self).await
    }
    
    pub fn file_name(&self) -> &Option<String> {
        &self.file_name
    }

    pub fn frames(&self) -> (u32, u32) {
        self.frames
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn animation_speed(&self) -> f32 {
        self.animation_speed
    }

    pub fn texture(&self) -> &Option<Texture2D> {
        &self.texture
    }

    pub fn x_offset(&self) -> Option<u32> {
        self.x_offset
    }
}
