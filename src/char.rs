#[cfg(feature = "use_kenney_input_prompts")]
use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse::stem_words as kenney;
#[cfg(feature = "use_xelu_free_controller_key_prompts")]
use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts::_Keyboard___Mouse::stem_words as xelu;

use crate::{Pack, ToFile};

impl ToFile for &str {
    type Options = ();

    fn file_indices<'a, 'b>(&self, pack: Pack, extra: ()) -> Option<&'a [&'b [usize]]> {
        // TODO: there might be some packs that contain prompts that correspond to many characters
        // TODO: in that case, this method should have actual behaviour
        Some(self.chars().next()?.file_indices(pack, extra)?)
    }
}

impl ToFile for char {
    type Options = ();

    fn file_indices<'a, 'b>(&self, pack: Pack, _: ()) -> Option<&'a [&'b [usize]]> {
        // TODO: more characters
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => match self {
                'a' | 'A' => Some(&[kenney::_a]),
                'b' | 'B' => Some(&[kenney::_b]),
                'c' | 'C' => Some(&[kenney::_c]),
                'd' | 'D' => Some(&[kenney::_d]),
                'e' | 'E' => Some(&[kenney::_e]),
                'f' | 'F' => Some(&[kenney::_f]),
                'g' | 'G' => Some(&[kenney::_g]),
                'h' | 'H' => Some(&[kenney::_h]),
                'i' | 'I' => Some(&[kenney::_i]),
                'j' | 'J' => Some(&[kenney::_j]),
                'k' | 'K' => Some(&[kenney::_k]),
                'l' | 'L' => Some(&[kenney::_l]),
                'm' | 'M' => Some(&[kenney::_m]),
                'n' | 'N' => Some(&[kenney::_n]),
                'o' | 'O' => Some(&[kenney::_o]),
                'p' | 'P' => Some(&[kenney::_p]),
                'q' | 'Q' => Some(&[kenney::_q]),
                'r' | 'R' => Some(&[kenney::_r]),
                's' | 'S' => Some(&[kenney::_s]),
                't' | 'T' => Some(&[kenney::_t]),
                'u' | 'U' => Some(&[kenney::_u]),
                'v' | 'V' => Some(&[kenney::_v]),
                'w' | 'W' => Some(&[kenney::_w]),
                'x' | 'X' => Some(&[kenney::_x]),
                'y' | 'Y' => Some(&[kenney::_y]),
                'z' | 'Z' => Some(&[kenney::_z]),

                ' ' => Some(&[kenney::_space]),
                '\n' | '\r' => Some(&[kenney::_enter]),
                '\t' => Some(&[kenney::_tab]),
                '\u{8}' => Some(&[kenney::_backspace]),
                '\'' => Some(&[kenney::_apostrophe]),
                '<' => Some(&[kenney::_less, kenney::_bracket]),
                '>' => Some(&[kenney::_greater, kenney::_bracket]),
                '~' => Some(&[kenney::_tilde]),
                '/' => Some(&[kenney::_slash]),
                '?' => Some(&[kenney::_question]),
                '[' => Some(&[kenney::_open, kenney::_bracket]),
                ']' => Some(&[kenney::_close, kenney::_bracket]),
                ',' => Some(&[kenney::_comma]),
                '.' => Some(&[kenney::_period]),
                ';' => Some(&[kenney::_semicolon]),
                ':' => Some(&[kenney::_colon]),
                '+' => Some(&[kenney::_plus]),
                '=' => Some(&[kenney::_equals]),
                '-' => Some(&[kenney::_minus]),

                // not doing 3 = £, 4 = $, 5 = % etc because this depends on locale and
                // and keyboard_input will check the keycode after this as a fallback
                // which gives some default maybe US locale
                '1' => Some(&[kenney::_1]),
                '!' => Some(&[kenney::_exclamation]),
                '2' => Some(&[kenney::_2]),
                '"' => Some(&[kenney::_quote]),
                '3' => Some(&[kenney::_3]),
                '4' => Some(&[kenney::_4]),
                '5' => Some(&[kenney::_5]),
                '6' => Some(&[kenney::_6]),
                '^' => Some(&[kenney::_caret]),
                '7' => Some(&[kenney::_7]),
                '8' => Some(&[kenney::_8]),
                '*' => Some(&[kenney::_asterisk]),
                '9' => Some(&[kenney::_9]),
                '0' => Some(&[kenney::_0]),
                _ => None,
            },

            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            Pack::Xelu => match self {
                // TODO: more characters
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
