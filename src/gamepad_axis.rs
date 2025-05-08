use Direction::*;
use bevy_input::gamepad::GamepadAxis::{self, *};

use crate::{
    FileIndices, Pack,
    gamepad_brand::GamepadBrand::{self, *},
};

#[derive(Clone, Copy)]
pub enum Direction {
    Positive,
    Negative,
    Bidirectional,
}

impl FileIndices for (GamepadBrand, GamepadAxis, Option<Direction>) {
    type Constraints<'c> = [&'c [&'c [usize]]; 2];
    fn file_indices<'c>(&self, pack: Pack) -> Option<Self::Constraints<'c>> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                Some([(self.0, self.1).file_indices(pack)?, (self.1, self.2).file_indices(pack)?])
            },
            // TODO:
            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            #[rustfmt::skip]
            Pack::Xelu => match self {
                _ => None,
            },
        }
    }
}

impl FileIndices for (GamepadAxis, Option<Direction>) {
    type Constraints<'c> = &'c [&'c [usize]];
    fn file_indices<'c>(&self, pack: Pack) -> Option<Self::Constraints<'c>> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::stem_words;
                match self {
                    (LeftStickX | RightStickX, None) => Some(&[]),
                    (LeftStickX | RightStickX, Some(Positive)) => Some(&[stem_words::_right]),
                    (LeftStickX | RightStickX, Some(Negative)) => Some(&[stem_words::_left]),
                    (LeftStickX | RightStickX, Some(Bidirectional)) => {
                        Some(&[stem_words::_horizontal])
                    }
                    (LeftZ, _) => Some(&[]),
                    (LeftStickY | RightStickY, None) => Some(&[]),
                    (LeftStickY | RightStickY, Some(Positive)) => Some(&[stem_words::_up]),
                    (LeftStickY | RightStickY, Some(Negative)) => Some(&[stem_words::_down]),
                    (LeftStickY | RightStickY, Some(Bidirectional)) => {
                        Some(&[stem_words::_vertical])
                    }
                    (RightZ, _) => Some(&[]),
                    (Other(_), _) => Some(&[]),
                }
            }
            // TODO:
            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            Pack::Xelu => Some(&[]),
        }
    }
}

