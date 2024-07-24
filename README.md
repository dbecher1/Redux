# Redux

(Name WIP - I didn't realize there was a JS library by the same name. Naming things is hard!)

A port of one of my other repos, RPGEngine, from C++ into Rust, powered by Macroquad (for now). I'm positively intrigued by the idea of Rust in game dev - borrow checking frustrations aside, Rust's memory safety guarantees and blazing speed feel so *right*. Macroquad strikes a good balance for me - batteries included, but doesn't feel like it's enforcing a specific design pattern on you, a la Bevy (no disrespect, I just don't like ECS).

The goal is to create an old school Final Fantasy-styled game, a la RPG Maker. As my other engine was still in its infancy, and this has not yet caught up to where that was yet, YMMV. Goal is to get it there; functioning scene manager/scene system, battle scenes, that's all coming next. I also need to fix my Github actions and figure out how to stop the action from trying to open a window, but alas, we pick and choose our battles.

Things that have been done:
 - Custom Tiled map parser/renderer (JSON only, XML perhaps TODO)
 - Z-index based rendering system, with Y-sorting enabled on arbitrary layers
 - Custom animation state machine renderer, with file loading from JSON (JSON schema TODO)
 - Basic entity system, with Input managing for arbitrarily defined player entity
 - Custom image loading and packing into an atlas has been programmed and tested, but yet unimplemented (but obviously TODO)
 - Scene management system has been started but is WIP

What is next:
 - Finish scenes, scene manager
   - Specifically, create battle scenes, menus and all
   - Resolve collisions in overworld scenes
   - Create pause scene with UI
   - Dialogue UI
 - Audio system - likely not using MQ's audio (Kira is the goal)
 - Flesh out the "RPG" parts of the engine
 - Ultimately minimal playable example
