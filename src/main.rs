
use gfx::SpriteBatch;
use macroquad::prelude::*;
use entity::{Player, Updateable};

mod tools;
mod tiled;
mod gfx;
mod entity;

// use crate::tools::ResourceManager;
use tiled::load_map;
use tools::ResourceManager;

#[macroquad::main("test")]
async fn main() {

    //let mut map = load_map("resources/maps/untitled3.tmj", &[("tiles.tsx", "tiles.tsj")]).await.unwrap();
    let mut map = load_map("resources/maps/help/help.tmj", &[]).await.unwrap();
    map.set_draw_scale(2.0);
    let mut player = Player::new("player").await.unwrap();

    let mut sb = SpriteBatch::new();

    let mut rm = ResourceManager::get_manager();
    
    loop {
        clear_background(GRAY);

        let dt = get_frame_time();
        // UPDATE //
        player.update(dt);
        
        let p = player.position();
        sb.update(p);
        //camera.target = Vec2::new(-1. + (1. / p.x), 1. + (1. / p.y));
        //set_camera(&camera);

        // DRAW //
        
        map.draw(&mut sb);
        player.draw(Some(&mut sb));

        sb.draw();

        next_frame().await
    }
}
