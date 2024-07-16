
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

    let mut map = load_map("resources/maps/untitled3.tmj", &[("tiles.tsx", "tiles.tsj")]).await.unwrap();
    map.set_draw_scale(2.0);
    map.set_layer_draw_type_by_idx(&[0,1], MapLayerDrawOptions::BelowPlayer);
    map.set_layer_draw_type_by_idx(&[2], MapLayerDrawOptions::PlayerSorted);
    map.set_layer_draw_type_by_idx(&[3], MapLayerDrawOptions::AbovePlayer);
    let mut player = Player::new("player").await.unwrap();

    let mut sb = SpriteBatch::new();

    let mut camera = Camera2D {
        target: player.position(),
        zoom: Vec2::new(2., 2.),
        ..Default::default()
    };

    set_camera(&camera);
    
    loop {
        clear_background(GRAY);

        let dt = get_frame_time();
        // UPDATE //
        player.update(dt);
        
        let p = player.position();
        camera.target = p;
        //camera.target = Vec2::new(-1. + (1. / p.x), 1. + (1. / p.y));
        //set_camera(&camera);

        // DRAW //
        
        //map.draw_all_immediate();
        map.draw_by_type(MapLayerDrawOptions::BelowPlayer, None);

        map.draw_by_type(MapLayerDrawOptions::PlayerSorted, Some(&mut sb));
        player.draw(Some(&mut sb));

        sb.draw();

        map.draw_by_type(MapLayerDrawOptions::AbovePlayer, None);

        set_default_camera();

        next_frame().await
    }
}
