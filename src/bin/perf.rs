use std::time::Instant;

fn main() {
    use bevy_input_prompts::directory_representation::*;
    use bevy_input_prompts::file_index;

    let d = Instant::now();
    let n = 100_000;
    for _ in 0..n {
        file_index(&[_tab, _icon, _outline, _alternative]);
        file_index(&[_0, _outline]);
        file_index(&[_mouse, _alt, _2, _caret, _9]);
        file_index(&[_f12]);
    }
    println!("time per file_index_safe call: {:?}", d.elapsed() / n);
}
