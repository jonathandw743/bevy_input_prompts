use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_input::gamepad::GamepadInput;
use bevy_input_prompts::{
    kenney::{
        self, Format, KenneyGamepadSettings, KenneyKeyboardAndMouseSettings,
        gamepad_button::KenneyGamepadButton, key_code::KenneyKeyCode,
        mouse_button::KenneyMouseButton,
    },
    xelu::{
        self, LightDark, XeluGamepadSettings, XeluKeyboardAndMouseSettings,
        gamepad_axis::XeluGamepadAxis, gamepad_button::XeluGamepadButton, key_code::XeluKeyCode,
        mouse_button::XeluMouseButton,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
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
    commands.spawn(Camera2d);
    commands.spawn((
        XeluGamepadPrompt(XeluGamepadSettings {
            gamepad_brand: xelu::GamepadBrand::PS5,
        }),
        Transform::default().with_translation(vec3(-200.0, -200.0, 0.0)),
    ));
    commands.spawn((
        XeluGamepadPrompt(XeluGamepadSettings {
            gamepad_brand: xelu::GamepadBrand::SteamDeck,
        }),
        Transform::default().with_translation(vec3(-200.0, -100.0, 0.0)),
    ));
    commands.spawn((
        XeluGamepadPrompt(XeluGamepadSettings {
            gamepad_brand: xelu::GamepadBrand::Switch,
        }),
        Transform::default().with_translation(vec3(-200.0, 0.0, 0.0)),
    ));
    commands.spawn((
        XeluGamepadPrompt(XeluGamepadSettings {
            gamepad_brand: xelu::GamepadBrand::XboxSeries,
        }),
        Transform::default().with_translation(vec3(-200.0, 100.0, 0.0)),
    ));

    commands.spawn((
        XeluKeyboardAndMousePrompt(XeluKeyboardAndMouseSettings {
            light_dark: LightDark::Dark,
        }),
        Transform::default().with_translation(vec3(0.0, -200.0, 0.0)),
    ));
    commands.spawn((
        XeluKeyboardAndMousePrompt(XeluKeyboardAndMouseSettings {
            light_dark: LightDark::Light,
        }),
        Transform::default().with_translation(vec3(0.0, -100.0, 0.0)),
    ));

    commands.spawn((
        KenneyKeyboardAndMousePrompt(KenneyKeyboardAndMouseSettings {
            outline: false,
            format: Format::Default,
        }),
        Transform::default().with_translation(vec3(200.0, -200.0, 0.0)),
    ));
    commands.spawn((
        KenneyKeyboardAndMousePrompt(KenneyKeyboardAndMouseSettings {
            outline: false,
            format: Format::Double,
        }),
        Transform::default().with_translation(vec3(200.0, -100.0, 0.0)),
    ));
    commands.spawn((
        KenneyKeyboardAndMousePrompt(KenneyKeyboardAndMouseSettings {
            outline: true,
            format: Format::Default,
        }),
        Transform::default().with_translation(vec3(200.0, 0.0, 0.0)),
    ));
    commands.spawn((
        KenneyKeyboardAndMousePrompt(KenneyKeyboardAndMouseSettings {
            outline: true,
            format: Format::Double,
        }),
        Transform::default().with_translation(vec3(200.0, 100.0, 0.0)),
    ));

    let mut i = 0;
    for round_if_possible in [false, true] {
        for outline_if_possible in [false, true] {
            for format in [Format::Default, Format::Double] {
                for color_if_possible in [false, true] {
                    for gamepad_brand in [kenney::GamepadBrand::XboxSeries, kenney::GamepadBrand::PlayStation] {
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
                        ));
                        i += 1;
                    }
                }
            }
        }
    }
}

fn update(
    mut commands: Commands,
    xelu_kbm: Query<(Entity, &XeluKeyboardAndMousePrompt)>,
    xelu_gp: Query<(Entity, &XeluGamepadPrompt)>,
    kenney_kbm: Query<(Entity, &KenneyKeyboardAndMousePrompt)>,
    kenney_gp: Query<(Entity, &KenneyGamepadPrompt)>,
    asset_server: Res<AssetServer>,
    key_code_input: Option<Res<ButtonInput<KeyCode>>>,
    mouse_button_input: Option<Res<ButtonInput<MouseButton>>>,
    gamepad: Option<Single<(Entity, &Gamepad)>>,
) {
    if let Some(key_code_input) = key_code_input {
        if let Some(&key_code) = key_code_input.get_just_pressed().next() {
            dbg!(key_code);
            for (entity, xelu_kbm) in &xelu_kbm {
                commands.entity(entity).insert(Sprite {
                    image: asset_server.load(XeluKeyCode {
                        key_code,
                        settings: xelu_kbm.0,
                    }),
                    custom_size: Some(vec2(100.0, 100.0)),
                    ..default()
                });
            }
            for (entity, kenney) in &kenney_kbm {
                commands.entity(entity).insert(Sprite {
                    image: asset_server.load(KenneyKeyCode {
                        key_code,
                        settings: kenney.0,
                    }),
                    custom_size: Some(vec2(100.0, 100.0)),
                    ..default()
                });
            }
            return;
        }
    }
    if let Some(mouse_button_input) = mouse_button_input {
        if let Some(&mouse_button) = mouse_button_input.get_just_pressed().next() {
            dbg!(mouse_button);
            for (entity, xelu_kbm) in &xelu_kbm {
                commands.entity(entity).insert(Sprite {
                    image: asset_server.load(XeluMouseButton {
                        mouse_button,
                        settings: xelu_kbm.0,
                    }),
                    custom_size: Some(vec2(100.0, 100.0)),
                    ..default()
                });
            }
            for (entity, kenney) in &kenney_kbm {
                commands.entity(entity).insert(Sprite {
                    image: asset_server.load(KenneyMouseButton {
                        mouse_button,
                        settings: kenney.0,
                    }),
                    custom_size: Some(vec2(100.0, 100.0)),
                    ..default()
                });
            }
            return;
        }
    }
    if let Some(gamepad) = gamepad {
        let (entity, gamepad) = *gamepad;
        if let Some(&gamepad_button) = gamepad.get_just_pressed().next() {
            dbg!(entity);
            // dbg!(gamepad);
            dbg!(gamepad_button);
            for (entity, xelu_gp) in &xelu_gp {
                commands.entity(entity).insert(Sprite {
                    image: asset_server.load(XeluGamepadButton {
                        gamepad_button,
                        settings: xelu_gp.0,
                    }),
                    custom_size: Some(vec2(100.0, 100.0)),
                    ..default()
                });
            }
            for (entity, kenney_gp) in &kenney_gp {
                commands.entity(entity).insert(Sprite {
                    image: asset_server.load(KenneyGamepadButton {
                        gamepad_button,
                        settings: kenney_gp.0,
                    }),
                    custom_size: Some(vec2(100.0, 100.0)),
                    ..default()
                });
            }
            return;
        }
        for &gamepad_input in gamepad.get_analog_axes() {
            if let Some(value) = gamepad.get(gamepad_input) {
                if value > 0.5 || value < -0.5 {
                    match gamepad_input {
                        GamepadInput::Axis(gamepad_axis) => {
                            dbg!(entity);
                            dbg!(gamepad_axis);
                            dbg!(value);
                            for (entity, xelu_gp) in &xelu_gp {
                                commands.entity(entity).insert(Sprite {
                                    image: asset_server.load(XeluGamepadAxis {
                                        gamepad_axis,
                                        settings: xelu_gp.0,
                                    }),
                                    custom_size: Some(vec2(100.0, 100.0)),
                                    ..default()
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
