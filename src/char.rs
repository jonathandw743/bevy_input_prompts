#[cfg(feature = "use_xelu_free_controller_key_prompts")]
use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts::_Keyboard___Mouse::stem_words as xelu;

use crate::{Pack, FileIndices};

impl FileIndices for &str {
    type Constraints<'c> = <char as FileIndices>::Constraints<'c>;
    fn file_indices<'c>(&self, pack: Pack) -> Option<Self::Constraints<'c>> {
        // TODO: there might be some packs that contain prompts that correspond to many characters
        // TODO: in that case, this method should have actual behaviour
        Some(self.chars().next()?.file_indices(pack)?)
    }
}

impl FileIndices for char {
    type Constraints<'c> = &'c [&'c [usize]];
    fn file_indices<'c>(&self, pack: Pack) -> Option<Self::Constraints<'c>> {
        // TODO: more characters
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse::stem_words::*;
                match self {
                    'a' | 'A' => Some(&[_a]),
                    'b' | 'B' => Some(&[_b]),
                    'c' | 'C' => Some(&[_c]),
                    'd' | 'D' => Some(&[_d]),
                    'e' | 'E' => Some(&[_e]),
                    'f' | 'F' => Some(&[_f]),
                    'g' | 'G' => Some(&[_g]),
                    'h' | 'H' => Some(&[_h]),
                    'i' | 'I' => Some(&[_i]),
                    'j' | 'J' => Some(&[_j]),
                    'k' | 'K' => Some(&[_k]),
                    'l' | 'L' => Some(&[_l]),
                    'm' | 'M' => Some(&[_m]),
                    'n' | 'N' => Some(&[_n]),
                    'o' | 'O' => Some(&[_o]),
                    'p' | 'P' => Some(&[_p]),
                    'q' | 'Q' => Some(&[_q]),
                    'r' | 'R' => Some(&[_r]),
                    's' | 'S' => Some(&[_s]),
                    't' | 'T' => Some(&[_t]),
                    'u' | 'U' => Some(&[_u]),
                    'v' | 'V' => Some(&[_v]),
                    'w' | 'W' => Some(&[_w]),
                    'x' | 'X' => Some(&[_x]),
                    'y' | 'Y' => Some(&[_y]),
                    'z' | 'Z' => Some(&[_z]),
                    ' ' => Some(&[_space]),
                    '\n' | '\r' => Some(&[_enter]),
                    '\t' => Some(&[_tab]),
                    '\u{8}' => Some(&[_backspace]),
                    '\'' => Some(&[_apostrophe]),
                    '<' => Some(&[_less, _bracket]),
                    '>' => Some(&[_greater, _bracket]),
                    '~' => Some(&[_tilde]),
                    '/' => Some(&[_slash]),
                    '?' => Some(&[_question]),
                    '[' => Some(&[_open, _bracket]),
                    ']' => Some(&[_close, _bracket]),
                    ',' => Some(&[_comma]),
                    '.' => Some(&[_period]),
                    ';' => Some(&[_semicolon]),
                    ':' => Some(&[_colon]),
                    '+' => Some(&[_plus]),
                    '=' => Some(&[_equals]),
                    '-' => Some(&[_minus]),
                    // not doing 3 = £, 4 = $, 5 = % etc because this depends on locale and
                    // and keyboard_input will check the keycode after this as a fallback
                    // which gives some default maybe US locale
                    '1' => Some(&[_1]),
                    '!' => Some(&[_exclamation]),
                    '2' => Some(&[_2]),
                    '"' => Some(&[_quote]),
                    '3' => Some(&[_3]),
                    '4' => Some(&[_4]),
                    '5' => Some(&[_5]),
                    '6' => Some(&[_6]),
                    '^' => Some(&[_caret]),
                    '7' => Some(&[_7]),
                    '8' => Some(&[_8]),
                    '*' => Some(&[_asterisk]),
                    '9' => Some(&[_9]),
                    '0' => Some(&[_0]),
                    _ => None,
                }
            }
            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            Pack::Xelu => match self {
                'a' | 'A' => Some(&[xelu::_A]),
                'b' | 'B' => Some(&[xelu::_B]),
                'c' | 'C' => Some(&[xelu::_C]),
                'd' | 'D' => Some(&[xelu::_D]),
                'e' | 'E' => Some(&[xelu::_E]),
                'f' | 'F' => Some(&[xelu::_F]),
                'g' | 'G' => Some(&[xelu::_G]),
                'h' | 'H' => Some(&[xelu::_H]),
                'i' | 'I' => Some(&[xelu::_I]),
                'j' | 'J' => Some(&[xelu::_J]),
                'k' | 'K' => Some(&[xelu::_K]),
                'l' | 'L' => Some(&[xelu::_L]),
                'm' | 'M' => Some(&[xelu::_M]),
                'n' | 'N' => Some(&[xelu::_N]),
                'o' | 'O' => Some(&[xelu::_O]),
                'p' | 'P' => Some(&[xelu::_P]),
                'q' | 'Q' => Some(&[xelu::_Q]),
                'r' | 'R' => Some(&[xelu::_R]),
                's' | 'S' => Some(&[xelu::_S]),
                't' | 'T' => Some(&[xelu::_T]),
                'u' | 'U' => Some(&[xelu::_U]),
                'v' | 'V' => Some(&[xelu::_V]),
                'w' | 'W' => Some(&[xelu::_W]),
                'x' | 'X' => Some(&[xelu::_X]),
                'y' | 'Y' => Some(&[xelu::_Y]),
                'z' | 'Z' => Some(&[xelu::_Z]),
                ' ' => Some(&[xelu::_Space]),
                '\n' | '\r' => Some(&[xelu::_Enter]),
                '\t' => Some(&[xelu::_Tab]),
                '\u{8}' => Some(&[xelu::_Backspace]),
                '\'' => None,
                '<' => Some(&[xelu::_Mark, xelu::_Left]),
                '>' => Some(&[xelu::_Mark, xelu::_Right]),
                '~' => Some(&[xelu::_Tilda]),
                '/' => Some(&[xelu::_Slash]),
                '?' => Some(&[xelu::_Question]),
                '[' => Some(&[xelu::_Bracket, xelu::_Left]),
                ']' => Some(&[xelu::_Bracket, xelu::_Right]),
                ',' => None,
                '.' => None,
                ';' => Some(&[xelu::_Semicolon]),
                ':' => None,
                '+' => Some(&[xelu::_Plus]),
                '=' => None,
                '-' => Some(&[xelu::_Minus]),
                '1' => Some(&[xelu::_1]),
                '2' => Some(&[xelu::_2]),
                '"' => Some(&[xelu::_Quote]),
                '3' => Some(&[xelu::_3]),
                '4' => Some(&[xelu::_4]),
                '5' => Some(&[xelu::_5]),
                '6' => Some(&[xelu::_6]),
                '7' => Some(&[xelu::_7]),
                '8' => Some(&[xelu::_8]),
                '*' => Some(&[xelu::_Asterisk]),
                '9' => Some(&[xelu::_9]),
                '0' => Some(&[xelu::_0]),
                _ => None,
            },
        }
    }
}
