use bevy::prelude::*;

mod camera;
use camera::spawn_camera;

mod game;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(200.0))
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}
