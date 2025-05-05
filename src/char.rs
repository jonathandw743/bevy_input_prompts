use bevy_input::keyboard::{Key, KeyCode};
#[cfg(feature = "use_kenney_input_prompts")]
use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::_Keyboard___Mouse::stem_words as kbm_sw;

use crate::{Pack, ToFile, first_file_path};

impl ToFile for char {
    type Extra = ();

    fn file_indices<'a, 'b>(&self, pack: Pack, _: ()) -> Option<&'a [&'b [usize]]> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => match self {
                'a' => Some(&[kbm_sw::_a]),
                'b' => Some(&[kbm_sw::_b]),
                'c' => Some(&[kbm_sw::_c]),
                'd' => Some(&[kbm_sw::_d]),
                'e' => Some(&[kbm_sw::_e]),
                'f' => Some(&[kbm_sw::_f]),
                'g' => Some(&[kbm_sw::_g]),
                'h' => Some(&[kbm_sw::_h]),
                'i' => Some(&[kbm_sw::_i]),
                'j' => Some(&[kbm_sw::_j]),
                'k' => Some(&[kbm_sw::_k]),
                'l' => Some(&[kbm_sw::_l]),
                'm' => Some(&[kbm_sw::_m]),
                'n' => Some(&[kbm_sw::_n]),
                'o' => Some(&[kbm_sw::_o]),
                'p' => Some(&[kbm_sw::_p]),
                'q' => Some(&[kbm_sw::_q]),
                'r' => Some(&[kbm_sw::_r]),
                's' => Some(&[kbm_sw::_s]),
                't' => Some(&[kbm_sw::_t]),
                'u' => Some(&[kbm_sw::_u]),
                'v' => Some(&[kbm_sw::_v]),
                'w' => Some(&[kbm_sw::_w]),
                'x' => Some(&[kbm_sw::_x]),
                'y' => Some(&[kbm_sw::_y]),
                'z' => Some(&[kbm_sw::_z]),
                // TODO:
                _ => None,
            },
        }
    }
}
