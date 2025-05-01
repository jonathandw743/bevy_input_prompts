use std::time::Instant;

use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::key_code::Version;
use tokenize_dir::file_indices;

use kenney_input_prompts::tokenize_dir::keyboard_and_mouse as ktd;

fn main() {
    use bevy_input_prompts::*;

    let d = Instant::now();
    let n = 10_000;
    for _ in 0..n {
        file_indices(
            [
                key_code::from_key_code(Version::Kenney, KeyCode::Tab).unwrap(),
                &[
                    ktd::stem_word::_icon,
                    ktd::stem_word::_outline,
                    ktd::stem_word::_alternative,
                ],
            ]
            .iter()
            .flat_map(|inner| inner.iter().copied()),
        );
    }
    println!("time per file_index_safe call: {:?}", d.elapsed() / n);
}
