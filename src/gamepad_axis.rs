use bevy_input::gamepad::GamepadAxis;

#[cfg(feature = "use_kenney_input_prompts")]
use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::{
    _Generic as generic, _Nintendo_Gamecube as gamecube, _Nintendo_Switch as switch,
    _Nintendo_Switch_2 as switch2, _Nintendo_Wii as wii, _Nintendo_WiiU as wiiu,
    _PlayStation_Series as ps, _Playdate as playdate, _Steam_Controller as steam,
    _Steam_Deck as steamdeck, _Xbox_Series as xbox,
};

use crate::{Pack, ToFile};

#[derive(Clone, Copy)]
pub enum GamepadBrand {
    Generic,
    NintendoGamecube,
    NintendoSwitch,
    NintendoSwitch2,
    NintendoWii,
    NintendoWiiU,
    PlayStationSeries,
    Playdate,
    SteamController,
    SteamDeck,
    XboxSeries,
}

impl ToFile for GamepadAxis {
    type Options = GamepadBrand;

    fn file_indices<'a, 'b>(
        &self,
        pack: Pack,
        gamepad_brand: GamepadBrand,
    ) -> Option<&'a [&'b [usize]]> {
        match pack {
            #[cfg(feature = "use_kenney_input_prompts")]
            #[rustfmt::skip]
            Pack::Kenney => match (gamepad_brand, self) {
                (GamepadBrand::Generic, GamepadAxis::LeftStickX) => Some(&[generic::stem_words::_stick]),
                (GamepadBrand::Generic, GamepadAxis::LeftStickY) => Some(&[generic::stem_words::_stick]),
                (GamepadBrand::Generic, GamepadAxis::LeftZ) => Some(&[generic::stem_words::_trigger]),
                (GamepadBrand::Generic, GamepadAxis::RightStickX) => Some(&[generic::stem_words::_stick]),
                (GamepadBrand::Generic, GamepadAxis::RightStickY) => Some(&[generic::stem_words::_stick]),
                (GamepadBrand::Generic, GamepadAxis::RightZ) => Some(&[generic::stem_words::_trigger]),
                (GamepadBrand::Generic, GamepadAxis::Other(_)) => None,

                (GamepadBrand::NintendoGamecube, GamepadAxis::LeftStickX) => Some(&[gamecube::stem_words::_grip, gamecube::stem_words::_stick]),
                (GamepadBrand::NintendoGamecube, GamepadAxis::LeftStickY) => Some(&[gamecube::stem_words::_grip, gamecube::stem_words::_stick]),
                (GamepadBrand::NintendoGamecube, GamepadAxis::LeftZ) => Some(&[gamecube::stem_words::_l, gamecube::stem_words::_trigger]),
                (GamepadBrand::NintendoGamecube, GamepadAxis::RightStickX) => Some(&[gamecube::stem_words::_c, gamecube::stem_words::_stick]),
                (GamepadBrand::NintendoGamecube, GamepadAxis::RightStickY) => Some(&[gamecube::stem_words::_c, gamecube::stem_words::_stick]),
                (GamepadBrand::NintendoGamecube, GamepadAxis::RightZ) => Some(&[gamecube::stem_words::_r, gamecube::stem_words::_trigger]),
                (GamepadBrand::NintendoGamecube, GamepadAxis::Other(_)) => None,

                (GamepadBrand::NintendoSwitch, GamepadAxis::LeftStickX) => Some(&[switch::stem_words::_l, switch::stem_words::_stick]),
                (GamepadBrand::NintendoSwitch, GamepadAxis::LeftStickY) => Some(&[switch::stem_words::_l, switch::stem_words::_stick]),
                (GamepadBrand::NintendoSwitch, GamepadAxis::LeftZ) => Some(&[switch::stem_words::_zl]),
                (GamepadBrand::NintendoSwitch, GamepadAxis::RightStickX) => Some(&[switch::stem_words::_r, switch::stem_words::_stick]),
                (GamepadBrand::NintendoSwitch, GamepadAxis::RightStickY) => Some(&[switch::stem_words::_r, switch::stem_words::_stick]),
                (GamepadBrand::NintendoSwitch, GamepadAxis::RightZ) => Some(&[switch::stem_words::_zr]),
                (GamepadBrand::NintendoSwitch, GamepadAxis::Other(_)) => None,

                (GamepadBrand::NintendoSwitch2, GamepadAxis::LeftStickX) => Some(&[switch2::stem_words::_l, switch2::stem_words::_stick]),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::LeftStickY) => Some(&[switch2::stem_words::_l, switch2::stem_words::_stick]),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::LeftZ) => Some(&[switch2::stem_words::_zl]),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::RightStickX) => Some(&[switch2::stem_words::_r, switch2::stem_words::_stick]),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::RightStickY) => Some(&[switch2::stem_words::_r, switch2::stem_words::_stick]),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::RightZ) => Some(&[switch2::stem_words::_zr]),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::Other(_)) => None,

                (GamepadBrand::NintendoWii, GamepadAxis::LeftStickX) => Some(&[wii::stem_words::_l, wii::stem_words::_stick]),
                (GamepadBrand::NintendoWii, GamepadAxis::LeftStickY) => Some(&[wii::stem_words::_l, wii::stem_words::_stick]),
                (GamepadBrand::NintendoWii, GamepadAxis::LeftZ) => Some(&[wii::stem_words::_zl]),
                (GamepadBrand::NintendoWii, GamepadAxis::RightStickX) => Some(&[wii::stem_words::_r, wii::stem_words::_stick]),
                (GamepadBrand::NintendoWii, GamepadAxis::RightStickY) => Some(&[wii::stem_words::_r, wii::stem_words::_stick]),
                (GamepadBrand::NintendoWii, GamepadAxis::RightZ) => Some(&[wii::stem_words::_zr]),
                (GamepadBrand::NintendoWii, GamepadAxis::Other(_)) => None,

                (GamepadBrand::NintendoWiiU, GamepadAxis::LeftStickX) => Some(&[wiiu::stem_words::_l, wiiu::stem_words::_stick]),
                (GamepadBrand::NintendoWiiU, GamepadAxis::LeftStickY) => Some(&[wiiu::stem_words::_l, wiiu::stem_words::_stick]),
                (GamepadBrand::NintendoWiiU, GamepadAxis::LeftZ) => Some(&[wiiu::stem_words::_zl]),
                (GamepadBrand::NintendoWiiU, GamepadAxis::RightStickX) => Some(&[wiiu::stem_words::_r, wiiu::stem_words::_stick]),
                (GamepadBrand::NintendoWiiU, GamepadAxis::RightStickY) => Some(&[wiiu::stem_words::_r, wiiu::stem_words::_stick]),
                (GamepadBrand::NintendoWiiU, GamepadAxis::RightZ) => Some(&[wiiu::stem_words::_zr]),
                (GamepadBrand::NintendoWiiU, GamepadAxis::Other(_)) => None,

                (GamepadBrand::PlayStationSeries, GamepadAxis::LeftStickX) => Some(&[ps::stem_words::_l, ps::stem_words::_stick]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::LeftStickY) => Some(&[ps::stem_words::_l, ps::stem_words::_stick]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::LeftZ) => Some(&[ps::stem_words::_l2]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::RightStickX) => Some(&[ps::stem_words::_r    , ps::stem_words::_stick]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::RightStickY) => Some(&[ps::stem_words::_r, ps::stem_words::_stick]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::RightZ) => Some(&[ps::stem_words::_r2]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::Other(_)) => None,
                
                // TODO:
                (GamepadBrand::Playdate, GamepadAxis::LeftStickX) => None,
                (GamepadBrand::Playdate, GamepadAxis::LeftStickY) => None,
                (GamepadBrand::Playdate, GamepadAxis::LeftZ) => None,
                (GamepadBrand::Playdate, GamepadAxis::RightStickX) => None,
                (GamepadBrand::Playdate, GamepadAxis::RightStickY) => None,
                (GamepadBrand::Playdate, GamepadAxis::RightZ) => None,
                (GamepadBrand::Playdate, GamepadAxis::Other(_)) => None,

                (GamepadBrand::SteamController, GamepadAxis::LeftStickX) => Some(&[steam::stem_words::_l, steam::stem_words::_stick]),
                (GamepadBrand::SteamController, GamepadAxis::LeftStickY) => Some(&[steam::stem_words::_l, steam::stem_words::_stick]),
                (GamepadBrand::SteamController, GamepadAxis::LeftZ) => Some(&[steam::stem_words::_lt]),
                (GamepadBrand::SteamController, GamepadAxis::RightStickX) => Some(&[steam::stem_words::_l, steam::stem_words::_stick]),
                (GamepadBrand::SteamController, GamepadAxis::RightStickY) => Some(&[steam::stem_words::_l, steam::stem_words::_stick]),
                (GamepadBrand::SteamController, GamepadAxis::RightZ) => Some(&[steam::stem_words::_rt]),
                (GamepadBrand::SteamController, GamepadAxis::Other(_)) => None,

                (GamepadBrand::SteamDeck, GamepadAxis::LeftStickX) => Some(&[steamdeck::stem_words::_l, steamdeck::stem_words::_stick]),
                (GamepadBrand::SteamDeck, GamepadAxis::LeftStickY) => Some(&[steamdeck::stem_words::_l, steamdeck::stem_words::_stick]),
                (GamepadBrand::SteamDeck, GamepadAxis::LeftZ) => Some(&[steamdeck::stem_words::_l2]),
                (GamepadBrand::SteamDeck, GamepadAxis::RightStickX) => Some(&[steamdeck::stem_words::_l, steamdeck::stem_words::_stick]),
                (GamepadBrand::SteamDeck, GamepadAxis::RightStickY) => Some(&[steamdeck::stem_words::_l, steamdeck::stem_words::_stick]),
                (GamepadBrand::SteamDeck, GamepadAxis::RightZ) => Some(&[steamdeck::stem_words::_r2]),
                (GamepadBrand::SteamDeck, GamepadAxis::Other(_)) => None,

                (GamepadBrand::XboxSeries, GamepadAxis::LeftStickX) => Some(&[xbox::stem_words::_l, xbox::stem_words::_stick]),
                (GamepadBrand::XboxSeries, GamepadAxis::LeftStickY) => Some(&[xbox::stem_words::_l, xbox::stem_words::_stick]),
                (GamepadBrand::XboxSeries, GamepadAxis::LeftZ) => Some(&[xbox::stem_words::_lt]),
                (GamepadBrand::XboxSeries, GamepadAxis::RightStickX) => Some(&[xbox::stem_words::_l, xbox::stem_words::_stick]),
                (GamepadBrand::XboxSeries, GamepadAxis::RightStickY) => Some(&[xbox::stem_words::_l, xbox::stem_words::_stick]),
                (GamepadBrand::XboxSeries, GamepadAxis::RightZ) => Some(&[xbox::stem_words::_rt]),
                (GamepadBrand::XboxSeries, GamepadAxis::Other(_)) => None,
            },
        }
    }
}
