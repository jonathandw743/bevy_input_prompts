// pub mod product_ids;
// pub mod vendor_ids;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use fs_extra::dir::CopyOptions;

pub mod key_code;

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

pub fn build() {
    copy_assets(["bevy_input_prompts"]);
}

bevy_input_prompts_macros::directory_representation!("assets/bevy_input_prompts");
// bevy_input_prompts_macros::directory_representation!("assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default");

// /// Assumes each slice is sorted
// pub fn file_indices(tokens_associated_files: &[&[usize]]) -> Vec<usize> {
//     let mut i = 0;
//     while i < tokens_associated_files.len() && tokens_associated_files[i].is_empty() {
//         i += 1;
//     }
//     if i >= tokens_associated_files.len() {
//         return Vec::new();
//     }
//     let mut possible_files = Vec::from(tokens_associated_files[i]);
//     for k in 1..tokens_associated_files.len() {
//         let mut new_possible_files = Vec::with_capacity(possible_files.len());
//         let mut i = 0;
//         let mut j = 0;
//         while i < possible_files.len() && j < tokens_associated_files[k].len() {
//             if possible_files[i] == tokens_associated_files[k][j] {
//                 new_possible_files.push(possible_files[i]);
//                 i += 1;
//                 j += 1;
//             } else if possible_files[i] < tokens_associated_files[k][j] {
//                 i += 1;
//             } else if possible_files[i] > tokens_associated_files[k][j] {
//                 j += 1;
//             }
//         }
//         if !new_possible_files.is_empty() {
//             possible_files = new_possible_files;
//         }
//     }
//     possible_files
// }

pub fn file_indices<'a>(mut tokens_associated_files: impl Iterator<Item = &'a [usize]>) -> Vec<usize> {
    let Some(possible_files) = tokens_associated_files.find(|token_associated_files| !token_associated_files.is_empty()) else {
        return Vec::new();
    };
    let mut possible_files = Vec::from(possible_files);
    while let Some(token_associated_files) = tokens_associated_files.next() {
        let mut new_possible_files = Vec::with_capacity(possible_files.len());
        let mut i = 0;
        let mut j = 0;
        while i < possible_files.len() && j < token_associated_files.len() {
            if possible_files[i] == token_associated_files[j] {
                new_possible_files.push(possible_files[i]);
                i += 1;
                j += 1;
            } else if possible_files[i] < token_associated_files[j] {
                i += 1;
            } else if possible_files[i] > token_associated_files[j] {
                j += 1;
            }
        }
        if !new_possible_files.is_empty() {
            possible_files = new_possible_files;
        }
    }
    possible_files
}

// fn strict_intersect<const N: usize>(tokens_associated_files: [&[usize]; N]) -> Vec<usize> {
//     let mut possible_files = Vec::from(tokens_associated_files[0]);
//     for k in 1..tokens_associated_files.len() {
//         let mut new_possible_files = Vec::with_capacity(possible_files.len());
//         let mut i = 0;
//         let mut j = 0;
//         while i < possible_files.len() && j < tokens_associated_files[k].len() {
//             if possible_files[i] == tokens_associated_files[k][j] {
//                 new_possible_files.push(possible_files[i]);
//                 i += 1;
//                 j += 1;
//             } else if possible_files[i] < tokens_associated_files[k][j] {
//                 i += 1;
//             } else if possible_files[i] > tokens_associated_files[k][j] {
//                 j += 1;
//             }
//         }
//         possible_files = new_possible_files;
//     }
//     possible_files
// }
