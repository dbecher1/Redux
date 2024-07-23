
use gfx::SpriteBatch;
use macroquad::prelude::*;
use entity::{Player, Updateable};

pub mod tools;
pub mod tiled;
pub mod gfx;
pub mod entity;

#[allow(dead_code)]
pub mod scene;

// use crate::tools::ResourceManager;
use tiled::TileMap;

#[allow(dead_code, unused_imports)]
use tools::{ResourceManager, ImagePacker};

#[macroquad::main("test")]
async fn main() {

    let mut map = TileMap::load_map("resources/maps/help/help.tmj", &[]).await.unwrap();
    // TODO: fix this in sprite batch to localize draw scale
    map.set_draw_scale(2.0);
    let mut player = Player::new("player").await.unwrap();

    let mut sb = SpriteBatch::new();
    
    loop {
        clear_background(GRAY);

        let dt = get_frame_time();
        // UPDATE //
        player.update(dt);
        
        let p = player.position();
        sb.update(p);

        // DRAW //
        
        map.draw(&mut sb);
        player.draw(Some(&mut sb));

        sb.draw();

        next_frame().await
    }
}
