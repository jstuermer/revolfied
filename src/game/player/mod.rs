use bevy::prelude::*;

mod components;

mod systems;
use systems::*;

/// Plugin that contains all logic and configurations related to the player.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, player_movement);
    }
}