impl FileIndices for (GamepadBrand, GamepadAxis) {
    type Constraints<'c> = &'c [&'c [usize]];
    fn file_indices<'c>(&self, pack: Pack) -> Option<Self::Constraints<'c>> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::{
                    _Generic::stem_words as generic, _Nintendo_Gamecube::stem_words as gamecube,
                    _Nintendo_Switch::stem_words as switch,
                    _Nintendo_Switch_2::stem_words as switch2, _Nintendo_Wii::stem_words as wii,
                    _Nintendo_WiiU::stem_words as wiiu, _PlayStation_Series::stem_words as ps,
                    _Steam_Controller::stem_words as steam, _Steam_Deck::stem_words as steamdeck,
                    _Xbox_Series::stem_words as xbox,
                };
                match self {
                    (Generic, LeftStickX) => Some(&[generic::_stick]),
                    (Generic, LeftStickY) => Some(&[generic::_stick]),
                    (Generic, LeftZ) => Some(&[generic::_trigger]),
                    (Generic, RightStickX) => Some(&[generic::_stick]),
                    (Generic, RightStickY) => Some(&[generic::_stick]),
                    (Generic, RightZ) => Some(&[generic::_trigger]),
                    (Generic, Other(_)) => None,

                    (Gamecube, LeftStickX) => Some(&[gamecube::_grip, gamecube::_stick]),
                    (Gamecube, LeftStickY) => Some(&[gamecube::_grip, gamecube::_stick]),
                    (Gamecube, LeftZ) => Some(&[gamecube::_l, gamecube::_trigger]),
                    (Gamecube, RightStickX) => Some(&[gamecube::_c, gamecube::_stick]),
                    (Gamecube, RightStickY) => Some(&[gamecube::_c, gamecube::_stick]),
                    (Gamecube, RightZ) => Some(&[gamecube::_r, gamecube::_trigger]),
                    (Gamecube, Other(_)) => None,

                    (Switch, LeftStickX) => Some(&[switch::_l, switch::_stick]),
                    (Switch, LeftStickY) => Some(&[switch::_l, switch::_stick]),
                    (Switch, LeftZ) => Some(&[switch::_zl]),
                    (Switch, RightStickX) => Some(&[switch::_r, switch::_stick]),
                    (Switch, RightStickY) => Some(&[switch::_r, switch::_stick]),
                    (Switch, RightZ) => Some(&[switch::_zr]),
                    (Switch, Other(_)) => None,

                    (Switch2, LeftStickX) => Some(&[switch2::_l, switch2::_stick]),
                    (Switch2, LeftStickY) => Some(&[switch2::_l, switch2::_stick]),
                    (Switch2, LeftZ) => Some(&[switch2::_zl]),
                    (Switch2, RightStickX) => Some(&[switch2::_r, switch2::_stick]),
                    (Switch2, RightStickY) => Some(&[switch2::_r, switch2::_stick]),
                    (Switch2, RightZ) => Some(&[switch2::_zr]),
                    (Switch2, Other(_)) => None,

                    (Wii, LeftStickX) => Some(&[wii::_l, wii::_stick]),
                    (Wii, LeftStickY) => Some(&[wii::_l, wii::_stick]),
                    (Wii, LeftZ) => Some(&[wii::_zl]),
                    (Wii, RightStickX) => Some(&[wii::_r, wii::_stick]),
                    (Wii, RightStickY) => Some(&[wii::_r, wii::_stick]),
                    (Wii, RightZ) => Some(&[wii::_zr]),
                    (Wii, Other(_)) => None,

                    (WiiU, LeftStickX) => Some(&[wiiu::_l, wiiu::_stick]),
                    (WiiU, LeftStickY) => Some(&[wiiu::_l, wiiu::_stick]),
                    (WiiU, LeftZ) => Some(&[wiiu::_zl]),
                    (WiiU, RightStickX) => Some(&[wiiu::_r, wiiu::_stick]),
                    (WiiU, RightStickY) => Some(&[wiiu::_r, wiiu::_stick]),
                    (WiiU, RightZ) => Some(&[wiiu::_zr]),
                    (WiiU, Other(_)) => None,

                    (PS3 | PS4 | PS5, LeftStickX) => Some(&[ps::_l, ps::_stick]),
                    (PS3 | PS4 | PS5, LeftStickY) => Some(&[ps::_l, ps::_stick]),
                    (PS3 | PS4 | PS5, LeftZ) => Some(&[ps::_l2]),
                    (PS3 | PS4 | PS5, RightStickX) => Some(&[ps::_r, ps::_stick]),
                    (PS3 | PS4 | PS5, RightStickY) => Some(&[ps::_r, ps::_stick]),
                    (PS3 | PS4 | PS5, RightZ) => Some(&[ps::_r2]),
                    (PS3 | PS4 | PS5, Other(_)) => None,

                    // TODO: the spinny thing on the playdate probably is mapped to some Other
                    (Playdate, LeftStickX) => None,
                    (Playdate, LeftStickY) => None,
                    (Playdate, LeftZ) => None,
                    (Playdate, RightStickX) => None,
                    (Playdate, RightStickY) => None,
                    (Playdate, RightZ) => None,
                    (Playdate, Other(_)) => None,

                    (SteamController, LeftStickX) => Some(&[steam::_l, steam::_stick]),
                    (SteamController, LeftStickY) => Some(&[steam::_l, steam::_stick]),
                    (SteamController, LeftZ) => Some(&[steam::_lt]),
                    (SteamController, RightStickX) => Some(&[steam::_l, steam::_stick]),
                    (SteamController, RightStickY) => Some(&[steam::_l, steam::_stick]),
                    (SteamController, RightZ) => Some(&[steam::_rt]),
                    (SteamController, Other(_)) => None,

                    (SteamDeck, LeftStickX) => Some(&[steamdeck::_l, steamdeck::_stick]),
                    (SteamDeck, LeftStickY) => Some(&[steamdeck::_l, steamdeck::_stick]),
                    (SteamDeck, LeftZ) => Some(&[steamdeck::_l2]),
                    (SteamDeck, RightStickX) => Some(&[steamdeck::_l, steamdeck::_stick]),
                    (SteamDeck, RightStickY) => Some(&[steamdeck::_l, steamdeck::_stick]),
                    (SteamDeck, RightZ) => Some(&[steamdeck::_r2]),
                    (SteamDeck, Other(_)) => None,

                    (Xbox, LeftStickX) => Some(&[xbox::_l, xbox::_stick]),
                    (Xbox, LeftStickY) => Some(&[xbox::_l, xbox::_stick]),
                    (Xbox, LeftZ) => Some(&[xbox::_lt]),
                    (Xbox, RightStickX) => Some(&[xbox::_l, xbox::_stick]),
                    (Xbox, RightStickY) => Some(&[xbox::_l, xbox::_stick]),
                    (Xbox, RightZ) => Some(&[xbox::_rt]),
                    (Xbox, Other(_)) => None,
                }
            }
            // TODO:
            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            Pack::Xelu => match self {
                _ => None,
            },
        }
    }
}
