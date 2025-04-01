use bevy::{math::{vec2, vec3}, prelude::*};
use bevy_input_prompts::{kenny::{key_code::KennyKeyCode, mouse_button::KennyMouseButton, FilledOutline, Format}, xelu::{
    gamepad_button::XeluGamepadButton, key_code::XeluKeyCode, mouse_button::XeluMouseButton, GamepadBrand, LightDark
}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
struct Xelu;

#[derive(Component)]
struct Kenny;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((Xelu, Transform::default().with_translation(vec3(-50.0, 0.0, 0.0))));
    commands.spawn((Kenny, Transform::default().with_translation(vec3(50.0, 0.0, 0.0))));
}

fn update(
    mut commands: Commands,
    xelu: Single<Entity, With<Xelu>>,
    kenny: Single<Entity, With<Kenny>>,
    asset_server: Res<AssetServer>,
    key_code_input: Option<Res<ButtonInput<KeyCode>>>,
    mouse_button_input: Option<Res<ButtonInput<MouseButton>>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    if let Some(key_code_input) = key_code_input {
        if let Some(&key_code) = key_code_input.get_just_pressed().next() {
            dbg!(key_code);
            commands.entity(*xelu).insert(Sprite {
                image: asset_server.load(XeluKeyCode {
                    key_code,
                    light_dark: LightDark::Dark,
                }),
                custom_size: Some(vec2(100.0, 100.0)),
                ..default()
            });
            commands.entity(*kenny).insert(Sprite {
                image: asset_server.load(KennyKeyCode {
                    key_code,
                    filled_outline: FilledOutline::Filled,
                    format: Format::Double,
                }),
                custom_size: Some(vec2(100.0, 100.0)),
                ..default()
            });
            return;
        }
    }
    if let Some(mouse_button_input) = mouse_button_input {
        if let Some(&mouse_button) = mouse_button_input.get_just_pressed().next() {
            dbg!(mouse_button);
            commands.entity(*xelu).insert(Sprite {
                image: asset_server.load(XeluMouseButton {
                    mouse_button,
                    light_dark: LightDark::Dark,
                }),
                custom_size: Some(vec2(100.0, 100.0)),
                ..default()
            });
            commands.entity(*kenny).insert(Sprite {
                image: asset_server.load(KennyMouseButton {
                    mouse_button,
                    filled_outline: FilledOutline::Filled,
                    format: Format::Double,
                }),
                custom_size: Some(vec2(100.0, 100.0)),
                ..default()
            });
            return;
        }
    }
    for (entity, gamepad) in &gamepads {
        if let Some(&gamepad_button) = gamepad.get_just_pressed().next() {
            dbg!(entity);
            dbg!(gamepad);
            dbg!(gamepad_button);
            commands.entity(*xelu).insert(Sprite {
                image: asset_server.load(XeluGamepadButton {
                    gamepad_button,
                    gamepad_brand: GamepadBrand::XboxSeries,
                }),
                custom_size: Some(vec2(100.0, 100.0)),
                ..default()
            });
            return;
        }
    }
}
