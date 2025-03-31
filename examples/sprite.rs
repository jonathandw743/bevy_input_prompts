use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::system::{Commands, Res},
    sprite::Sprite,
    utils::default,
};
use bevy_asset::AssetServer;
use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::xelu::{xelu_key_code::XeluKeyCode, LightDark, XeluPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(XeluPlugin)
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
