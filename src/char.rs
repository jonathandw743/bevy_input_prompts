use crate::{FileConstraints, Pack};

impl FileConstraints for &str {
    type Constraints<'c> = <char as FileConstraints>::Constraints<'c>;
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        // TODO: there might be some packs that contain prompts that correspond to many characters
        // TODO: in that case, this method should have actual behaviour
        self.chars()
            .next()
            .map(|c| c.file_constriants(pack))
            .unwrap_or_default()
    }
}

impl FileConstraints for char {
    type Constraints<'c> = &'c [&'c [usize]];
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        // TODO: more characters
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse::stem_words::*;
                match self {
                    'a' | 'A' => &[_a],
                    'b' | 'B' => &[_b],
                    'c' | 'C' => &[_c],
                    'd' | 'D' => &[_d],
                    'e' | 'E' => &[_e],
                    'f' | 'F' => &[_f],
                    'g' | 'G' => &[_g],
                    'h' | 'H' => &[_h],
                    'i' | 'I' => &[_i],
                    'j' | 'J' => &[_j],
                    'k' | 'K' => &[_k],
                    'l' | 'L' => &[_l],
                    'm' | 'M' => &[_m],
                    'n' | 'N' => &[_n],
                    'o' | 'O' => &[_o],
                    'p' | 'P' => &[_p],
                    'q' | 'Q' => &[_q],
                    'r' | 'R' => &[_r],
                    's' | 'S' => &[_s],
                    't' | 'T' => &[_t],
                    'u' | 'U' => &[_u],
                    'v' | 'V' => &[_v],
                    'w' | 'W' => &[_w],
                    'x' | 'X' => &[_x],
                    'y' | 'Y' => &[_y],
                    'z' | 'Z' => &[_z],
                    ' ' => &[_space],
                    '\n' | '\r' => &[_enter],
                    '\t' => &[_tab],
                    '\u{8}' => &[_backspace],
                    '\'' => &[_apostrophe],
                    '<' => &[_less, _bracket],
                    '>' => &[_greater, _bracket],
                    '~' => &[_tilde],
                    '/' => &[_slash],
                    '?' => &[_question],
                    '[' => &[_open, _bracket],
                    ']' => &[_close, _bracket],
                    ',' => &[_comma],
                    '.' => &[_period],
                    ';' => &[_semicolon],
                    ':' => &[_colon],
                    '+' => &[_plus],
                    '=' => &[_equals],
                    '-' => &[_minus],
                    // not doing 3 = £, 4 = $, 5 = % etc because this depends on locale and
                    // and keyboard_input will check the keycode after this as a fallback
                    // which gives some default maybe US locale
                    '1' => &[_1],
                    '!' => &[_exclamation],
                    '2' => &[_2],
                    '"' => &[_quote],
                    '3' => &[_3],
                    '4' => &[_4],
                    '5' => &[_5],
                    '6' => &[_6],
                    '^' => &[_caret],
                    '7' => &[_7],
                    '8' => &[_8],
                    '*' => &[_asterisk],
                    '9' => &[_9],
                    '0' => &[_0],
                    _ => &[&[]],
                }
            }
            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            Pack::Xelu => {
                use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts::_Keyboard___Mouse::stem_words::*;
                match self {
                    'a' | 'A' => &[_A],
                    'b' | 'B' => &[_B],
                    'c' | 'C' => &[_C],
                    'd' | 'D' => &[_D],
                    'e' | 'E' => &[_E],
                    'f' | 'F' => &[_F],
                    'g' | 'G' => &[_G],
                    'h' | 'H' => &[_H],
                    'i' | 'I' => &[_I],
                    'j' | 'J' => &[_J],
                    'k' | 'K' => &[_K],
                    'l' | 'L' => &[_L],
                    'm' | 'M' => &[_M],
                    'n' | 'N' => &[_N],
                    'o' | 'O' => &[_O],
                    'p' | 'P' => &[_P],
                    'q' | 'Q' => &[_Q],
                    'r' | 'R' => &[_R],
                    's' | 'S' => &[_S],
                    't' | 'T' => &[_T],
                    'u' | 'U' => &[_U],
                    'v' | 'V' => &[_V],
                    'w' | 'W' => &[_W],
                    'x' | 'X' => &[_X],
                    'y' | 'Y' => &[_Y],
                    'z' | 'Z' => &[_Z],
                    ' ' => &[_Space],
                    '\n' | '\r' => &[_Enter],
                    '\t' => &[_Tab],
                    '\u{8}' => &[_Backspace],
                    '\'' => &[&[]],
                    '<' => &[_Mark, _Left],
                    '>' => &[_Mark, _Right],
                    '~' => &[_Tilda],
                    '/' => &[_Slash],
                    '?' => &[_Question],
                    '[' => &[_Bracket, _Left],
                    ']' => &[_Bracket, _Right],
                    ',' => &[&[]],
                    '.' => &[&[]],
                    ';' => &[_Semicolon],
                    ':' => &[&[]],
                    '+' => &[_Plus],
                    '=' => &[&[]],
                    '-' => &[_Minus],
                    '1' => &[_1],
                    '2' => &[_2],
                    '"' => &[_Quote],
                    '3' => &[_3],
                    '4' => &[_4],
                    '5' => &[_5],
                    '6' => &[_6],
                    '7' => &[_7],
                    '8' => &[_8],
                    '*' => &[_Asterisk],
                    '9' => &[_9],
                    '0' => &[_0],
                    _ => &[&[]],
                }
            }
        }
    }
}
