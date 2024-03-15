use bevy::{prelude::*, window::WindowMode};

mod camera;
use camera::spawn_camera;

mod game;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_hz(200.0))
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}
