use bevy::prelude::*;
use bevy_input_prompts::{
    CopyAssetsError, FileConstraints, Pack, copy_assets,
    gamepad_brand::GamepadBrand,
    kenney_tokenize::_Xbox_Series::{_Double, stem_words::_color},
};

fn main() -> Result<(), CopyAssetsError> {
    // DO NOT DO THIS, PUT THIS IS build.rs, THIS IS FOR THE EXAMPLE ONLY
    copy_assets()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_kenney_controller)
        .run();

    Ok(())
}

#[derive(Component)]
struct KenneyController;

#[derive(Component)]
struct XeluController;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default()));

    commands.spawn((
        KenneyController,
        Sprite {
            custom_size: Some(vec2(100.0, 100.0)),
            ..default()
        },
        Transform::default().with_translation(vec3(-100.0, 0.0, 0.0)),
    ));
    commands.spawn((
        XeluController,
        Sprite {
            custom_size: Some(vec2(100.0, 100.0)),
            ..default()
        },
        Transform::default().with_translation(vec3(100.0, 0.0, 0.0)),
    ));
}

fn update_kenney_controller(
    mut kenney_controller: Query<&mut Sprite, (With<KenneyController>, Without<XeluController>)>,
    mut xelu_controller: Query<&mut Sprite, (With<XeluController>, Without<KenneyController>)>,
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

    let Some(path) = (
        // this is the point of this example, normally you should `unwrap_or`
        GamepadBrand::from_gamepad(&gamepad).expect("gamepad cannot be identified"),
        gamepad_button,
    )
        .file_path_extra(Pack::Kenney, &[_color, _Double::DIR])
    else {
        warn!("no prompt found");
        return;
    };
    for mut sprite in &mut kenney_controller {
        sprite.image = asset_server.load(&path);
    }

    let Some(path) = (
        GamepadBrand::from_gamepad(&gamepad).expect("gamepad cannot be identified"),
        gamepad_button,
    )
        .file_path(Pack::Xelu)
    else {
        warn!("no prompt found");
        return;
    };
    for mut sprite in &mut xelu_controller {
        sprite.image = asset_server.load(&path);
    }
}
