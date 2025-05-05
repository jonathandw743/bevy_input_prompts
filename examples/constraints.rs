use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::kenney_tokenize::_Keyboard___Mouse::stem_words;
use bevy_input_prompts::{Pack, key_code::key_code_file_path};

fn main() {
    let no_extra_contraints =
        key_code_file_path(Pack::Kenney, KeyCode::Tab, &[]).unwrap_or_default();
    println!("{}", no_extra_contraints);

    let with_extra_contraints = key_code_file_path(
        Pack::Kenney,
        KeyCode::Tab,
        &[
            stem_words::_outline,
            stem_words::_alternative,
            stem_words::_icon,
        ],
    )
    .unwrap_or_default();
    println!("{}", with_extra_contraints);

    // if extra constraints are applied, they will only be applied if they can be fulfilled
    // for example if there are no "A" prompts with "alternative"
    // then it will just use an "A" prompt without the alternative
    // because "A" is earlier and therefore has a higher priority
    // (see the actual kenney pack for prompts that have an "alternative")
    // but then there is an "A" prompt with an "outline" so that is applies
    let a = key_code_file_path(
        Pack::Kenney,
        KeyCode::KeyA,
        &[stem_words::_alternative, stem_words::_outline],
    )
    .unwrap_or_default();
    println!("{}", a);
}
