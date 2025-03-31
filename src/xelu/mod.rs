use std::path::PathBuf;

use bevy_app::{App, Plugin};
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;

pub mod xelu_key_code;
pub mod xelu_key_code_name;
pub mod xelu_mouse_button;
pub mod xelu_mouse_button_name;

use std::env;
use std::fs;

pub enum LightDark {
    Light,
    Dark,
}

pub struct XeluPlugin;
impl XeluPlugin {
    const ASSET_DIRS: [&'static str; 1] = ["xelu"];
}

impl Plugin for XeluPlugin {
    fn build(&self, _app: &mut App) {
        let addon_asset_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
        let asset_paths = Self::ASSET_DIRS
            .iter()
            .map(|dir| addon_asset_dir.join(dir))
            .collect::<Vec<_>>();
        let this_asset_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets");
        fs::create_dir_all(&this_asset_dir).expect("creating asset directory failed");
        copy_items(&asset_paths, this_asset_dir, &CopyOptions::default())
            .expect("copying assets failed");
    }
}
