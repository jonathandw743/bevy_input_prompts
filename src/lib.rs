use std::{
    env, fs,
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;

pub mod xelu;

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
        &CopyOptions::default().skip_exist(true),
    )
    .expect("copying assets failed");
}
