use super::{traits::Moveable, Entity, Updateable};
use macroquad::math::Vec2;
use crate::tools::InputManager;
use crate::gfx::SpriteBatch;

pub struct Player {
    entity: Entity,
}

impl Player {
    pub async fn new(path: &str) -> Result<Self, String> {
        let try_entity = Entity::new(path).await;

        match try_entity {
            Ok(entity) => Ok(Self {
                entity,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.entity.position()
    }

    pub fn draw(&self, spritebatch: Option<&mut SpriteBatch>) {
        self.entity.draw(spritebatch);
    }
}

impl Updateable for Player {
    fn update(&mut self, dt: f32) {
        self.entity.update(dt);
        self.move_entity(dt);
    }
}

impl Moveable for Player {
    fn position(&self) -> Vec2 {
        self.entity.position()
    }

    fn move_speed(&self) -> f32 {
        self.entity.move_speed()
    }

    fn move_entity(&mut self, dt: f32) -> () {
        self.entity.delta = InputManager::get_input();
        self.entity.move_entity(dt);
    }
    
    fn delta(&self) -> Vec2 {
        self.entity.delta()
    }
    
    fn is_moving(&self) -> bool {
        self.entity.is_moving()
    }
}