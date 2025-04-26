use bevy::{math::vec3, prelude::*};
use bevy_input::gamepad::GamepadInput;

use bevy_input_prompts::directory_representation::_bevy_input_prompts::_kenney::_kenney_input_prompts::_Keyboard___Mouse as kenney_kbm;
// use bevy_input_prompts::directory_representation::_bevy_input_prompts::_kenney::_kenney_input_prompts:: as kenney_kbm;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_xelu_kbm,
                update_kenney_kbm,
                update_xelu_gp,
                update_kenney_gp,
            ),
        )
        .run();
}

#[derive(Component)]
struct XeluGamepadPrompt(XeluGamepadSettings);

#[derive(Component)]
struct XeluKeyboardAndMousePrompt(XeluKeyboardAndMouseSettings);

#[derive(Component)]
struct KenneyKeyboardAndMousePrompt(KenneyKeyboardAndMouseSettings);

#[derive(Component)]
struct KenneyGamepadPrompt(KenneyGamepadSettings);

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default().with_translation(vec3(300.0, 0.0, 0.0)),
    ));

    let mut i = 0;
    for gamepad_brand in [
        xelu::GamepadBrand::PS5,
        xelu::GamepadBrand::SteamDeck,
        xelu::GamepadBrand::Switch,
        xelu::GamepadBrand::XboxSeries,
    ] {
        commands.spawn((
            XeluGamepadPrompt(XeluGamepadSettings { gamepad_brand }),
            Transform::default().with_translation(vec3(
                (i / 6) as f32 * 100.0,
                (i % 6) as f32 * 100.0 - 300.0,
                0.0,
            )),
            Sprite::default(),
        ));
        i += 1;
    }

    let mut i = 0;
    for light_dark in [LightDark::Light, LightDark::Dark] {
        commands.spawn((
            XeluKeyboardAndMousePrompt(XeluKeyboardAndMouseSettings { light_dark }),
            Transform::default().with_translation(vec3(
                (i / 6) as f32 * 100.0 + 100.0,
                (i % 6) as f32 * 100.0 - 300.0,
                0.0,
            )),
            Sprite::default(),
        ));
        i += 1;
    }

    let mut i = 0;
    for outline in [false, true] {
        for icon_if_possible in [false, true] {
            for arrows_if_possible in [false, true] {
                for alternative_icon_if_possible in [false, true] {
                    for format in [Format::Default, Format::Double] {
                        commands.spawn((
                            KenneyKeyboardAndMousePrompt(KenneyKeyboardAndMouseSettings {
                                outline,
                                format,
                                icon_if_possible,
                                alternative_icon_if_possible,
                                arrows_if_possible,
                            }),
                            Transform::default().with_translation(vec3(
                                (i / 6) as f32 * 100.0 + 200.0,
                                (i % 6) as f32 * 100.0 - 300.0,
                                0.0,
                            )),
                            Sprite::default(),
                        ));
                        i += 1;
                    }
                }
            }
        }
    }

    let mut i = 0;
    for round_if_possible in [false, true] {
        for outline_if_possible in [false, true] {
            for format in [Format::Default, Format::Double] {
                for color_if_possible in [false, true] {
                    for gamepad_brand in [
                        kenney::GamepadBrand::XboxSeries,
                        kenney::GamepadBrand::PlayStation5,
                    ] {
                        commands.spawn((
                            KenneyGamepadPrompt(KenneyGamepadSettings {
                                round_if_possible,
                                outline_if_possible,
                                format,
                                color_if_possible,
                                gamepad_brand,
                            }),
                            Transform::default().with_translation(vec3(
                                (i / 6) as f32 * 100.0 + 300.0,
                                (i % 6) as f32 * 100.0 - 300.0,
                                0.0,
                            )),
                            Sprite::default(),
                        ));
                        i += 1;
                    }
                }
            }
        }
    }
}

fn update_xelu_kbm(
    mut xelu_kbm: Query<(&mut Sprite, &XeluKeyboardAndMousePrompt)>,
    key_code_input: Option<Res<ButtonInput<KeyCode>>>,
    mouse_button_input: Option<Res<ButtonInput<MouseButton>>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(key_code_input) = key_code_input {
        if let Some(&key_code) = key_code_input.get_just_pressed().next() {
            dbg!(key_code);
            for (mut sprite, settings) in &mut xelu_kbm {
                sprite.image = asset_server.load(XeluKeyCode {
                    key_code,
                    settings: settings.0,
                });
            }
        }
    }
    if let Some(mouse_button_input) = mouse_button_input {
        if let Some(&mouse_button) = mouse_button_input.get_just_pressed().next() {
            dbg!(mouse_button);
            for (mut sprite, settings) in &mut xelu_kbm {
                sprite.image = asset_server.load(XeluMouseButton {
                    mouse_button,
                    settings: settings.0,
                });
            }
        }
    }
}

