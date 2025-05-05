use bevy::{math::vec3, prelude::*};
use bevy_input_prompts::{
    Pack, copy_assets,
    gamepad_button::{GamepadBrand, gamepad_button_file_path},
    key_code::key_code_file_path,
};

fn main() {
    // DO NOT DO THIS, PUT THIS IS build.rs, THIS IS FOR THE EXAMPLE ONLY
    let _ = copy_assets();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_kenney_keyboard, update_kenney_controller))
        .run();
}

#[derive(Component)]
struct KenneyKeyboard;

#[derive(Component)]
struct KenneyController;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default().with_translation(vec3(0.0, 0.0, 0.0)),
    ));

    commands.spawn((
        KenneyKeyboard,
        Sprite::default(),
        Transform::default().with_translation(vec3(-100.0, 0.0, 0.0)),
    ));
    commands.spawn((
        KenneyController,
        Sprite::default(),
        Transform::default().with_translation(vec3(-100.0, 50.0, 0.0)),
    ));
}

fn update_kenney_keyboard(
    mut kenney_keyboard: Query<&mut Sprite, With<KenneyKeyboard>>,
    key_code_input: Option<Res<ButtonInput<KeyCode>>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(key_code_input) = key_code_input {
        if let Some(&key_code) = key_code_input.get_just_pressed().next() {
            println!("{:?}", key_code);
            if let Some(path) = key_code_file_path(Pack::Kenney, key_code, &[]) {
                for mut sprite in &mut kenney_keyboard {
                    sprite.image = asset_server.load(&path);
                }
            } else {
                warn!("no prompt found");
            }
        }
    }
}

fn update_kenney_controller(
    mut kenney_controller: Query<&mut Sprite, With<KenneyController>>,
    gamepad: Option<Single<(Entity, &Gamepad)>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(gamepad) = gamepad {
        if let Some(&gamepad_button) = gamepad.1.get_just_pressed().next() {
            println!("{:?}", gamepad_button);
            if let Some(path) = gamepad_button_file_path(
                Pack::Kenney,
                gamepad_button,
                GamepadBrand::XboxSeries,
                &[],
            ) {
                for mut sprite in &mut kenney_controller {
                    sprite.image = asset_server.load(&path);
                }
            } else {
                warn!("no prompt found");
            }
        }
    }
}
