use bevy::{math::vec3, prelude::*};
use bevy_input_prompts::{Pack, copy_assets, kenney_tokenize::_Keyboard___Mouse::stem_words, key_code::key_code_file_path};

fn main() {
    // DO NOT DO THIS, PUT THIS IS build.rs, THIS IS FOR THE EXAMPLE ONLY
    let _ = copy_assets();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_kenney_kbm, update_kenney_kbm_outline))
        .run();
}

#[derive(Component)]
struct KenneyKeyboard;

#[derive(Component)]
struct KenneyKeyboardOutline;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default().with_translation(vec3(0.0, 0.0, 0.0))));

    commands.spawn((KenneyKeyboard, Sprite::default(), Transform::default().with_translation(vec3(-100.0, 0.0, 0.0))));
    commands.spawn((KenneyKeyboardOutline, Sprite::default(), Transform::default().with_translation(vec3(100.0, 0.0, 0.0))));
}

fn update_kenney_kbm(mut kenney_kbm: Query<&mut Sprite, With<KenneyKeyboard>>, key_code_input: Option<Res<ButtonInput<KeyCode>>>, asset_server: Res<AssetServer>) {
    if let Some(key_code_input) = key_code_input {
        if let Some(&key_code) = key_code_input.get_just_pressed().next() {
            println!("{:?}", key_code);
            if let Some(path) = key_code_file_path(Pack::Kenney, key_code, &[]) {
                for mut sprite in &mut kenney_kbm {
                    sprite.image = asset_server.load(&path);
                }
            } else {
                warn!("no prompt found");
            }
        }
    }
}

fn update_kenney_kbm_outline(mut kenney_kbm: Query<&mut Sprite, With<KenneyKeyboardOutline>>, key_code_input: Option<Res<ButtonInput<KeyCode>>>, asset_server: Res<AssetServer>) {
    if let Some(key_code_input) = key_code_input {
        if let Some(&key_code) = key_code_input.get_just_pressed().next() {
            println!("{:?}", key_code);
            if let Some(path) = key_code_file_path(Pack::Kenney, key_code, &[stem_words::_outline]) {
                for mut sprite in &mut kenney_kbm {
                    sprite.image = asset_server.load(&path);
                }
            } else {
                warn!("no prompt found");
            }
        }
    }
}
