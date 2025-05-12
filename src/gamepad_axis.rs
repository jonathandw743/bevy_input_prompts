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

impl FileConstraints for (GamepadBrand, &GamepadAxis, &Option<Direction>) {
    type Constraints<'c> = ((&'c [&'c [usize]], &'c [usize]), &'c [&'c [usize]]);
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        (
            (self.0, self.1).file_constriants(pack),
            (self.1, self.2).file_constriants(pack),
        )
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
                match self {
                    // no direction given means unconstrained
                    (LeftStickX | RightStickX, None) => &[],
                    (LeftStickX | RightStickX, Some(Positive)) => &[_right],
                    (LeftStickX | RightStickX, Some(Negative)) => &[_left],
                    (LeftStickX | RightStickX, Some(Bidirectional)) => &[_horizontal],
                    // for axes where directionality does not apply, directions have no meaning so unconstrained
                    (LeftZ, _) => &[],
                    // no direction given means unconstrained
                    (LeftStickY | RightStickY, None) => &[],
                    (LeftStickY | RightStickY, Some(Positive)) => &[_up],
                    (LeftStickY | RightStickY, Some(Negative)) => &[_down],
                    (LeftStickY | RightStickY, Some(Bidirectional)) => &[_vertical],
                    // for axes where directionality does not apply, directions have no meaning so unconstrained
                    (RightZ, _) => &[],
                    // for axes where directionality does not apply, directions have no meaning so unconstrained
                    (Other(_), _) => &[],
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

impl FileConstraints for (GamepadBrand, &GamepadAxis) {
    type Constraints<'c> = (&'c [&'c [usize]], &'c [usize]);
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                use crate::gamepad_brand::KenneyGamepadBrand::{self, *};
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::{
                    _Generic::stem_words as generic, _Nintendo_Gamecube::stem_words as gamecube,
                    stem_words::*, *,
                };
                let gamepad_brand = KenneyGamepadBrand::from(self.0);
                let input: &[&[usize]] = match (&gamepad_brand, self.1) {
                    (Generic, LeftStickX | LeftStickY | RightStickX | RightStickY) => {
                        &[generic::_stick]
                    }
                    (Generic, LeftZ | RightZ) => &[generic::_trigger],
                    (Generic, Other(_)) => &[&[]],

                    (Gamecube, LeftStickX | LeftStickY) => &[gamecube::_grip, gamecube::_stick],
                    (Gamecube, LeftZ) => &[gamecube::_l, gamecube::_trigger],
                    (Gamecube, RightStickX | RightStickY) => &[gamecube::_c, gamecube::_stick],
                    (Gamecube, RightZ) => &[gamecube::_r, gamecube::_trigger],
                    (Gamecube, Other(_)) => &[&[]],

                    (Switch | Switch2 | WiiU, LeftZ) => &[_zl],
                    (Switch | Switch2 | WiiU, RightZ) => &[_zr],
                    (Wii, LeftZ) => &[_z],
                    (Wii, RightZ) => &[&[]],


                    (
                        Switch | Switch2 | PS3 | PS4 | PS5 | Xbox360 | XboxOne | XboxSeries | Wii
                        | WiiU | SteamController | SteamDeck,
                        LeftStickX | LeftStickY,
                    ) => &[_l, _stick],
                    (
                        Switch | Switch2 | PS3 | PS4 | PS5 | Xbox360 | XboxOne | XboxSeries | Wii
                        | WiiU | SteamController | SteamDeck,
                        RightStickX | RightStickY,
                    ) => &[_r, _stick],

                    (
                        Switch | Switch2 | PS3 | PS4 | PS5 | Xbox360 | XboxOne | XboxSeries | Wii
                        | WiiU | SteamController | SteamDeck,
                        Other(_),
                    ) => &[&[]],

                    (PS3 | PS4 | PS5 | SteamDeck, LeftZ) => &[_l2],
                    (PS3 | PS4 | PS5 | SteamDeck, RightZ) => &[_r2],

                    // TODO: the spinny thing on the playdate probably is mapped to some Other
                    (Playdate, LeftStickX) => &[&[]],
                    (Playdate, LeftStickY) => &[&[]],
                    (Playdate, LeftZ) => &[&[]],
                    (Playdate, RightStickX) => &[&[]],
                    (Playdate, RightStickY) => &[&[]],
                    (Playdate, RightZ) => &[&[]],
                    (Playdate, Other(_)) => &[&[]],

                    (SteamController | Xbox360 | XboxOne | XboxSeries, LeftZ) => &[_lt],
                    (SteamController | Xbox360 | XboxOne | XboxSeries, RightZ) => &[_rt],
                };
                (
                    input,
                    match &gamepad_brand {
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
                use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts::{
                    stem_words::*, *
                };
                use crate::gamepad_brand::XeluGamepadBrand::*;
                let gamepad_brand = XeluGamepadBrand::from(self.0);
                let input: &[&[usize]] = match (&gamepad_brand, self.1) {
                    (
                        AmazonLuna | GoogleStadia | Ouya | Oculus | Switch | WiiU | PS3 | PS4
                        | PS5 | PSMove | PSVita | SteamDeck | XboxSeries | Xbox360 | XboxOne,
                        LeftStickX | LeftStickY,
                    ) => &[_Stick, _Left_0],
                    (SteamController | Wii, LeftStickX | LeftStickY) => &[_Stick],
                    (Vive, LeftStickX | LeftStickY) => &[&[]],

                    (
                        AmazonLuna | GoogleStadia | Ouya | Oculus | Switch | WiiU | PS3 | PS4
                        | PS5 | PSVita | SteamDeck | XboxSeries | Xbox360 | XboxOne,
                        RightStickX | RightStickY,
                    ) => &[_Stick, _Right_0],
                    (PSMove | Vive | SteamController | Wii, RightStickX | RightStickY) => &[&[]],

                    (
                        Xbox360 | XboxOne | XboxSeries | Switch | AmazonLuna | PSMove
                        | SteamController | Oculus | Vive,
                        LeftZ,
                    ) => &[_LT],
                    (
                        Xbox360 | XboxOne | XboxSeries | Switch | AmazonLuna | PSMove
                        | SteamController | Oculus | Vive,
                        RightZ,
                    ) => &[_RT],

                    (PS5 | SteamDeck | GoogleStadia | Ouya | PS3 | PS4, LeftZ) => &[_L2],
                    (PS5 | SteamDeck | GoogleStadia | Ouya | PS3 | PS4, RightZ) => &[_R2],

                    (Wii, LeftZ) => &[_Z],
                    (Wii, RightZ) => &[&[]],

                    (WiiU, LeftZ) => &[_ZL],
                    (WiiU, RightZ) => &[_ZR],

                    (PSVita, LeftZ) => &[&[]],
                    (PSVita, RightZ) => &[&[]],

                    (
                        AmazonLuna | PSVita | PSMove | PS5 | PS4 | PS3 | WiiU | Wii | Switch | Vive
                        | Oculus | Ouya | GoogleStadia | SteamController | SteamDeck | XboxOne
                        | Xbox360 | XboxSeries,
                        Other(_),
                    ) => &[&[]],
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
