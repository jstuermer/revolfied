use bevy::{prelude::*, window::PrimaryWindow};

use super::components::Player;

const PLAYER_SPEED: f32 = 500.0;

/// Spawn the player sprite.
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/spacecraft_player.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = player_query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction += Vec3::new(-1.0, 0.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += Vec3::new(1.0, 0.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction += Vec3::new(0.0, -1.0, 0.0)
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += Vec3::new(0.0, 1.0, 0.0)
    }

    direction = direction.normalize_or_zero();
    player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
}
