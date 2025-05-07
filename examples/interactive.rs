use bevy::{math::vec3, prelude::*};
use bevy_input_prompts::{
    Pack, ToFile, ToFileDefault, copy_assets,
    gamepad_brand::GamepadBrand,
    kenney_tokenize::{_Keyboard___Mouse as kbm, _Xbox_Series::stem_words as xboxsw},
};

fn main() {
    // DO NOT DO THIS, PUT THIS IS build.rs, THIS IS FOR THE EXAMPLE ONLY
    let _ = copy_assets();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_kenney_keyboard_default,
                update_kenney_keyboard_double_outline,
                update_kenney_controller,
                update_kenney_controller_color,
                // update_kenney_keyboard_key,
            ),
        )
        .run();
}

#[derive(Component)]
struct KenneyKeyboardDefault;

#[derive(Component)]
struct KenneyKeyboardDoubleOutline;

#[derive(Component)]
struct KenneyController;

#[derive(Component)]
struct KenneyControllerColor;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default().with_translation(vec3(0.0, 0.0, 0.0)),
    ));

    commands.spawn((
        KenneyKeyboardDefault,
        Sprite::default(),
        Transform::default().with_translation(vec3(-100.0, -50.0, 0.0)),
    ));
    commands.spawn((
        KenneyKeyboardDoubleOutline,
        Sprite::default(),
        Transform::default().with_translation(vec3(100.0, -50.0, 0.0)),
    ));
    commands.spawn((
        KenneyController,
        Sprite::default(),
        Transform::default().with_translation(vec3(-100.0, 50.0, 0.0)),
    ));
    commands.spawn((
        KenneyControllerColor,
        Sprite::default(),
        Transform::default().with_translation(vec3(100.0, 50.0, 0.0)),
    ));
}

fn update_kenney_keyboard_default(
    mut kenney_keyboard: Query<&mut Sprite, With<KenneyKeyboardDefault>>,
    key_code_input: Option<Res<ButtonInput<KeyCode>>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(key_code_input) = key_code_input {
        if let Some(&key_code) = key_code_input.get_just_pressed().next() {
            println!("{:?}", key_code);
            // specify that we want the prompts from the "Default" directory
            if let Some(path) = key_code.file_path(Pack::Kenney, &[kbm::_Default::DIR]) {
                for mut sprite in &mut kenney_keyboard {
                    sprite.image = asset_server.load(&path);
                }
            } else {
                warn!("no prompt found");
            }
        }
    }
}

// fn update_kenney_keyboard_key(
//     mut kenney_keyboard: Query<&mut Sprite, With<KenneyKeyboardDefault>>,
//     mut keyboard_input: EventReader<KeyboardInput>,
//     asset_server: Res<AssetServer>,
// ) {
//     for x in keyboard_input.read() {
//         dbg!(x);
//     }
//     // if let Some(ki) = keyboard_input.read().next() {
//     //     dbg!()
//     //     // let lk = &ki.logical_key;
//     //     // println!("{:?}", lk);
//     //     // if let Some(path) = todo!() {
//     //     // for mut sprite in &mut kenney_keyboard {
//     //     //     sprite.image = asset_server.load(&path);
//     //     // }
//     //     // } else {
//     //     //     warn!("no prompt found");
//     //     // }
//     // }
// }

fn update_kenney_keyboard_double_outline(
    mut kenney_keyboard: Query<&mut Sprite, With<KenneyKeyboardDoubleOutline>>,
    key_code_input: Option<Res<ButtonInput<KeyCode>>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(key_code_input) = key_code_input {
        if let Some(&key_code) = key_code_input.get_just_pressed().next() {
            println!("{:?}", key_code);
            // specify that we want the prompts from the "Double" directory (which contains 2x resolution prompts)
            // and that we want the outline prompt
            if let Some(path) = key_code.file_path(
                Pack::Kenney,
                &[kbm::_Double::DIR, kbm::stem_words::_outline],
            ) {
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
    gamepad: Option<Single<&Gamepad>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(gamepad) = gamepad {
        if let Some(&gamepad_button) = gamepad.get_just_pressed().next() {
            println!("{:?}", gamepad_button);
            if let Some(path) = gamepad_button.file_path(Pack::Kenney, GamepadBrand::Xbox, &[]) {
                for mut sprite in &mut kenney_controller {
                    sprite.image = asset_server.load(&path);
                }
            } else {
                warn!("no prompt found");
            }
        }
    }
}

fn update_kenney_controller_color(
    mut kenney_controller: Query<&mut Sprite, With<KenneyControllerColor>>,
    gamepad: Option<Single<(Entity, &Gamepad)>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(gamepad) = gamepad {
        // dbg!(gamepad.1.);
        dbg!(gamepad.1.vendor_id(), gamepad.1.product_id());
        if let Some(&gamepad_button) = gamepad.1.get_just_pressed().next() {
            println!("{:?}", gamepad_button);
            if let Some(path) =
                gamepad_button.file_path(Pack::Kenney, GamepadBrand::Xbox, &[xboxsw::_color])
            {
                for mut sprite in &mut kenney_controller {
                    sprite.image = asset_server.load(&path);
                }
            } else {
                warn!("no prompt found");
            }
        }
    }
}
