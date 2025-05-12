use std::time::Instant;

use bevy_input::keyboard::KeyCode;
use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse as k_kbm;

fn main() {
    use bevy_input_prompts::*;

    let d = Instant::now();
    let n = 10_000;
    for _ in 0..n {
        let _x = KeyCode::Tab.file_path_extra(
            Pack::Kenney,
            &[
                k_kbm::stem_words::_icon,
                k_kbm::stem_words::_outline,
                k_kbm::stem_words::_alternative,
            ],
        );
        let _x = KeyCode::Tab.file_path(Pack::Kenney);
    }
    println!("time per iteration call: {:?}", d.elapsed() / n);
}
