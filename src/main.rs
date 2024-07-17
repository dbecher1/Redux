
use gfx::SpriteBatch;
use macroquad::prelude::*;
use entity::{Player, Updateable};

mod tools;
mod tiled;
mod gfx;
mod entity;

use miniquad::window::get_window_position;
// use crate::tools::ResourceManager;
use tiled::{load_map, MapLayerDrawOptions};

#[macroquad::main("test")]
async fn main() {

    //let mut map = load_map("resources/maps/untitled3.tmj", &[("tiles.tsx", "tiles.tsj")]).await.unwrap();
    let mut map = load_map("resources/maps/help/help.tmj", &[]).await.unwrap();
    map.set_draw_scale(2.0);
    //map.set_layer_draw_type_by_idx(&[0,1], MapLayerDrawOptions::BelowPlayer);
    //map.set_layer_draw_type_by_idx(&[2], MapLayerDrawOptions::PlayerSorted);
    //map.set_layer_draw_type_by_idx(&[3], MapLayerDrawOptions::AbovePlayer);
    let mut player = Player::new("player").await.unwrap();

    let mut sb = SpriteBatch::new();
    
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
