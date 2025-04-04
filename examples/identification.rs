use bevy::{math::vec3, prelude::*};
use bevy_input::gamepad::GamepadInput;
use bevy_input_prompts::{
    kenney::{self, Format, KenneyGamepadSettings, gamepad_button::KenneyGamepadButton},
    xelu::{
        self, XeluGamepadSettings, gamepad_axis::XeluGamepadAxis, gamepad_button::XeluGamepadButton,
    },
};

/// will display pressed inputs as prompts if a gamepad brand is detected
/// will not use a default if the gamepad brand is not detected
/// if you are using a gamepad brand that does not get detected, it should still work in the interactive example

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_xelu_gp, update_kenney_gp))
        .run();
}

#[derive(Component)]
struct XeluGamepadPrompt(XeluGamepadSettings);

#[derive(Component)]
struct KenneyGamepadPrompt(KenneyGamepadSettings);

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default().with_translation(vec3(300.0, 0.0, 0.0)),
    ));

    commands.spawn((
        XeluGamepadPrompt(XeluGamepadSettings {
            gamepad_brand: default(),
        }),
        Transform::default().with_translation(vec3(0.0, -300.0, 0.0)),
        Sprite::default(),
    ));

    let mut i = 0;
    for round_if_possible in [false, true] {
        for outline_if_possible in [false, true] {
            for format in [Format::Default, Format::Double] {
                for color_if_possible in [false, true] {
                    commands.spawn((
                        KenneyGamepadPrompt(KenneyGamepadSettings {
                            round_if_possible,
                            outline_if_possible,
                            format,
                            color_if_possible,
                            gamepad_brand: default(),
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

fn update_xelu_gp(
    mut xelu_gp: Query<(&mut Sprite, &XeluGamepadPrompt)>,
    asset_server: Res<AssetServer>,
    gamepad: Option<Single<(Entity, &Gamepad)>>,
) {
    if let Some(gamepad) = gamepad {
        let (entity, gamepad) = *gamepad;
        let Some(gamepad_brand) = xelu::GamepadBrand::from_gamepad(gamepad) else {
            return;
        };
        if let Some(&gamepad_button) = gamepad.get_just_pressed().next() {
            dbg!(entity);
            dbg!(gamepad);
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
                            dbg!(entity);
                            dbg!(gamepad_axis);
                            dbg!(value);
                            for (mut sprite, settings) in &mut xelu_gp {
                                sprite.image = asset_server.load(XeluGamepadAxis {
                                    gamepad_axis,
                                    settings: XeluGamepadSettings {
                                        gamepad_brand,
                                        ..settings.0
                                    },
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
        let (entity, gamepad) = *gamepad;
        let Some(gamepad_brand) = kenney::GamepadBrand::from_gamepad(gamepad) else {
            return;
        };
        if let Some(&gamepad_button) = gamepad.get_just_pressed().next() {
            dbg!(entity);
            dbg!(gamepad);
            dbg!(gamepad_button);
            for (mut sprite, settings) in &mut kenney_gp {
                sprite.image = asset_server.load(KenneyGamepadButton {
                    gamepad_button,
                    settings: KenneyGamepadSettings {
                        gamepad_brand,
                        ..settings.0
                    },
                });
            }
        }
    }
}
