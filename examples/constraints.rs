use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::kenney_tokenize::_Keyboard___Mouse::stem_words;
use bevy_input_prompts::{FileConstraints, Pack};

fn main() {
    let no_extra_contraints = KeyCode::Tab.file_path(Pack::Kenney, &[]).unwrap();
    dbg!(no_extra_contraints);

    let with_extra_contraints = KeyCode::Tab
        .file_path(
            Pack::Kenney,
            &[
                stem_words::_outline,
                stem_words::_alternative,
                stem_words::_icon,
            ],
        )
        .unwrap();
    dbg!(with_extra_contraints);

    // if extra constraints are applied, they will only be applied if they can be fulfilled
    // for example if there are no "A" prompts with "alternative"
    // then it will just use an "A" prompt without the alternative
    // because "A" is earlier and therefore has a higher priority
    // (see the actual kenney pack for prompts that have an "alternative")
    // but then there is an "A" prompt with an "outline" so that is applied
    // basically, you can just throw a bunch of words at it and it will make a decent guess
    let a = KeyCode::KeyA
        .file_path(
            Pack::Kenney,
            &[stem_words::_alternative, stem_words::_outline],
        )
        .unwrap();
    dbg!(a);
}
