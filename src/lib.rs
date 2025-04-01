use std::{
    env, fs,
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;

pub mod xelu;
pub mod kenny;

fn copy_assets<I: IntoIterator<Item = P>, P: AsRef<Path>>(asset_dirs: I) {
    let addon_asset_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let asset_paths = asset_dirs
        .into_iter()
        .map(|dir| addon_asset_dir.join(dir))
        .collect::<Vec<_>>();
    let this_asset_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets");
    fs::create_dir_all(&this_asset_dir).expect("creating asset directory failed");
    fs_extra::copy_items(
        &asset_paths,
        this_asset_dir,
        &CopyOptions {
            overwrite: false,
            skip_exist: true,
            content_only: false,
            depth: 0,
            ..Default::default()
        },
    )
    .expect("copying assets failed");
}

pub fn build_xelu() {
    copy_assets(["unknown.png"]);
    copy_assets(xelu::ASSET_DIRS);
}

pub fn build_kenny() {
    copy_assets(["unknown.png"]);
    copy_assets(kenny::ASSET_DIRS);
}

pub fn build_all() {
    copy_assets(["unknown.png"]);
    copy_assets(xelu::ASSET_DIRS);
    copy_assets(kenny::ASSET_DIRS);
}