use bevy::prelude::*;
use bevy_input_prompts::{CopyAssetsError, Pack, ToFileDefault, copy_assets};

fn main() -> Result<(), CopyAssetsError> {
    // DO NOT DO THIS, PUT THIS IS build.rs, THIS IS FOR THE EXAMPLE ONLY
    copy_assets()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();

    Ok(())
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        image: asset_server.load(KeyCode::ArrowLeft.file_path(Pack::Kenney, &[]).unwrap()),
        ..default()
    });
}
