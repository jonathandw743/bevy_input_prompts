use bevy_input::keyboard::KeyCode;
use bevy_input_prompts::kenney_tokenize::_Keyboard___Mouse::stem_words;
use bevy_input_prompts::{Pack, key_code::key_code_file_path};

fn main() {
    let no_extra_contraints = key_code_file_path(Pack::Kenney, KeyCode::Tab, &[]).unwrap_or_default();
    println!("{}", no_extra_contraints);
    let with_extra_contraints = key_code_file_path(Pack::Kenney, KeyCode::Tab, &[stem_words::_outline, stem_words::_alternative, stem_words::_icon]).unwrap_or_default();
    println!("{}", with_extra_contraints);
}
