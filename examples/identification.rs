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

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default()));

    commands.spawn((KenneyController, Sprite::default()));
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
    // this is the point of this example, normally you should `unwrap_or`
    let Some(path) = (
        GamepadBrand::from_gamepad(&gamepad).expect("gamepad cannot be identified"),
        gamepad_button,
    )
        .file_path(Pack::Kenney, &[_color, _Double::DIR])
    else {
        warn!("no prompt found");
        return;
    };
    for mut sprite in &mut kenney_controller {
        sprite.image = asset_server.load(&path);
    }
}
