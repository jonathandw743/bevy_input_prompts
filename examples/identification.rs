use bevy::{math::vec3, prelude::*};
use bevy_input_prompts::{
    Pack, ToFile, copy_assets,
    gamepad_brand::GamepadBrand,
    kenney_tokenize::_Xbox_Series::{_Double, stem_words::_color},
};

fn main() {
    // DO NOT DO THIS, PUT THIS IS build.rs, THIS IS FOR THE EXAMPLE ONLY
    let _ = copy_assets();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_kenney_controller)
        .run();
}

#[derive(Component)]
struct KenneyController;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default()));

    commands.spawn((
        KenneyController,
        Sprite::default(),
        Transform::default().with_translation(vec3(0.0, 0.0, 0.0)),
    ));
}

fn update_kenney_controller(
    mut kenney_controller: Query<&mut Sprite, With<KenneyController>>,
    gamepad: Option<Single<(Entity, &Gamepad)>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(gamepad) = gamepad {
        if let Some(&gamepad_button) = gamepad.1.get_just_pressed().next() {
            println!("{:?}", gamepad_button);
            if let Some(path) = gamepad_button.file_path(
                Pack::Kenney,
                GamepadBrand::from_gamepad(gamepad.1).unwrap_or(GamepadBrand::Generic),
                &[_color, _Double::DIR],
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
