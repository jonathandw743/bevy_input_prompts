use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::kenney_tokenize::_Keyboard___Mouse::stem_words;
use bevy_input_prompts::{FileConstraints, Pack, first_file_path};
use kenney_input_prompts::tokenize_dir::FILE_PATHS;
use tokenize_dir::ToConstraints;

fn main() {
    // the main way you should use this crate
    // no extra constraints
    dbg!(KeyCode::Tab.file_path(Pack::Kenney, &[]).unwrap());
    // extra constraints
    dbg!(
        KeyCode::Tab
            .file_path(
                Pack::Kenney,
                &[
                    stem_words::_outline,
                    stem_words::_alternative,
                    stem_words::_icon,
                ],
            )
            .unwrap()
    );

    // what happens under-the hood (you can do this manually for more fine-grained control)
    // no extra contraints
    dbg!(first_file_path(Pack::Kenney, KeyCode::Tab.file_constriants(Pack::Kenney)).unwrap());
    // extra constraints
    dbg!(
        first_file_path(
            Pack::Kenney,
            [
                // this is nothing special, its just a collection of constrains (see below)
                KeyCode::Tab.file_constriants(Pack::Kenney),
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
                KeyCode::Tab.file_constriants(Pack::Kenney)
            ]
        )
        .unwrap()
    );
    // a case where this causes issues
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
                KeyCode::KeyA.file_constriants(Pack::Kenney)
            ]
        )
        .unwrap()
    );

    // if you want the set of all files that satisfy the constraints and not just the best one then:
    dbg!(
        tokenize_dir::solve_constraints_nonstrict(
            [stem_words::_tab, stem_words::_outline, stem_words::_icon].to_constraints()
        )
        .unwrap()
        .iter()
        .map(|file_index| FILE_PATHS[*file_index])
        .collect::<Vec<_>>()
    );
    // the input to `to_iter` have different shapes
    dbg!(
        tokenize_dir::solve_constraints_nonstrict(
            [
                // make rust use a `&[&[usize]]` instead of a `&[&[usize]; 1]`
                &[stem_words::_tab, stem_words::_outline] as &[_],
                &[stem_words::_icon] as &[_],
            ]
            .to_constraints()
        )
        .unwrap()
        .iter()
        .map(|file_index| FILE_PATHS[*file_index])
        .collect::<Vec<_>>()
    );
}
