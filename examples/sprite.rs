use bevy::prelude::*;
use bevy_input_prompts::xelu::{xelu_key_code::XeluKeyCode, LightDark};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        image: asset_server.load(XeluKeyCode {
            key_code: KeyCode::ArrowLeft,
            light_dark: LightDark::Dark,
        }),
        ..default()
    });
}
