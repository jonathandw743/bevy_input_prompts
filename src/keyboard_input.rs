use crate::{Pack, FileIndices};
use bevy_input::keyboard::KeyboardInput;

impl FileIndices for KeyboardInput {
    type Constraints<'c> = &'c [&'c [usize]];
    fn file_indices<'c>(&self, pack: Pack) -> Option<Self::Constraints<'c>> {
        // TODO: consider which of these should have priority
        if let Some(file_indices) = self.logical_key.file_indices(pack) {
            return Some(file_indices);
        }
        if let Some(file_indices) = self.key_code.file_indices(pack) {
            return Some(file_indices);
        }
        if let Some(text) = &self.text {
            if let Some(file_indices) = text.as_str().file_indices(pack) {
                return Some(file_indices);
            }
        }
        None
    }
}
