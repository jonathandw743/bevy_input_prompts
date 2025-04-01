use bevy::{ecs::component, math::vec2, prelude::*};
use bevy_input::keyboard::KeyboardInput;
use bevy_input_prompts::xelu::{gamepad_button::XeluGamepadButton, key_code::XeluKeyCode, GamepadBrand, LightDark};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
struct ShowInput;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(ShowInput);
}

fn update(
    mut commands: Commands,
    show_input: Single<Entity, With<ShowInput>>,
    asset_server: Res<AssetServer>,
    key_code_input: Res<ButtonInput<KeyCode>>,
    gamepad_button_input: Res<ButtonInput<GamepadButton>>,
) {
    let mut entity = commands.entity(*show_input);
    if let Some(&key_code) = key_code_input.get_just_pressed().next() {
        dbg!(key_code);
        entity.insert(Sprite {
            image: asset_server.load(XeluKeyCode {
                key_code,
                light_dark: LightDark::Dark,
            }),
            custom_size: Some(vec2(100.0, 100.0)),
            ..default()
        });
        return;
    }
    if let Some(&gamepad_button) = gamepad_button_input.get_just_pressed().next() {
        dbg!(gamepad_button);
        entity.insert(Sprite {
            image: asset_server.load(XeluGamepadButton {
                gamepad_button,
                gamepad_brand: GamepadBrand::XboxSeries,
            }),
            custom_size: Some(vec2(100.0, 100.0)),
            ..default()
        });
        return
    }
}