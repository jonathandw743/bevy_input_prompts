use Direction::*;
use bevy_input::gamepad::GamepadAxis::{self, *};

use crate::{
    FileConstraints, Pack,
    gamepad_brand::{GamepadBrand, XeluGamepadBrand},
};

#[derive(Clone, Copy)]
pub enum Direction {
    Positive,
    Negative,
    Bidirectional,
}

impl FileConstraints for (&GamepadAxis, GamepadBrand, &Option<Direction>) {
    type Constraints<'c> = ((&'c [&'c [usize]], &'c [usize]), &'c [&'c [usize]]);
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        ((self.0, self.1).file_constriants(pack), (self.0, self.2).file_constriants(pack))
    }
}

impl FileConstraints for (&GamepadAxis, &Option<Direction>) {
    type Constraints<'c> = &'c [&'c [usize]];
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::stem_words::*;
                // this just gives the directionality constraints
                // because the brand is required for the axis constraints
                match self.0 {
                    LeftStickX | RightStickX => match self.1 {
                        // no direction given means unconstrained
                        None => &[],
                        Some(Positive) => &[_right],
                        Some(Negative) => &[_left],
                        Some(Bidirectional) => &[_horizontal],
                    },
                    LeftStickY | RightStickY => match self.1 {
                        // no direction given means unconstrained
                        None => &[],
                        Some(Positive) => &[_up],
                        Some(Negative) => &[_down],
                        Some(Bidirectional) => &[_vertical],
                    },
                    // for axes where directionality does not apply, directions have no meaning so unconstrained
                    LeftZ => &[],
                    // for axes where directionality does not apply, directions have no meaning so unconstrained
                    RightZ => &[],
                    // for axes where directionality does not apply, directions have no meaning so unconstrained
                    Other(_) => &[],
                }
            }
            // TODO:
            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            Pack::Xelu => {
                match self {
                    // there are no directional axis inputs in the xelu pack, so leave this unconstrained
                    _ => &[],
                }
            }
        }
    }
}

impl FileConstraints for (&GamepadAxis, GamepadBrand) {
    type Constraints<'c> = (&'c [&'c [usize]], &'c [usize]);
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                use crate::gamepad_brand::KenneyGamepadBrand::{self, *};
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::{_Generic::stem_words as generic, _Nintendo_Gamecube::stem_words as gamecube, stem_words::*, *};

