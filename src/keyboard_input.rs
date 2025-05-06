use bevy_input::keyboard::{Key, KeyCode, KeyboardInput};
#[cfg(feature = "use_kenney_input_prompts")]
use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse::stem_words as kbm_sw;

use crate::{Pack, ToFile, first_file_path};

impl ToFile for KeyboardInput {
    type Options = ();

    fn file_indices<'a, 'b>(&self, pack: Pack, _: ()) -> Option<&'a [&'b [usize]]> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                // TODO: consider which of these should have priority
                if let Some(file_indices) = self.logical_key.file_indices(pack, ()) {
                    return Some(file_indices);
                }
                if let Some(file_indices) = self.key_code.file_indices(pack, ()) {
                    return Some(file_indices);
                }
                if let Some(text) = &self.text {
                    if let Some(file_indices) = text.as_str().file_indices(pack, ()) {
                        return Some(file_indices);
                    }
                }
                None
            },
        }
    }
}
