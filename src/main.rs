mod components;
mod constants;
mod systems;

use bevy::prelude::*;
use constants::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                title: "Pong!".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(ARENA_COLOR))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                paddle_movement,
                random_paddle,
                human_paddle,
                handle_create_ball,
                move_ball,
                handle_wall_collision,
                handle_paddle_collision,
                handle_destroy_ball,
            ),
        )
        .run();
}
