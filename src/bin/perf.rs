use std::time::Instant;

use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::Pack;
use tokenize_dir::file_indices;

use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse as k_kbm;

fn main() {
    use bevy_input_prompts::*;

    let d = Instant::now();
    let n = 10_000;
    for _ in 0..n {
        file_indices(
            [
                key_code::from_key_code(Pack::Kenney, KeyCode::Tab).unwrap(),
                &[
                    k_kbm::stem_words::_icon,
                    k_kbm::stem_words::_outline,
                    k_kbm::stem_words::_alternative,
                ],
            ]
            .iter()
            .flat_map(|inner| inner.iter().copied()),
        );
    }
    println!("time per file_index_safe call: {:?}", d.elapsed() / n);
}
