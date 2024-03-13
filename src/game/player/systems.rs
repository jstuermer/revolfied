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

/// System that handles the player movement.
pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = player_query.single_mut();
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction = Vec2::new(-1.0, 0.0);
        player_transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, direction);
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction = Vec2::new(1.0, 0.0);
        player_transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, direction);
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction = Vec2::new(0.0, -1.0);
        player_transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, direction);
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction = Vec2::new(0.0, 1.0);
        player_transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, direction);
    }

    player_transform.translation += direction.extend(0.0) * PLAYER_SPEED * time.delta_seconds();
}
