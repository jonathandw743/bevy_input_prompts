use bevy_input::gamepad::GamepadButton::{self, *};

use crate::{FileConstraints, Pack, gamepad_brand::GamepadBrand};

impl FileConstraints for (&GamepadButton, GamepadBrand) {
    type Constraints<'c> = (&'c [&'c [usize]], &'c [usize]);
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                use crate::gamepad_brand::KenneyGamepadBrand::{self, *};
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::{_Generic::stem_words as generic, _Nintendo_Gamecube::stem_words as gamecube, _Nintendo_WiiU::stem_words as wiiu, _PlayStation_Series::stem_words as ps, _Playdate::stem_words as playdate, _Steam_Controller::stem_words as steam, _Steam_Deck::stem_words as steamdeck, _Xbox_Series::stem_words as xbox, stem_words::*, *};
                // TODO: most of playdate, start, select, mode on switch(2), xbox/ps elite controller Other(_) mappings
                let gamepad_brand = KenneyGamepadBrand::from(self.1);

                let input: &[&[usize]] = match self.0 {
                    South => match gamepad_brand {
                        Generic => &[generic::_button, generic::_circle],
                        Gamecube | SteamController | SteamDeck | XboxSeries | Xbox360 | XboxOne => &[_a],
                        Switch | Switch2 | Wii | WiiU => &[_b],
                        PS3 | PS4 | PS5 => &[ps::_cross],
                        Playdate => &[playdate::_a],
                    },
                    East => match gamepad_brand {
                        Generic => &[generic::_button, generic::_circle],
                        Gamecube | SteamController | SteamDeck | XboxSeries | Xbox360 | XboxOne => &[_b],
                        Switch | Switch2 | Wii | WiiU => &[_a],
                        PS3 | PS4 | PS5 => &[ps::_circle],
                        Playdate => &[playdate::_b],
                    },
                    North => match gamepad_brand {
                        Generic => &[generic::_button, generic::_circle],
                        Gamecube | SteamController | SteamDeck | XboxSeries | Xbox360 | XboxOne => &[_y],
                        Switch | Switch2 | Wii | WiiU => &[_x],
                        PS3 | PS4 | PS5 => &[ps::_triangle],
                        Playdate => &[&[]],
                    },
                    West => match gamepad_brand {
                        Generic => &[generic::_button, generic::_circle],
                        Gamecube | SteamController | SteamDeck | XboxSeries | Xbox360 | XboxOne => &[_x],
                        Switch | Switch2 | Wii | WiiU => &[_y],
                        PS3 | PS4 | PS5 => &[ps::_square],
                        Playdate => &[&[]],
                    },
                    LeftTrigger => match gamepad_brand {
                        Generic => &[generic::_trigger],
                        Gamecube => &[_trigger, _l],
                        Switch | Switch2 | WiiU => &[_l],
                        PS3 | PS4 | PS5 | SteamDeck => &[_l1],
                        SteamController | XboxSeries | Xbox360 | XboxOne => &[_lb],
                        Wii | Playdate => &[&[]],
                    },
                    LeftTrigger2 => match gamepad_brand {
                        Generic => &[generic::_trigger],
                        Switch | Switch2 | WiiU => &[_zl],
                        PS3 | PS4 | PS5 | SteamDeck => &[_l2],
                        SteamController | XboxSeries | Xbox360 | XboxOne => &[_lt],
                        Gamecube | Wii | Playdate => &[&[]],
                    },
                    RightTrigger => match gamepad_brand {
                        Generic => &[generic::_trigger],
                        Gamecube => &[_trigger, _r],
                        Switch | Switch2 | WiiU => &[_r],
                        PS3 | PS4 | PS5 | SteamDeck => &[_r1],
                        SteamController | XboxSeries | Xbox360 | XboxOne => &[_rb],
                        Wii | Playdate => &[&[]],
                    },
                    RightTrigger2 => match gamepad_brand {
                        Generic => &[generic::_trigger],
                        Switch | Switch2 | WiiU => &[_zr],
                        PS3 | PS4 | PS5 | SteamDeck => &[_r2],
                        SteamController | XboxSeries | Xbox360 | XboxOne => &[_rt],
                        Gamecube | Wii | Playdate => &[&[]],
                    },
                    LeftThumb => match gamepad_brand {
                        Generic => &[generic::_press, generic::_stick],
                        Gamecube => &[gamecube::_grip, gamecube::_stick],
                        Switch | Switch2 | PS3 | PS4 | PS5 | SteamDeck | Xbox360 | XboxOne | XboxSeries | SteamController => &[_press, _l, _stick],
                        Wii | WiiU | Playdate => &[&[]],
                    },
                    RightThumb => match gamepad_brand {
                        Generic => &[generic::_press, generic::_stick],
                        Gamecube => &[gamecube::_c, gamecube::_stick],
                        Switch | Switch2 | PS3 | PS4 | PS5 | SteamDeck | Xbox360 | XboxOne | XboxSeries => &[_press, _r, _stick],
                        SteamController => &[steam::_pad],
                        Wii | WiiU | Playdate => &[&[]],
                    },
                    DPadDown => match gamepad_brand {
                        Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5 | Playdate | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries => &[_down, _dpad],
                        Generic => &[&[]],
                    },
                    DPadLeft => match gamepad_brand {
                        Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5 | Playdate | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries => &[_left, _dpad],
                        Generic => &[&[]],
                    },
                    DPadRight => match gamepad_brand {
                        Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5 | Playdate | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries => &[_right, _dpad],
                        Generic => &[&[]],
                    },
                    DPadUp => match gamepad_brand {
                        Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5 | Playdate | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries => &[_up, _dpad],
                        Generic => &[&[]],
                    },
                    Select => match gamepad_brand {
                        WiiU => &[wiiu::_minus],
                        PS3 | PS4 | PS5 => &[ps::_select],
                        Xbox360 | SteamController => &[_back],
                        XboxSeries | XboxOne => &[xbox::_view],
                        SteamDeck => &[steamdeck::_view],
                        Playdate => &[playdate::_menu],
                        Generic | Gamecube | Switch | Switch2 | Wii => &[&[]],
                    },
                    Start => match gamepad_brand {
                        WiiU => &[wiiu::_plus],
                        Playdate => &[],
                        SteamDeck => &[steamdeck::_options],
                        Gamecube | Xbox360 | SteamController | PS3 | PS4 | PS5 => &[_start],
                        XboxOne | XboxSeries => &[_menu],
                        Generic | Switch | Switch2 | Wii => &[&[]],
                    },
                    Mode => match gamepad_brand {
                        WiiU => &[wiiu::_home],
                        PS3 | PS4 | PS5 => &[ps::_touchpad],
                        XboxSeries | Xbox360 | XboxOne => &[xbox::_guide],
                        SteamController | SteamDeck | Playdate | Wii | Switch2 | Switch | Gamecube | Generic => &[&[]],
                    },
                    C => match gamepad_brand {
                        Generic | Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5 | Playdate | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries => &[&[]],
                    },
                    Z => match gamepad_brand {
                        Gamecube => &[_z],
                        Generic | Wii | WiiU | Switch | Switch2 | Playdate | SteamController | SteamDeck | PS3 | PS4 | PS5 | Xbox360 | XboxOne | XboxSeries => &[&[]],
                    },
                    Other(_) => match gamepad_brand {
                        Generic | Gamecube | Switch | Switch2 | Playdate | Wii | WiiU | PS3 | PS4 | PS5 | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries => &[&[]],
                    },
                };
                (
                    input,
                    match KenneyGamepadBrand::from(self.1) {
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
            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            Pack::Xelu => {
                use crate::gamepad_brand::XeluGamepadBrand::{self, *};
                use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts::{stem_words::*, *};
                let gamepad_brand = XeluGamepadBrand::from(self.1);
                let input: &[&[usize]] = match self.0 {
                    South => match gamepad_brand {
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController | Oculus | Xbox360 | XboxOne => &[_A],
                        Switch | Wii | WiiU => &[_B],
                        PS3 | PS4 | PS5 | PSMove | PSVita => &[_Cross],
                        Vive => &[&[]],
                    },
                    East => match gamepad_brand {
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController | Oculus | Xbox360 | XboxOne => &[_B],
                        Switch | Wii | WiiU => &[_A],
                        PS3 | PS4 | PS5 | PSMove | PSVita => &[_Circle],
                        Vive => &[&[]],
                    },
                    North => match gamepad_brand {
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController | Oculus | Xbox360 | XboxOne => &[_Y],
                        Switch | Wii | WiiU => &[_X],
                        PS3 | PS4 | PS5 | PSMove | PSVita => &[_Triangle],
                        Vive => &[&[]],
                    },
                    West => match gamepad_brand {
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController | Oculus | Xbox360 | XboxOne => &[_X],
                        Switch | Wii | WiiU => &[_Y],
                        PS3 | PS4 | PS5 | PSMove | PSVita => &[_Square],
                        Vive => &[&[]],
                    },
                    LeftTrigger => match gamepad_brand {
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController | Oculus | Xbox360 | XboxOne | Switch => &[_LB],
                        PS3 | PS4 | PS5 | PSMove | PSVita => &[_L1],
                        WiiU => &[_L],
                        Wii | Vive => &[&[]],
                    },
                    LeftTrigger2 => match gamepad_brand {
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController | Oculus | Xbox360 | XboxOne | Switch | Vive => &[_LT],
                        WiiU => &[_ZL],
                        PS3 | PS4 | PS5 | PSMove | PSVita => &[_L2],
                        Wii => &[_Z],
                    },
                    RightTrigger => match gamepad_brand {
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController | Oculus | Xbox360 | XboxOne | Switch => &[_RB],
                        PS3 | PS4 | PS5 | PSMove | PSVita => &[_R1],
                        WiiU => &[_R],
                        Wii | Vive => &[&[]],
                    },
                    RightTrigger2 => match gamepad_brand {
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController | Oculus | Xbox360 | XboxOne | Switch => &[_RT],
                        WiiU => &[_ZR],
                        PS3 | PS4 | PS5 | PSMove | PSVita => &[_R2],
                        Wii | Vive => &[&[]],
                    },
                    C => match gamepad_brand {
                        Xbox360 | XboxOne | XboxSeries | SteamController | SteamDeck | AmazonLuna | GoogleStadia | Ouya | Oculus | Vive | Switch | Wii | WiiU | PS3 | PS4 | PS5 | PSMove | PSVita => &[&[]],
                    },
                    Z => match gamepad_brand {
                        Xbox360 | XboxOne | XboxSeries | SteamController | SteamDeck | AmazonLuna | GoogleStadia | Ouya | Oculus | Vive | Switch | Wii | WiiU | PS3 | PS4 | PS5 | PSMove | PSVita => &[&[]],
                    },
                    Select => match gamepad_brand {
                        XboxSeries => &[_View],
                        PS4 | PS5 => &[_Share],
                        SteamDeck => &[_Inventory],
                        AmazonLuna => &[_Microphone],
                        GoogleStadia => &[_Dots],
                        Ouya => &[_Touch],
                        PS3 | PSMove | PSVita => &[_Select],
                        SteamController | Xbox360 => &[_Back],
                        Wii | WiiU | Switch | Oculus => &[_Minus],
                        Vive => &[_System],
                        XboxOne => &[_Windows],
                    },
                    Start => match gamepad_brand {
                        Wii | WiiU | Switch | Oculus => &[_Plus],
                        XboxSeries | XboxOne | SteamDeck | AmazonLuna | GoogleStadia | Ouya => &[_Menu],
                        PS4 | PS5 => &[_Options],
                        PS3 | PSMove | PSVita | SteamController | Xbox360 => &[_Start],
                        Vive => &[_Menu],
                    },
                    Mode => match gamepad_brand {
                        XboxSeries | AmazonLuna | XboxOne => &[&[]],
                        PSVita | PS4 | PS5 => &[_Touch, _Pad],
                        GoogleStadia => &[_Assistant],
                        SteamDeck | Vive => &[_System],
                        Switch | Wii | WiiU | PS3 | SteamController | Xbox360 | Ouya | PSMove | Oculus => &[&[]],
                    },
                    LeftThumb => match gamepad_brand {
                        XboxSeries | Switch | SteamDeck | PS5 | AmazonLuna | PS3 | PS4 | Xbox360 | XboxOne => &[_Left_0, _Stick, _Click],
                        GoogleStadia | Ouya | PSMove | PSVita | Oculus | WiiU => &[_Left_0, _Stick],
                        SteamController => &[_Stick],
                        Vive | Wii => &[&[]],
                    },
                    RightThumb => match gamepad_brand {
                        XboxSeries | Switch | SteamDeck | PS5 | AmazonLuna | PS3 | PS4 | Xbox360 | XboxOne => &[_Right_0, _Stick, _Click],
                        GoogleStadia | Ouya | PSMove | PSVita | Oculus | WiiU => &[_Right_0, _Stick],
                        SteamController => &[_Track, _Right_0, _Center],
                        Vive | Wii => &[&[]],
                    },
                    DPadUp => match gamepad_brand {
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus | Vive | Wii | WiiU | Switch => &[_Up, _Dpad],
                    },

                    DPadDown => match gamepad_brand {
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus | Vive | Wii | WiiU | Switch => &[_Down, _Dpad],
                    },

                    DPadLeft => match gamepad_brand {
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus | Vive | Wii | WiiU | Switch => &[_Left_0, _Dpad],
                    },

                    DPadRight => match gamepad_brand {
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus | Vive | Wii | WiiU | Switch => &[_Right_0, _Dpad],
                    },

                    Other(_) => match gamepad_brand {
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus | Vive | Wii | WiiU | Switch => &[&[]],
                    },
                };
                (
                    input,
                    match gamepad_brand {
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
