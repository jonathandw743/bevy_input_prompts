use std::time::Instant;

use fixedbitset::FixedBitSet;

fn main() {
    use bevy_input_prompts::directory_representation::*;
    use bevy_input_prompts::{file_index, file_index_safe};
    // let x = FILE_PATHS[file_index(&[&_tab, &_icon, &_outline])];
    // let _ = std::process::Command::new("xdg-open").arg(x).output();
    // let x = FILE_PATHS[file_index(&[&_tab, &_icon, &_outline, &_alternative])];
    // let _ = std::process::Command::new("xdg-open").arg(x).output();
    // let x = FILE_PATHS[file_index(&[&_mouse, &_a, &_vertical])];
    // let _ = std::process::Command::new("xdg-open").arg(x).output();

    // let d = Instant::now();
    // let n = 100_000;
    // for _ in 0..n {
    //     file_index(&[&_tab, &_icon, &_outline, &_alternative]);
    //     file_index(&[&_0, &_outline]);
    //     file_index(&[&_mouse, &_alt, &_2, &_caret, &_9]);
    //     file_index(&[&_f12]);
    // }
    // println!("time per file_index call: {:?}", d.elapsed() / n);

    let d = Instant::now();
    let n = 100_000;
    for _ in 0..n {
        file_index_safe(&[_tab, _icon, _outline, _alternative]);
        file_index_safe(&[_0, _outline]);
        file_index_safe(&[_mouse, _alt, _2, _caret, _9]);
        file_index_safe(&[_f12]);
    }
    println!("time per file_index_safe call: {:?}", d.elapsed() / n);

    // let d = Instant::now();
    // let n = 1_000_000;
    // for _ in 0..n {
    //     let x = FixedBitSet::from_iter(_tab);
    //     let x = FixedBitSet::from_iter(_icon);
    //     let x = FixedBitSet::from_iter(_outline);
    //     let x = FixedBitSet::from_iter(_alternative);
    //     // let x: usize = FixedBitSet::new();
    //     // let y: usize = 0b0101000001010001000000010000000100000101;
    //     // let z: usize = 0b0101000001010001000000010000000100000101;
    //     // let w: usize = 0b0101000001010001000000010000000100000101;
    //     // let mut a = y;
    //     // a &= y;
    //     // a &= z;
    //     // a &= w;
    // }
    // dbg!(d.elapsed() / n);
}
