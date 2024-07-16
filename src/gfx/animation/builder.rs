use macroquad::texture::Texture2D;

pub(crate) struct AnimationBuilder {
    pub(crate) texture: Option<Texture2D>,
    pub(crate) num_frames_x: u32,
    pub(crate) num_frames_y: u32,
    pub(crate) animation_speed: f32,
    pub(crate) x_offset: u32,
    pub(crate) y_offset: u32,
    pub(crate) size: (u32, u32),
    pub(crate) scale: (f32, f32),
    pub(crate) flip: bool,
    pub(crate) default_frame: u32,
}