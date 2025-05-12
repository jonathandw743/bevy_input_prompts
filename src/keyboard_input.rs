use crate::{FileConstraints, Pack};
use bevy_input::keyboard::KeyboardInput;

impl FileConstraints for &KeyboardInput {
    type Constraints<'c> = [&'c [&'c [usize]]; 3];
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        // TODO: consider which of these should have priority
        [
            self.logical_key.file_constriants(pack),
            self.key_code.file_constriants(pack),
            if let Some(text) = &self.text {
                text.as_str().file_constriants(pack)
            } else {
                // text "is `None` if the current keypress cannot be interpreted as text"
                // so i guess just unconstrained
                &[]
            },
        ]
    }
}