fn update_kenney_kbm(
    mut kenney_kbm: Query<(&mut Sprite, &KenneyKeyboardAndMousePrompt)>,
    key_code_input: Option<Res<ButtonInput<KeyCode>>>,
    mouse_button_input: Option<Res<ButtonInput<MouseButton>>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(key_code_input) = key_code_input {
        if let Some(&key_code) = key_code_input.get_just_pressed().next() {
            dbg!(key_code);
            for (mut sprite, settings) in &mut kenney_kbm {
                sprite.image = asset_server.load(KenneyKeyCode {
                    key_code,
                    settings: settings.0,
                });
            }
        }
    }
    if let Some(mouse_button_input) = mouse_button_input {
        if let Some(&mouse_button) = mouse_button_input.get_just_pressed().next() {
            dbg!(mouse_button);
            for (mut sprite, settings) in &mut kenney_kbm {
                sprite.image = asset_server.load(KenneyMouseButton {
                    mouse_button,
                    settings: settings.0,
                });
            }
        }
    }
}

fn update_xelu_gp(
    mut xelu_gp: Query<(&mut Sprite, &XeluGamepadPrompt)>,
    asset_server: Res<AssetServer>,
    gamepad: Option<Single<(Entity, &Gamepad)>>,
) {
    if let Some(gamepad) = gamepad {
        let (_entity, gamepad) = *gamepad;
        if let Some(&gamepad_button) = gamepad.get_just_pressed().next() {
            // dbg!(entity);
            // dbg!(gamepad);
            dbg!(gamepad_button);
            for (mut sprite, settings) in &mut xelu_gp {
                sprite.image = asset_server.load(XeluGamepadButton {
                    gamepad_button,
                    settings: settings.0,
                });
            }
        }
        for &gamepad_input in gamepad.get_analog_axes() {
            if let Some(value) = gamepad.get(gamepad_input) {
                if value > 0.5 || value < -0.5 {
                    match gamepad_input {
                        GamepadInput::Axis(gamepad_axis) => {
                            // dbg!(entity);
                            dbg!(gamepad_axis);
                            dbg!(value);
                            for (mut sprite, settings) in &mut xelu_gp {
                                sprite.image = asset_server.load(XeluGamepadAxis {
                                    gamepad_axis,
                                    settings: settings.0,
                                });
                            }
                        }
                        GamepadInput::Button(_gamepad_button) => {
                            // handled by the `gamepad.get_just_pressed()` part of this example

                            // dbg!(entity);
                            // dbg!(x);
                            // dbg!(v);
                            // commands.entity(*xelu).insert(Sprite {
                            //     image: asset_server.load(XeluGamepadButton {
                            //         gamepad_button,
                            //         gamepad_brand: GamepadBrand::XboxSeries,
                            //     }),
                            //     custom_size: Some(vec2(100.0, 100.0)),
                            //     ..default()
                            // });
                        }
                    }
                }
            }
        }
    }
}

fn update_kenney_gp(
    mut kenney_gp: Query<(&mut Sprite, &KenneyGamepadPrompt)>,
    asset_server: Res<AssetServer>,
    gamepad: Option<Single<(Entity, &Gamepad)>>,
) {
    if let Some(gamepad) = gamepad {
        let (_entity, gamepad) = *gamepad;
        if let Some(&gamepad_button) = gamepad.get_just_pressed().next() {
            // dbg!(entity);
            // dbg!(gamepad);
            dbg!(gamepad_button);
            for (mut sprite, settings) in &mut kenney_gp {
                sprite.image = asset_server.load(KenneyGamepadButton {
                    gamepad_button,
                    settings: settings.0,
                });
            }
        }
        for &gamepad_input in gamepad.get_analog_axes() {
            if let Some(value) = gamepad.get(gamepad_input) {
                if value > 0.5 || value < -0.5 {
                    match gamepad_input {
                        GamepadInput::Axis(gamepad_axis) => {
                            // dbg!(entity);
                            dbg!(gamepad_axis);
                            dbg!(value);
                            for (mut sprite, settings) in &mut kenney_gp {
                                sprite.image = asset_server.load(KenneyGamepadAxis {
                                    gamepad_axis,
                                    direction: if value < -0.5 {
                                        Direction::Negative
                                    } else {
                                        Direction::Positive
                                    },
                                    settings: settings.0,
                                });
                            }
                        }
                        GamepadInput::Button(_gamepad_button) => {
                            // handled by the `gamepad.get_just_pressed()` part of this example

                            // dbg!(entity);
                            // dbg!(x);
                            // dbg!(v);
                            // commands.entity(*xelu).insert(Sprite {
                            //     image: asset_server.load(XeluGamepadButton {
                            //         gamepad_button,
                            //         gamepad_brand: GamepadBrand::XboxSeries,
                            //     }),
                            //     custom_size: Some(vec2(100.0, 100.0)),
                            //     ..default()
                            // });
                        }
                    }
                }
            }
        }
    }
}
