use bevy::prelude::*;
use bevy_conway::BoardPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Game of Life".into(),
            resolution: (800., 800.).into(),
            ..default()
        }),
        ..default()
    }))
        .add_plugins(BoardPlugin)
        .add_systems(Startup, camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
