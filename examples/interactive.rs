use bevy::{math::vec3, prelude::*};
use bevy_input_prompts::{
    CopyAssetsError, FileConstraints, Pack, copy_assets,
    gamepad_brand::GamepadBrand,
    kenney_tokenize::{_Keyboard___Mouse as kbm, _Xbox_Series::stem_words as xboxsw},
};

fn main() -> Result<(), CopyAssetsError> {
    // DO NOT DO THIS, PUT THIS IS build.rs, THIS IS FOR THE EXAMPLE ONLY
    copy_assets()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_kenney_keyboard_default,
                update_kenney_keyboard_double_outline,
                update_kenney_mouse,
                update_kenney_controller,
                update_kenney_controller_color,
            ),
        )
        .run();

    Ok(())
}

#[derive(Component)]
struct KenneyKeyboardDefault;

#[derive(Component)]
struct KenneyKeyboardDoubleOutline;

#[derive(Component)]
struct KenneyMouse;

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
        KenneyMouse,
        Sprite::default(),
        Transform::default().with_translation(vec3(-200.0, -50.0, 0.0)),
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
    let Some(key_code_input) = key_code_input else {
        return;
    };
    let Some(&key_code) = key_code_input.get_just_pressed().next() else {
        return;
    };
    println!("{:?}", key_code);
    // specify that we want the prompts from the "Default" directory
    let Some(path) = key_code.file_path_extra(Pack::Kenney, &[kbm::_Default::DIR]) else {
        warn!("no prompt found");
        return;
    };
    for mut sprite in &mut kenney_keyboard {
        sprite.image = asset_server.load(&path);
    }
}

fn update_kenney_keyboard_double_outline(
    mut kenney_keyboard: Query<&mut Sprite, With<KenneyKeyboardDoubleOutline>>,
    key_code_input: Option<Res<ButtonInput<KeyCode>>>,
    asset_server: Res<AssetServer>,
) {
    let Some(key_code_input) = key_code_input else {
        return;
    };
    let Some(&key_code) = key_code_input.get_just_pressed().next() else {
        return;
    };
    println!("{:?}", key_code);
    // specify that we want the prompts from the "Double" directory (which contains 2x resolution prompts)
    // and that we want the outline prompt
    let Some(path) = key_code.file_path_extra(
        Pack::Kenney,
        &[kbm::_Double::DIR, kbm::stem_words::_outline],
    ) else {
        warn!("no prompt found");
        return;
    };
    for mut sprite in &mut kenney_keyboard {
        sprite.image = asset_server.load(&path);
    }
}

fn update_kenney_mouse(
    mut kenney_mouse: Query<&mut Sprite, With<KenneyMouse>>,
    mouse_button_input: Option<Res<ButtonInput<MouseButton>>>,
    asset_server: Res<AssetServer>,
) {
    let Some(mouse_button_inputs) = mouse_button_input else {
        return;
    };
    let Some(&mouse_button) = mouse_button_inputs.get_just_pressed().next() else {
        return;
    };
    println!("{:?}", mouse_button);
    let Some(path) = mouse_button.file_path_extra(Pack::Kenney, &[kbm::_Default::DIR]) else {
        warn!("no prompt found");
        return;
    };
    for mut sprite in &mut kenney_mouse {
        sprite.image = asset_server.load(&path);
    }
}

fn update_kenney_controller(
    mut kenney_controller: Query<&mut Sprite, With<KenneyController>>,
    gamepad: Option<Single<&Gamepad>>,
    asset_server: Res<AssetServer>,
) {
    let Some(gamepad) = gamepad else {
        return;
    };
    let Some(gamepad_button) = gamepad.get_just_pressed().next() else {
        return;
    };
    println!("{:?}", gamepad_button);
    let Some(path) = (gamepad_button, GamepadBrand::XboxSeries).file_path(Pack::Kenney) else {
        warn!("no prompt found");
        return;
    };
    for mut sprite in &mut kenney_controller {
        sprite.image = asset_server.load(&path);
    }
}

fn update_kenney_controller_color(
    mut kenney_controller: Query<&mut Sprite, With<KenneyControllerColor>>,
    gamepad: Option<Single<(Entity, &Gamepad)>>,
    asset_server: Res<AssetServer>,
) {
    let Some(gamepad) = gamepad else {
        return;
    };
    dbg!(gamepad.1.vendor_id(), gamepad.1.product_id());
    let Some(gamepad_button) = gamepad.1.get_just_pressed().next() else {
        return;
    };
    println!("{:?}", gamepad_button);
    let Some(path) = (gamepad_button, GamepadBrand::XboxSeries).file_path_extra(Pack::Kenney, &[xboxsw::_color])
    else {
        warn!("no prompt found");
        return;
    };
    for mut sprite in &mut kenney_controller {
        sprite.image = asset_server.load(&path);
    }
}