                let gamepad_brand = KenneyGamepadBrand::from(self.1);
                let input: &[&[usize]] = match self.0 {
                    LeftStickX | LeftStickY => match gamepad_brand {
                        Generic => &[generic::_stick],
                        Gamecube => &[gamecube::_grip, gamecube::_stick],
                        Switch | Switch2 | PS3 | PS4 | PS5 | Xbox360 | XboxOne | XboxSeries | Wii | WiiU | SteamController | SteamDeck => &[_l, _stick],
                        Playdate => &[&[]],
                    },
                    RightStickX | RightStickY => match gamepad_brand {
                        Generic => &[generic::_stick],
                        Gamecube => &[gamecube::_c, gamecube::_stick],
                        Switch | Switch2 | PS3 | PS4 | PS5 | Xbox360 | XboxOne | XboxSeries | Wii | WiiU | SteamController | SteamDeck => &[_r, _stick],
                        Playdate => &[&[]],
                    },
                    LeftZ => match gamepad_brand {
                        Generic => &[generic::_trigger],
                        Gamecube => &[gamecube::_l, gamecube::_trigger],
                        Switch | Switch2 | WiiU => &[_zl],
                        Wii => &[_z],
                        PS3 | PS4 | PS5 | SteamDeck => &[_l2],
                        SteamController | Xbox360 | XboxOne | XboxSeries => &[_lt],
                        Playdate => &[&[]],
                    },
                    RightZ => match gamepad_brand {
                        Generic => &[generic::_trigger],
                        Gamecube => &[gamecube::_r, gamecube::_trigger],
                        Switch | Switch2 | WiiU => &[_zr],
                        Wii => &[&[]],
                        PS3 | PS4 | PS5 | SteamDeck => &[_r2],
                        SteamController | Xbox360 | XboxOne | XboxSeries => &[_rt],
                        Playdate => &[&[]],
                    },
                    Other(_) => match gamepad_brand {
                        Generic => &[&[]],
                        Gamecube => &[&[]],
                        Switch | Switch2 | PS3 | PS4 | PS5 | Xbox360 | XboxOne | XboxSeries | Wii | WiiU | SteamController | SteamDeck => &[&[]],
                        Playdate => &[&[]],
                    },
                };
                (
                    input,
                    match gamepad_brand {
                        Generic => _Generic::DIR,
                        Switch => _Nintendo_Switch::DIR,
                        Wii => _Nintendo_Wii::DIR,
                        WiiU => _Nintendo_WiiU::DIR,
                        PS3 | PS4 | PS5 => _PlayStation_Series::DIR,
                        SteamController => _Steam_Controller::DIR,
                        SteamDeck => _Steam_Deck::DIR,
                        Xbox360 | XboxOne | XboxSeries => _Xbox_Series::DIR,
                        Gamecube => _Nintendo_Gamecube::DIR,
                        Switch2 => _Nintendo_Switch_2::DIR,
                        Playdate => _Playdate::DIR,
                    },
                )
            }
            // TODO:
            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            Pack::Xelu => {
                use crate::gamepad_brand::XeluGamepadBrand::*;
                use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts::{stem_words::*, *};

                let gamepad_brand = XeluGamepadBrand::from(self.1);
                let input: &[&[usize]] = match self.0 {
                    LeftStickX | LeftStickY => match gamepad_brand {
                        AmazonLuna | GoogleStadia | Ouya | Oculus | Switch | WiiU | PS3 | PS4 | PS5 | PSMove | PSVita | SteamDeck | XboxSeries | Xbox360 | XboxOne => &[_Stick, _Left_0],
                        SteamController | Wii => &[_Stick],
                        Vive => &[&[]],
                    },
                    RightStickX | RightStickY => match gamepad_brand {
                        AmazonLuna | GoogleStadia | Ouya | Oculus | Switch | WiiU | PS3 | PS4 | PS5 | PSVita | SteamDeck | XboxSeries | Xbox360 | XboxOne => &[_Stick, _Right_0],
                        PSMove | Vive | SteamController | Wii => &[&[]],
                    },
                    LeftZ => match gamepad_brand {
                        Xbox360 | XboxOne | XboxSeries | Switch | AmazonLuna | PSMove | SteamController | Oculus | Vive => &[_LT],
                        PS5 | SteamDeck | GoogleStadia | Ouya | PS3 | PS4 => &[_L2],
                        WiiU => &[_ZL],
                        Wii => &[_Z],
                        PSVita => &[&[]],
                    },
                    RightZ => match gamepad_brand {
                        Xbox360 | XboxOne | XboxSeries | Switch | AmazonLuna | PSMove | SteamController | Oculus | Vive => &[_RT],
                        PS5 | SteamDeck | GoogleStadia | Ouya | PS3 | PS4 => &[_R2],
                        WiiU => &[_ZR],
                        Wii => &[&[]],
                        PSVita => &[&[]],
                    },
                    Other(_) => match gamepad_brand {
                        AmazonLuna | PSVita | PSMove | PS5 | PS4 | PS3 | WiiU | Wii | Switch | Vive | Oculus | Ouya | GoogleStadia | SteamController | SteamDeck | XboxOne | Xbox360 | XboxSeries => &[&[]],
                    },
                };
                (
                    input,
                    match &gamepad_brand {
                        Switch => _Switch::DIR,
                        Wii => _Others::_Wii::DIR,
                        WiiU => _Others::_WiiU::DIR,
                        PS3 => _Others::_PS3::DIR,
                        PS4 => _Others::_PS4::DIR,
                        PS5 => _PS5::DIR,
                        SteamController => _Others::_Steam::DIR,
                        SteamDeck => _Steam_Deck::DIR,
                        Xbox360 => _Others::_Xbox_360::DIR,
                        XboxOne => _Others::_Xbox_One::DIR,
                        XboxSeries => _Xbox_Series::DIR,
                        AmazonLuna => _Others::_Amazon_Luna::DIR,
                        GoogleStadia => _Others::_Google_Stadia::DIR,
                        Ouya => _Others::_Ouya::DIR,
                        PSMove => _Others::_PS_Move::DIR,
                        PSVita => _Others::_PS_Vita::DIR,
                        Oculus => _Others::_VR::_Oculus::DIR,
                        Vive => _Others::_VR::_Vive::DIR,
                    },
                )
            }
        }
    }
}
