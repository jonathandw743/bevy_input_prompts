use std::time::Instant;

use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::key_code::Version;

fn main() {
    use bevy_input_prompts::*;

    let d = Instant::now();
    let n = 10_000;
    for _ in 0..n {
        // file_indices(
        //     &mut [
        //         stem_word::_tab,
        //         stem_word::_icon,
        //         stem_word::_outline,
        //         stem_word::_alternative,
        //     ]
        //     .into_iter(),
        // );
        // file_indices(&mut [stem_word::_0, stem_word::_outline].into_iter());
        // file_indices(&mut [stem_word::_mouse, stem_word::_alt, stem_word::_2, stem_word::_caret, stem_word::_9].into_iter());
        // file_indices(&mut [stem_word::_f12].into_iter());

        file_indices(
            [
                key_code::from_key_code(Version::Kenney, KeyCode::Tab).unwrap(),
                &[
                    stem_word::_icon,
                    stem_word::_outline,
                    stem_word::_alternative,
                ],
            ]
            .iter()
            .flat_map(|inner| inner.iter().copied()),
        );

        // file_indices(&[stem_word::_tab, stem_word::_icon, stem_word::_outline, stem_word::_alternative]);
        // file_indices(&[stem_word::_0, stem_word::_outline]);
        // file_indices(&[stem_word::_mouse, stem_word::_alt, stem_word::_2, stem_word::_caret, stem_word::_9]);
        // file_indices(&[stem_word::_f12]);
    }
    println!("time per file_index_safe call: {:?}", d.elapsed().as_secs_f64() / n as f64);

    // dbg!(
    //     FILE_PATHS[file_indices(
    //         &mut [
    //             key_code::from_key_code(key_code::Version::Kenney, KeyCode::Tab).unwrap(),
    //             &[
    //                 stem_word::_icon,
    //                 stem_word::_outline,
    //                 stem_word::_alternative,
    //                 dir::_Vector,
    //             ],
    //         ]
    //         .into_iter()
    //         .flat_map(|inner| inner.iter().copied()),
    //     )[0]]
    // );
}
