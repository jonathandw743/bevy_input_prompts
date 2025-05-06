#[cfg(feature = "use_kenney_input_prompts")]
use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse::stem_words as kbm_sw;

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
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => match self {
                // TODO: more characters
                'a' | 'A' => Some(&[kbm_sw::_a]),
                'b' | 'B' => Some(&[kbm_sw::_b]),
                'c' | 'C' => Some(&[kbm_sw::_c]),
                'd' | 'D' => Some(&[kbm_sw::_d]),
                'e' | 'E' => Some(&[kbm_sw::_e]),
                'f' | 'F' => Some(&[kbm_sw::_f]),
                'g' | 'G' => Some(&[kbm_sw::_g]),
                'h' | 'H' => Some(&[kbm_sw::_h]),
                'i' | 'I' => Some(&[kbm_sw::_i]),
                'j' | 'J' => Some(&[kbm_sw::_j]),
                'k' | 'K' => Some(&[kbm_sw::_k]),
                'l' | 'L' => Some(&[kbm_sw::_l]),
                'm' | 'M' => Some(&[kbm_sw::_m]),
                'n' | 'N' => Some(&[kbm_sw::_n]),
                'o' | 'O' => Some(&[kbm_sw::_o]),
                'p' | 'P' => Some(&[kbm_sw::_p]),
                'q' | 'Q' => Some(&[kbm_sw::_q]),
                'r' | 'R' => Some(&[kbm_sw::_r]),
                's' | 'S' => Some(&[kbm_sw::_s]),
                't' | 'T' => Some(&[kbm_sw::_t]),
                'u' | 'U' => Some(&[kbm_sw::_u]),
                'v' | 'V' => Some(&[kbm_sw::_v]),
                'w' | 'W' => Some(&[kbm_sw::_w]),
                'x' | 'X' => Some(&[kbm_sw::_x]),
                'y' | 'Y' => Some(&[kbm_sw::_y]),
                'z' | 'Z' => Some(&[kbm_sw::_z]),

                ' ' => Some(&[kbm_sw::_space]),
                '\n' => Some(&[kbm_sw::_enter]),
                '\r' => Some(&[kbm_sw::_enter]),
                '\t' => Some(&[kbm_sw::_tab]),
                '\u{8}' => Some(&[kbm_sw::_backspace]),
                '\'' => Some(&[kbm_sw::_apostrophe]),
                '<' => Some(&[kbm_sw::_less, kbm_sw::_bracket]),
                '>' => Some(&[kbm_sw::_greater, kbm_sw::_bracket]),
                '~' => Some(&[kbm_sw::_tilde]),
                '/' => Some(&[kbm_sw::_slash]),
                '?' => Some(&[kbm_sw::_question]),
                '[' => Some(&[kbm_sw::_open, kbm_sw::_bracket]),
                ']' => Some(&[kbm_sw::_close, kbm_sw::_bracket]),
                ',' => Some(&[kbm_sw::_comma]),
                '.' => Some(&[kbm_sw::_period]),
                ';' => Some(&[kbm_sw::_semicolon]),
                ':' => Some(&[kbm_sw::_colon]),
                '+' => Some(&[kbm_sw::_plus]),
                '=' => Some(&[kbm_sw::_equals]),
                '-' => Some(&[kbm_sw::_minus]),

                // not doing 3 = £, 4 = $, 5 = % etc because this depends on locale and
                // and keyboard_input will check the keycode after this as a fallback
                // which gives some default maybe US locale
                '1' => Some(&[kbm_sw::_1]),
                '!' => Some(&[kbm_sw::_exclamation]),
                '2' => Some(&[kbm_sw::_2]),
                '"' => Some(&[kbm_sw::_quote]),
                '3' => Some(&[kbm_sw::_3]),
                '4' => Some(&[kbm_sw::_4]),
                '5' => Some(&[kbm_sw::_5]),
                '6' => Some(&[kbm_sw::_6]),
                '^' => Some(&[kbm_sw::_caret]),
                '7' => Some(&[kbm_sw::_7]),
                '8' => Some(&[kbm_sw::_8]),
                '*' => Some(&[kbm_sw::_asterisk]),
                '9' => Some(&[kbm_sw::_9]),
                '0' => Some(&[kbm_sw::_0]),
                _ => None,
            },
        }
    }
}
