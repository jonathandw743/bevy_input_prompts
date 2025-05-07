use bevy::prelude::*;
use bevy_input_prompts::{Pack, ToFileDefault};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        image: asset_server
            .load(KeyCode::ArrowLeft.file_path(Pack::Kenney, &[]).unwrap()),
        ..default()
    });
}
