use macroquad::math::Vec2;

pub trait Moveable {
    fn position(&self) -> Vec2;
    fn delta(&self) -> Vec2;
    fn move_speed(&self) -> f32;
    fn move_entity(&mut self, dt: f32) -> ();
    fn is_moving(&self) -> bool;
}

pub trait Updateable {
    fn update(&mut self, dt: f32);
}