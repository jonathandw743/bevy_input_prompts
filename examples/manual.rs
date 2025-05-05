use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::first_file_path;
use bevy_input_prompts::kenney_tokenize::_Keyboard___Mouse::stem_words;
use bevy_input_prompts::key_code::key_code_file_indices;
use bevy_input_prompts::{Pack, key_code::key_code_file_path};
use kenney_input_prompts::tokenize_dir::FILE_PATHS;
use tokenize_dir::{ToIter, file_indices};

fn main() {
    // the main way you should use this crate
    // no extra constraints
    dbg!(key_code_file_path(Pack::Kenney, KeyCode::Tab, &[],).unwrap());
    // extra constraints
    dbg!(
        key_code_file_path(
            Pack::Kenney,
            KeyCode::Tab,
            &[
                stem_words::_outline,
                stem_words::_alternative,
                stem_words::_icon,
            ],
        )
        .unwrap()
    );

    // what happens under-the hood (you can do this manually for more fine-grained controll)
    // no extra contraints
    dbg!(
        first_file_path(
            Pack::Kenney,
            key_code_file_indices(Pack::Kenney, KeyCode::Tab).unwrap()
        )
        .unwrap()
    );
    // extra constraints
    dbg!(
        first_file_path(
            Pack::Kenney,
            [
                // this is nothing special, its just a collection of constrains (see below)
                key_code_file_indices(Pack::Kenney, KeyCode::Tab).unwrap(),
                &[
                    stem_words::_outline,
                    stem_words::_alternative,
                    stem_words::_icon,
                ]
            ]
        )
        .unwrap()
    );

    // you can re-order the contraints which changes the priorty (earlier = higher priority)
    // this is not generally recomended as you might get the wrong key
    // because (for example) outline is prioritised
    dbg!(
        first_file_path(
            Pack::Kenney,
            [
                &[
                    stem_words::_outline,
                    stem_words::_alternative,
                    stem_words::_icon,
                ],
                // this is nothing special, its just a collection of constrains (see below)
                key_code_file_indices(Pack::Kenney, KeyCode::Tab).unwrap()
            ]
        )
        .unwrap()
    );

    // if you want the set of all files that satisfy the constraints and not just the best one then:
    dbg!(file_indices(
        [
            stem_words::_tab,
            stem_words::_outline,
            stem_words::_icon,
        ]
        .to_iter()
    ).iter().map(|file_index| FILE_PATHS[*file_index]).collect::<Vec<_>>());
}
