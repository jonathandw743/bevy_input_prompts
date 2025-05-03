use std::time::Instant;

use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::key_code::key_code_file_path;
use tokenize_dir::{ToIter, file_indices};

use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse as k_kbm;

fn main() {
    use bevy_input_prompts::*;

    let t: &[&[usize]] = &[k_kbm::stem_words::_tab];
    let other: &[&[usize]] = &[k_kbm::stem_words::_icon, k_kbm::stem_words::_outline, k_kbm::stem_words::_alternative];

    let x = [t, other];

    let y = &[x, x];

    let i = y.to_iter();

    for x in i {
        for y in x.as_ref().iter() {
            // println!("{}", y);
        }
        println!("--");
    }

    let _ = Some([5, 6, 7]).unwrap_or_default();

    let d = Instant::now();
    let n = 10_000;
    for _ in 0..n {
        // kenney_input_prompts::tokenize_dir::FILE_PATHS[file_indices(
        //     [
        //         key_code::from_key_code(Pack::Kenney, KeyCode::Tab).unwrap_or(&[]),
        //         &[k_kbm::stem_words::_icon, k_kbm::stem_words::_outline, k_kbm::stem_words::_alternative],
        //     ]
        //     .to_iter(),
        // )[0]];
        // let _ = first_file_path(
        //     Pack::Kenney,
        //     [
        //         key_code::from_key_code(Pack::Kenney, KeyCode::Tab).unwrap_or_default(),
        //         &[k_kbm::stem_words::_icon, k_kbm::stem_words::_outline, k_kbm::stem_words::_alternative],
        //     ],
        // );
        // let _ = first_file_path(Pack::Kenney, key_code::from_key_code(Pack::Kenney, KeyCode::Tab).unwrap_or_default());
        let _x = key_code_file_path(
            Pack::Kenney,
            KeyCode::Tab,
            &[k_kbm::stem_words::_icon, k_kbm::stem_words::_outline, k_kbm::stem_words::_alternative],
        );
        let _x = key_code_file_path(Pack::Kenney, KeyCode::Tab, &[]);
    }
    println!("time per file_index_safe call: {:?}", d.elapsed() / n);
}
