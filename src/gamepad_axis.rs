use bevy_input::gamepad::GamepadAxis;

#[cfg(feature = "use_kenney_input_prompts")]
use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4::{
    _Generic as generic, _Nintendo_Gamecube as gamecube, _Nintendo_Switch as switch,
    _Nintendo_Switch_2 as switch2, _Nintendo_Wii as wii, _Nintendo_WiiU as wiiu,
    _PlayStation_Series as ps, _Steam_Controller as steam,
    _Steam_Deck as steamdeck, _Xbox_Series as xbox,
};

use crate::{gamepad_brand::GamepadBrand, Pack, ToFile};

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

                (GamepadBrand::Gamecube, GamepadAxis::LeftStickX) => Some(&[gamecube::stem_words::_grip, gamecube::stem_words::_stick]),
                (GamepadBrand::Gamecube, GamepadAxis::LeftStickY) => Some(&[gamecube::stem_words::_grip, gamecube::stem_words::_stick]),
                (GamepadBrand::Gamecube, GamepadAxis::LeftZ) => Some(&[gamecube::stem_words::_l, gamecube::stem_words::_trigger]),
                (GamepadBrand::Gamecube, GamepadAxis::RightStickX) => Some(&[gamecube::stem_words::_c, gamecube::stem_words::_stick]),
                (GamepadBrand::Gamecube, GamepadAxis::RightStickY) => Some(&[gamecube::stem_words::_c, gamecube::stem_words::_stick]),
                (GamepadBrand::Gamecube, GamepadAxis::RightZ) => Some(&[gamecube::stem_words::_r, gamecube::stem_words::_trigger]),
                (GamepadBrand::Gamecube, GamepadAxis::Other(_)) => None,

                (GamepadBrand::Switch, GamepadAxis::LeftStickX) => Some(&[switch::stem_words::_l, switch::stem_words::_stick]),
                (GamepadBrand::Switch, GamepadAxis::LeftStickY) => Some(&[switch::stem_words::_l, switch::stem_words::_stick]),
                (GamepadBrand::Switch, GamepadAxis::LeftZ) => Some(&[switch::stem_words::_zl]),
                (GamepadBrand::Switch, GamepadAxis::RightStickX) => Some(&[switch::stem_words::_r, switch::stem_words::_stick]),
                (GamepadBrand::Switch, GamepadAxis::RightStickY) => Some(&[switch::stem_words::_r, switch::stem_words::_stick]),
                (GamepadBrand::Switch, GamepadAxis::RightZ) => Some(&[switch::stem_words::_zr]),
                (GamepadBrand::Switch, GamepadAxis::Other(_)) => None,

                (GamepadBrand::Switch2, GamepadAxis::LeftStickX) => Some(&[switch2::stem_words::_l, switch2::stem_words::_stick]),
                (GamepadBrand::Switch2, GamepadAxis::LeftStickY) => Some(&[switch2::stem_words::_l, switch2::stem_words::_stick]),
                (GamepadBrand::Switch2, GamepadAxis::LeftZ) => Some(&[switch2::stem_words::_zl]),
                (GamepadBrand::Switch2, GamepadAxis::RightStickX) => Some(&[switch2::stem_words::_r, switch2::stem_words::_stick]),
                (GamepadBrand::Switch2, GamepadAxis::RightStickY) => Some(&[switch2::stem_words::_r, switch2::stem_words::_stick]),
                (GamepadBrand::Switch2, GamepadAxis::RightZ) => Some(&[switch2::stem_words::_zr]),
                (GamepadBrand::Switch2, GamepadAxis::Other(_)) => None,

                (GamepadBrand::Wii, GamepadAxis::LeftStickX) => Some(&[wii::stem_words::_l, wii::stem_words::_stick]),
                (GamepadBrand::Wii, GamepadAxis::LeftStickY) => Some(&[wii::stem_words::_l, wii::stem_words::_stick]),
                (GamepadBrand::Wii, GamepadAxis::LeftZ) => Some(&[wii::stem_words::_zl]),
                (GamepadBrand::Wii, GamepadAxis::RightStickX) => Some(&[wii::stem_words::_r, wii::stem_words::_stick]),
                (GamepadBrand::Wii, GamepadAxis::RightStickY) => Some(&[wii::stem_words::_r, wii::stem_words::_stick]),
                (GamepadBrand::Wii, GamepadAxis::RightZ) => Some(&[wii::stem_words::_zr]),
                (GamepadBrand::Wii, GamepadAxis::Other(_)) => None,

                (GamepadBrand::WiiU, GamepadAxis::LeftStickX) => Some(&[wiiu::stem_words::_l, wiiu::stem_words::_stick]),
                (GamepadBrand::WiiU, GamepadAxis::LeftStickY) => Some(&[wiiu::stem_words::_l, wiiu::stem_words::_stick]),
                (GamepadBrand::WiiU, GamepadAxis::LeftZ) => Some(&[wiiu::stem_words::_zl]),
                (GamepadBrand::WiiU, GamepadAxis::RightStickX) => Some(&[wiiu::stem_words::_r, wiiu::stem_words::_stick]),
                (GamepadBrand::WiiU, GamepadAxis::RightStickY) => Some(&[wiiu::stem_words::_r, wiiu::stem_words::_stick]),
                (GamepadBrand::WiiU, GamepadAxis::RightZ) => Some(&[wiiu::stem_words::_zr]),
                (GamepadBrand::WiiU, GamepadAxis::Other(_)) => None,

                (GamepadBrand::PS3 | GamepadBrand::PS4 | GamepadBrand::PS5, GamepadAxis::LeftStickX) => Some(&[ps::stem_words::_l, ps::stem_words::_stick]),
                (GamepadBrand::PS3 | GamepadBrand::PS4 | GamepadBrand::PS5, GamepadAxis::LeftStickY) => Some(&[ps::stem_words::_l, ps::stem_words::_stick]),
                (GamepadBrand::PS3 | GamepadBrand::PS4 | GamepadBrand::PS5, GamepadAxis::LeftZ) => Some(&[ps::stem_words::_l2]),
                (GamepadBrand::PS3 | GamepadBrand::PS4 | GamepadBrand::PS5, GamepadAxis::RightStickX) => Some(&[ps::stem_words::_r    , ps::stem_words::_stick]),
                (GamepadBrand::PS3 | GamepadBrand::PS4 | GamepadBrand::PS5, GamepadAxis::RightStickY) => Some(&[ps::stem_words::_r, ps::stem_words::_stick]),
                (GamepadBrand::PS3 | GamepadBrand::PS4 | GamepadBrand::PS5, GamepadAxis::RightZ) => Some(&[ps::stem_words::_r2]),
                (GamepadBrand::PS3 | GamepadBrand::PS4 | GamepadBrand::PS5, GamepadAxis::Other(_)) => None,
                
                // TODO: the spinny thing on the playdate probably is mapped to some Other
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

                (GamepadBrand::Xbox, GamepadAxis::LeftStickX) => Some(&[xbox::stem_words::_l, xbox::stem_words::_stick]),
                (GamepadBrand::Xbox, GamepadAxis::LeftStickY) => Some(&[xbox::stem_words::_l, xbox::stem_words::_stick]),
                (GamepadBrand::Xbox, GamepadAxis::LeftZ) => Some(&[xbox::stem_words::_lt]),
                (GamepadBrand::Xbox, GamepadAxis::RightStickX) => Some(&[xbox::stem_words::_l, xbox::stem_words::_stick]),
                (GamepadBrand::Xbox, GamepadAxis::RightStickY) => Some(&[xbox::stem_words::_l, xbox::stem_words::_stick]),
                (GamepadBrand::Xbox, GamepadAxis::RightZ) => Some(&[xbox::stem_words::_rt]),
                (GamepadBrand::Xbox, GamepadAxis::Other(_)) => None,
            },
            #[cfg(feature = "use_xelu_free_controller_key_prompts")]
            #[rustfmt::skip]
            Pack::Xelu => match (gamepad_brand, self) {
                _ => todo!(),
            }
        }
    }
}
