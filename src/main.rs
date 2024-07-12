
use macroquad::prelude::*;
use gfx::AnimationStateMachine;

mod tools;
mod tiled;
mod gfx;

// use crate::tools::ResourceManager;
use crate::tiled::load_map;

#[macroquad::main("test")]
async fn main() {

    let mut map = load_map("resources/maps/untitled3.tmj", &[("tiles.tsx", "tiles.tsj")]).await.unwrap();
    map.set_draw_scale(2.0);
    
    let mut test_anim = AnimationStateMachine::new_with_atlas("resources/sprites/test2.json").await
        .expect("Lol");
    println!("{:?}", test_anim);

    //texture::build_textures_atlas();

    loop {
        clear_background(GRAY);

        // UPDATE //
        
        // DRAW //
        
        map.draw_all_immediate();
        test_anim.update(get_frame_time());
        let _ = test_anim.draw(100., 100.).await;
        
        // map.draw_layers_immediate(&[0, 3]);

        next_frame().await
    }
}
