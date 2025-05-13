use bevy_input::mouse::MouseButton::{self, *};

use crate::{FileConstraints, Pack};

impl FileConstraints for &MouseButton {
    type Constraints<'c> = &'c [&'c [usize]];
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        match pack {
            #[cfg(feature = "kenney_input_prompts")]
            Pack::Kenney => {
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse::stem_words::*;
                
                let input: &[&[usize]] = match self {
                    Left => &[_left, _mouse],
                    Right => &[_right, _mouse],
                    Middle => &[_scroll],
                    Back => &[&[]],
                    Forward => &[&[]],
                    Other(_) => &[&[]],
                };
                input
            }
            #[cfg(feature = "xelu_free_controller_key_prompts")]
            Pack::Xelu => {
                use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts::_Keyboard___Mouse::stem_words::*;

                let input: &[&[usize]] = match self {
                    Left => &[_Left, _Mouse],
                    Right => &[_Right, _Mouse],
                    Middle => &[_Middle, _Mouse],
                    Back => &[&[]],
                    Forward => &[&[]],
                    Other(_) => &[&[]],
                };
                input
            }
        }
    }
}
