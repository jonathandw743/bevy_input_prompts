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

                (GamepadBrand::NintendoSwitch, GamepadAxis::LeftStickX) => todo!(),
                (GamepadBrand::NintendoSwitch, GamepadAxis::LeftStickY) => todo!(),
                (GamepadBrand::NintendoSwitch, GamepadAxis::LeftZ) => todo!(),
                (GamepadBrand::NintendoSwitch, GamepadAxis::RightStickX) => todo!(),
                (GamepadBrand::NintendoSwitch, GamepadAxis::RightStickY) => todo!(),
                (GamepadBrand::NintendoSwitch, GamepadAxis::RightZ) => todo!(),
                (GamepadBrand::NintendoSwitch, GamepadAxis::Other(_)) => todo!(),

                (GamepadBrand::NintendoSwitch2, GamepadAxis::LeftStickX) => todo!(),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::LeftStickY) => todo!(),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::LeftZ) => todo!(),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::RightStickX) => todo!(),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::RightStickY) => todo!(),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::RightZ) => todo!(),
                (GamepadBrand::NintendoSwitch2, GamepadAxis::Other(_)) => todo!(),

                (GamepadBrand::NintendoWii, GamepadAxis::LeftStickX) => todo!(),
                (GamepadBrand::NintendoWii, GamepadAxis::LeftStickY) => todo!(),
                (GamepadBrand::NintendoWii, GamepadAxis::LeftZ) => todo!(),
                (GamepadBrand::NintendoWii, GamepadAxis::RightStickX) => todo!(),
                (GamepadBrand::NintendoWii, GamepadAxis::RightStickY) => todo!(),
                (GamepadBrand::NintendoWii, GamepadAxis::RightZ) => todo!(),
                (GamepadBrand::NintendoWii, GamepadAxis::Other(_)) => todo!(),

                (GamepadBrand::NintendoWiiU, GamepadAxis::LeftStickX) => todo!(),
                (GamepadBrand::NintendoWiiU, GamepadAxis::LeftStickY) => todo!(),
                (GamepadBrand::NintendoWiiU, GamepadAxis::LeftZ) => todo!(),
                (GamepadBrand::NintendoWiiU, GamepadAxis::RightStickX) => todo!(),
                (GamepadBrand::NintendoWiiU, GamepadAxis::RightStickY) => todo!(),
                (GamepadBrand::NintendoWiiU, GamepadAxis::RightZ) => todo!(),
                (GamepadBrand::NintendoWiiU, GamepadAxis::Other(_)) => todo!(),

                (GamepadBrand::PlayStationSeries, GamepadAxis::LeftStickX) => Some(&[ps::stem_words::_l, ps::stem_words::_stick]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::LeftStickY) => Some(&[ps::stem_words::_l, ps::stem_words::_stick]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::LeftZ) => Some(&[ps::stem_words::_l2]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::RightStickX) => Some(&[ps::stem_words::_l, ps::stem_words::_stick]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::RightStickY) => Some(&[ps::stem_words::_l, ps::stem_words::_stick]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::RightZ) => Some(&[ps::stem_words::_r2]),
                (GamepadBrand::PlayStationSeries, GamepadAxis::Other(_)) => None,

                (GamepadBrand::Playdate, GamepadAxis::LeftStickX) => todo!(),
                (GamepadBrand::Playdate, GamepadAxis::LeftStickY) => todo!(),
                (GamepadBrand::Playdate, GamepadAxis::LeftZ) => todo!(),
                (GamepadBrand::Playdate, GamepadAxis::RightStickX) => todo!(),
                (GamepadBrand::Playdate, GamepadAxis::RightStickY) => todo!(),
                (GamepadBrand::Playdate, GamepadAxis::RightZ) => todo!(),
                (GamepadBrand::Playdate, GamepadAxis::Other(_)) => todo!(),

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
