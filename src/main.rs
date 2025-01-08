use bevy::prelude::*;
// This file all about starting point where application runs we import all the other files here like game.rs, alien.rs, player.rs etc.
// here we also give resolution of screen (x:612, y:612), windows position as well 
// run function automatically runs when we start or run command cargo run.
//link our modules to our project
pub mod game;
pub mod alien;
pub mod resolution;
pub mod player;
pub mod projectile;
fn main() {
    App::new()
        .add_plugins(
            (
                //list of plugins added to the game
                DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space Invaders"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(612., 612.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),

                game::GamePlugin,
            ),
            
        )
        .run();
}
