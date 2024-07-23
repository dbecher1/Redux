
use gfx::SpriteBatch;
use macroquad::prelude::*;
use entity::{Player, Updateable};

mod tools;
mod tiled;
mod gfx;
mod entity;
mod scene;

// use crate::tools::ResourceManager;
use tiled::load_map;

#[allow(unused_imports)]
use tools::{ResourceManager, ImagePacker};

#[macroquad::main("test")]
async fn main() {

    let mut map = load_map("resources/maps/help/help.tmj", &[]).await.unwrap();
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
