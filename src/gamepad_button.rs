use bevy_input::gamepad::GamepadButton::{self, *};

use crate::{FileConstraints, Pack, gamepad_brand::GamepadBrand};

impl FileConstraints for (GamepadBrand, &GamepadButton) {
    type Constraints<'c> = (&'c [&'c [usize]], &'c [usize]);
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            Pack::Kenney => {
                use crate::gamepad_brand::KenneyGamepadBrand::{self, *};
                use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::{
                    _Generic::stem_words as generic, _Nintendo_Gamecube::stem_words as gamecube,
                    _Nintendo_WiiU::stem_words as wiiu, _PlayStation_Series::stem_words as ps,
                    _Playdate::stem_words as playdate, _Steam_Controller::stem_words as steam,
                    _Steam_Deck::stem_words as steamdeck, _Xbox_Series::stem_words as xbox,
                    stem_words::*, *,
                };
                // TODO: most of playdate, start, select, mode on switch(2), xbox/ps elite controller Other(_) mappings
                let input: &[&[usize]] = match (KenneyGamepadBrand::from(self.0), self.1) {
                    (Generic, South) => &[generic::_button, generic::_circle],
                    (
                        Gamecube | SteamController | SteamDeck | XboxSeries | Xbox360 | XboxOne,
                        South,
                    ) => &[_a],
                    (Switch | Switch2 | Wii | WiiU, South) => &[_b],
                    (PS3 | PS4 | PS5, South) => &[ps::_cross],
                    (Playdate, South) => &[playdate::_a],

                    (Generic, East) => &[generic::_button, generic::_circle],
                    (
                        Gamecube | SteamController | SteamDeck | XboxSeries | Xbox360 | XboxOne,
                        East,
                    ) => &[_b],
                    (Switch | Switch2 | Wii | WiiU, East) => &[_a],
                    (PS3 | PS4 | PS5, East) => &[ps::_circle],
                    (Playdate, East) => &[playdate::_b],

                    (Generic, North) => &[generic::_button, generic::_circle],
                    (
                        Gamecube | SteamController | SteamDeck | XboxSeries | Xbox360 | XboxOne,
                        North,
                    ) => &[_y],
                    (Switch | Switch2 | Wii | WiiU, North) => &[_x],
                    (PS3 | PS4 | PS5, North) => &[ps::_triangle],
                    (Playdate, North) => &[&[]],

                    (Generic, West) => &[generic::_button, generic::_circle],
                    (
                        Gamecube | SteamController | SteamDeck | XboxSeries | Xbox360 | XboxOne,
                        West,
                    ) => &[_x],
                    (Switch | Switch2 | Wii | WiiU, West) => &[_y],
                    (PS3 | PS4 | PS5, West) => &[ps::_square],
                    (Playdate, West) => &[&[]],

                    (Generic, LeftTrigger) => &[generic::_trigger],
                    (Gamecube, LeftTrigger) => &[_trigger, _l],
                    (Switch | Switch2 | WiiU, LeftTrigger) => &[_l],
                    (PS3 | PS4 | PS5 | SteamDeck, LeftTrigger) => &[_l1],
                    (SteamController | XboxSeries | Xbox360 | XboxOne, LeftTrigger) => &[_lb],
                    (Wii | Playdate, LeftTrigger) => &[&[]],

                    (Generic, LeftTrigger2) => &[generic::_trigger],
                    (Switch | Switch2 | WiiU, LeftTrigger2) => &[_zl],
                    (PS3 | PS4 | PS5 | SteamDeck, LeftTrigger2) => &[_l2],
                    (SteamController | XboxSeries | Xbox360 | XboxOne, LeftTrigger2) => &[_lt],
                    (Gamecube | Wii | Playdate, LeftTrigger2) => &[&[]],

                    (Generic, RightTrigger) => &[generic::_trigger],
                    (Gamecube, RightTrigger) => &[_trigger, _r],
                    (Switch | Switch2 | WiiU, RightTrigger) => &[_r],
                    (PS3 | PS4 | PS5 | SteamDeck, RightTrigger) => &[_r1],
                    (SteamController | XboxSeries | Xbox360 | XboxOne, RightTrigger) => &[_rb],
                    (Wii | Playdate, RightTrigger) => &[&[]],

                    (Generic, RightTrigger2) => &[generic::_trigger],
                    (Switch | Switch2 | WiiU, RightTrigger2) => &[_zr],
                    (PS3 | PS4 | PS5 | SteamDeck, RightTrigger2) => &[_r2],
                    (SteamController | XboxSeries | Xbox360 | XboxOne, RightTrigger2) => &[_rt],
                    (Gamecube | Wii | Playdate, RightTrigger2) => &[&[]],

                    (Generic, LeftThumb) => &[generic::_press, generic::_stick],
                    (Gamecube, LeftThumb) => &[gamecube::_grip, gamecube::_stick],
                    (
                        Switch | Switch2 | PS3 | PS4 | PS5 | SteamDeck | Xbox360 | XboxOne
                        | XboxSeries | SteamController,
                        LeftThumb,
                    ) => &[_press, _l, _stick],
                    (Wii | WiiU | Playdate, LeftThumb) => &[&[]],

                    (Generic, RightThumb) => &[generic::_press, generic::_stick],
                    (Gamecube, RightThumb) => &[gamecube::_c, gamecube::_stick],
                    (
                        Switch | Switch2 | PS3 | PS4 | PS5 | SteamDeck | Xbox360 | XboxOne
                        | XboxSeries,
                        RightThumb,
                    ) => &[_press, _r, _stick],
                    (SteamController, RightThumb) => &[steam::_pad],
                    (Wii | WiiU | Playdate, RightThumb) => &[&[]],

                    (
                        Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5 | Playdate
                        | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries,
                        DPadDown,
                    ) => &[_down, _dpad],
                    (Generic, DPadDown) => &[&[]],

                    (
                        Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5 | Playdate
                        | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries,
                        DPadLeft,
                    ) => &[_left, _dpad],
                    (Generic, DPadLeft) => &[&[]],

                    (
                        Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5 | Playdate
                        | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries,
                        DPadRight,
                    ) => &[_right, _dpad],
                    (Generic, DPadRight) => &[&[]],

                    (
                        Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5 | Playdate
                        | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries,
                        DPadUp,
                    ) => &[_up, _dpad],
                    (Generic, DPadUp) => &[&[]],

                    (WiiU, Select) => &[wiiu::_minus],
                    (PS3 | PS4 | PS5, Select) => &[ps::_select],
                    (SteamController, Select) => &[steam::_back],
                    (XboxSeries | Xbox360 | XboxOne, Select) => &[xbox::_menu],
                    (SteamDeck, Select) => &[steamdeck::_view],
                    (Playdate, Select) => &[playdate::_menu],
                    (Generic | Gamecube | Switch | Switch2 | Wii, Select) => &[&[]],

                    (WiiU, Start) => &[wiiu::_plus],
                    (Playdate, Start) => &[],
                    (SteamDeck, Start) => &[steamdeck::_options],
                    (
                        Gamecube | Xbox360 | XboxOne | XboxSeries | SteamController | PS3 | PS4
                        | PS5,
                        Start,
                    ) => &[_start],
                    (Generic | Switch | Switch2 | Wii, Start) => &[&[]],

                    (WiiU, Mode) => &[wiiu::_home],
                    (PS3 | PS4 | PS5, Mode) => &[ps::_touchpad],
                    (XboxSeries | Xbox360 | XboxOne, Mode) => &[xbox::_guide],
                    (
                        SteamController | SteamDeck | Playdate | Wii | Switch2 | Switch | Gamecube
                        | Generic,
                        Mode,
                    ) => &[&[]],

                    (
                        Generic | Gamecube | Switch | Switch2 | Wii | WiiU | PS3 | PS4 | PS5
                        | Playdate | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries,
                        C,
                    ) => &[&[]],

                    (Gamecube, Z) => &[_z],
                    (
                        Generic | Wii | WiiU | Switch | Switch2 | Playdate | SteamController
                        | SteamDeck | PS3 | PS4 | PS5 | Xbox360 | XboxOne | XboxSeries,
                        Z,
                    ) => &[&[]],

                    (
                        Generic | Gamecube | Switch | Switch2 | Playdate | Wii | WiiU | PS3 | PS4
                        | PS5 | SteamController | SteamDeck | Xbox360 | XboxOne | XboxSeries,
                        Other(_),
                    ) => &[&[]],
                };
                (
                    input,
                    match KenneyGamepadBrand::from(self.0) {
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
                use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts::{
                    stem_words::*, *,
                };
                let input: &[&[usize]] = match (XeluGamepadBrand::from(self.0), self.1) {
                    (
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController
                        | Oculus | Xbox360 | XboxOne,
                        South,
                    ) => &[_A],
                    (Switch | Wii | WiiU, South) => &[_B],
                    (PS3 | PS4 | PS5 | PSMove | PSVita, South) => &[_Cross],
                    (Vive, South) => &[&[]],

                    (
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController
                        | Oculus | Xbox360 | XboxOne,
                        East,
                    ) => &[_B],
                    (Switch | Wii | WiiU, East) => &[_A],
                    (PS3 | PS4 | PS5 | PSMove | PSVita, East) => &[_Circle],
                    (Vive, East) => &[&[]],

                    (
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController
                        | Oculus | Xbox360 | XboxOne,
                        North,
                    ) => &[_Y],
                    (Switch | Wii | WiiU, North) => &[_X],
                    (PS3 | PS4 | PS5 | PSMove | PSVita, North) => &[_Triangle],
                    (Vive, North) => &[&[]],

                    (
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController
                        | Oculus | Xbox360 | XboxOne,
                        West,
                    ) => &[_X],
                    (Switch | Wii | WiiU, West) => &[_Y],
                    (PS3 | PS4 | PS5 | PSMove | PSVita, West) => &[_Square],
                    (Vive, West) => &[&[]],

                    (
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController
                        | Oculus | Xbox360 | XboxOne | Switch,
                        LeftTrigger,
                    ) => &[_LB],
                    (PS3 | PS4 | PS5 | PSMove | PSVita, LeftTrigger) => &[_L1],
                    (WiiU, LeftTrigger) => &[_L],
                    (Wii | Vive, LeftTrigger) => &[&[]],
                    
                    (
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController
                        | Oculus | Xbox360 | XboxOne | Switch | Vive,
                        LeftTrigger2,
                    ) => &[_LT],
                    (WiiU, LeftTrigger2) => &[_ZL],
                    (PS3 | PS4 | PS5 | PSMove | PSVita, LeftTrigger2) => &[_L2],
                    (Wii, LeftTrigger2) => &[_Z],

                    (
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController
                        | Oculus | Xbox360 | XboxOne | Switch,
                        RightTrigger,
                    ) => &[_RB],
                    (PS3 | PS4 | PS5 | PSMove | PSVita, RightTrigger) => &[_R1],
                    (WiiU, RightTrigger) => &[_R],
                    (Wii| Vive, RightTrigger) => &[&[]],

                    (
                        XboxSeries | SteamDeck | AmazonLuna | GoogleStadia | Ouya | SteamController
                        | Oculus | Xbox360 | XboxOne | Switch,
                        RightTrigger2,
                    ) => &[_RT],
                    (WiiU, RightTrigger2) => &[_ZR],
                    (PS3 | PS4 | PS5 | PSMove | PSVita, RightTrigger2) => &[_R2],
                    (Wii | Vive, RightTrigger2) => &[&[]],

                    (
                        Xbox360 | XboxOne | XboxSeries | SteamController | SteamDeck | AmazonLuna
                        | GoogleStadia | Ouya | Oculus | Vive | Switch | Wii | WiiU | PS3 | PS4
                        | PS5 | PSMove | PSVita,
                        C,
                    ) => &[&[]],

                    (
                        Xbox360 | XboxOne | XboxSeries | SteamController | SteamDeck | AmazonLuna
                        | GoogleStadia | Ouya | Oculus | Vive | Switch | Wii | WiiU | PS3 | PS4
                        | PS5 | PSMove | PSVita,
                        Z,
                    ) => &[&[]],

                    (XboxSeries, Select) => &[_View],
                    (PS4 | PS5, Select) => &[_Share],
                    (SteamDeck, Select) => &[_Inventory],
                    (AmazonLuna, Select) => &[_Microphone],
                    (GoogleStadia, Select) => &[_Dots],
                    (Ouya, Select) => &[_Touch],
                    (PS3 | PSMove | PSVita, Select) => &[_Select],
                    (SteamController | Xbox360, Select) => &[_Back],
                    (Wii | WiiU | Switch | Oculus, Select) => &[_Minus],
                    (Vive, Select) => &[_System],
                    (XboxOne, Select) => &[_Windows],

                    (Wii | WiiU | Switch | Oculus, Start) => &[_Plus],
                    (
                        XboxSeries | XboxOne | SteamDeck | AmazonLuna | GoogleStadia | Ouya,
                        Start,
                    ) => &[_Menu],
                    (PS4 | PS5, Start) => &[_Options],
                    (PS3 | PSMove | PSVita | SteamController | Xbox360, Start) => &[_Start],
                    (Vive, Start) => &[_Menu],

                    (XboxSeries | AmazonLuna | XboxOne, Mode) => &[&[]],
                    (PSVita | PS4 | PS5, Mode) => &[_Touch, _Pad],
                    (GoogleStadia, Mode) => &[_Assistant],
                    (SteamDeck | Vive, Mode) => &[_System],
                    (Switch | Wii | WiiU | PS3 | SteamController | Xbox360 | Ouya | PSMove | Oculus, Mode) => &[&[]],

                    (
                        XboxSeries | Switch | SteamDeck | PS5 | AmazonLuna | PS3 | PS4 | Xbox360
                        | XboxOne,
                        LeftThumb,
                    ) => &[_Left_0, _Stick, _Click],
                    (GoogleStadia | Ouya | PSMove | PSVita | Oculus | WiiU, LeftThumb) => {
                        &[_Left_0, _Stick]
                    }
                    (SteamController, LeftThumb) => &[_Stick],
                    (Vive | Wii, LeftThumb) => &[&[]],

                    (
                        XboxSeries | Switch | SteamDeck | PS5 | AmazonLuna | PS3 | PS4 | Xbox360
                        | XboxOne,
                        RightThumb,
                    ) => &[_Right_0, _Stick, _Click],
                    (GoogleStadia | Ouya | PSMove | PSVita | Oculus | WiiU, RightThumb) => {
                        &[_Right_0, _Stick]
                    }
                    (SteamController, RightThumb) => &[_Track, _Right_0, _Center],
                    (Vive | Wii, RightThumb) => &[&[]],

                    (
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita
                        | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus
                        | Vive | Wii | WiiU | Switch,
                        DPadUp,
                    ) => &[_Up, _Dpad],
                    (
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita
                        | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus
                        | Vive | Wii | WiiU | Switch,
                        DPadDown,
                    ) => &[_Down, _Dpad],
                    (
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita
                        | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus
                        | Vive | Wii | WiiU | Switch,
                        DPadLeft,
                    ) => &[_Left_0, _Dpad],
                    (
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita
                        | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus
                        | Vive | Wii | WiiU | Switch,
                        DPadRight,
                    ) => &[_Right_0, _Dpad],
                    (
                        Xbox360 | XboxOne | XboxSeries | PS3 | PS4 | PS5 | PSMove | PSVita
                        | SteamDeck | SteamController | AmazonLuna | GoogleStadia | Ouya | Oculus
                        | Vive | Wii | WiiU | Switch,
                        Other(_),
                    ) => &[],
                };
                (
                    input,
                    match XeluGamepadBrand::from(self.0) {
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
