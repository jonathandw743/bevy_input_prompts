use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadAxis;

use crate::not_found::gamepad_axis::NotFoundGamepadAxis;

use super::{GamepadBrand, KenneyGamepadSettings};

#[derive(Clone, Debug)]
pub struct KenneyGamepadAxis {
    pub gamepad_axis: GamepadAxis,
    pub settings: KenneyGamepadSettings,
}

impl<'a> Into<AssetPath<'a>> for KenneyGamepadAxis {
    fn into(self) -> AssetPath<'a> {
        let Some(gamepad_axis_name) = self.gamepad_axis_name() else {
            return NotFoundGamepadAxis {
                gamepad_axis: self.gamepad_axis,
            }
            .into();
        };
        format!(
            "bevy_input_prompts/kenney/kenney_input-prompts/{}/{}/{}_{}.{}",
            self.settings.gamepad_brand.directory(),
            self.settings.format.directiory(),
            self.gamepad_brand_prefix(),
            gamepad_axis_name,
            self.settings.format.extension(),
        )
        .into()
    }
}

impl KenneyGamepadAxis {
    pub fn gamepad_brand_prefix(&self) -> &'static str {
        match self.settings.gamepad_brand {
            GamepadBrand::Generic => "generic",
            GamepadBrand::Switch => "switch",
            GamepadBrand::Wii => "wii",
            GamepadBrand::WiiU => "wiiu",
            GamepadBrand::Playdate => "playdate",
            GamepadBrand::PlayStation3
            | GamepadBrand::PlayStation4
            | GamepadBrand::PlayStation5 => "playstation",
            GamepadBrand::SteamController => "steam",
            GamepadBrand::SteamDeck => "steamdeck",
            GamepadBrand::XboxSeries => "xbox",
        }
    }
    pub fn gamepad_axis_name(&self) -> Option<&'static str> {
        match (self.settings.gamepad_brand, self.gamepad_axis) {
            (GamepadBrand::Generic, GamepadAxis::LeftStickX) => None,
            (GamepadBrand::Generic, GamepadAxis::LeftStickY) => None,
            (GamepadBrand::Generic, GamepadAxis::LeftZ) => None,
            (GamepadBrand::Generic, GamepadAxis::RightStickX) => None,
            (GamepadBrand::Generic, GamepadAxis::RightStickY) => None,
            (GamepadBrand::Generic, GamepadAxis::RightZ) => None,
            (GamepadBrand::Generic, GamepadAxis::Other(_)) => None,
            (GamepadBrand::Switch, GamepadAxis::LeftStickX) => None,
            (GamepadBrand::Switch, GamepadAxis::LeftStickY) => None,
            (GamepadBrand::Switch, GamepadAxis::LeftZ) => None,
            (GamepadBrand::Switch, GamepadAxis::RightStickX) => None,
            (GamepadBrand::Switch, GamepadAxis::RightStickY) => None,
            (GamepadBrand::Switch, GamepadAxis::RightZ) => None,
            (GamepadBrand::Switch, GamepadAxis::Other(_)) => None,
            (GamepadBrand::Wii, GamepadAxis::LeftStickX) => None,
            (GamepadBrand::Wii, GamepadAxis::LeftStickY) => None,
            (GamepadBrand::Wii, GamepadAxis::LeftZ) => None,
            (GamepadBrand::Wii, GamepadAxis::RightStickX) => None,
            (GamepadBrand::Wii, GamepadAxis::RightStickY) => None,
            (GamepadBrand::Wii, GamepadAxis::RightZ) => None,
            (GamepadBrand::Wii, GamepadAxis::Other(_)) => None,
            (GamepadBrand::WiiU, GamepadAxis::LeftStickX) => None,
            (GamepadBrand::WiiU, GamepadAxis::LeftStickY) => None,
            (GamepadBrand::WiiU, GamepadAxis::LeftZ) => None,
            (GamepadBrand::WiiU, GamepadAxis::RightStickX) => None,
            (GamepadBrand::WiiU, GamepadAxis::RightStickY) => None,
            (GamepadBrand::WiiU, GamepadAxis::RightZ) => None,
            (GamepadBrand::WiiU, GamepadAxis::Other(_)) => None,
            (GamepadBrand::Playdate, GamepadAxis::LeftStickX) => None,
            (GamepadBrand::Playdate, GamepadAxis::LeftStickY) => None,
            (GamepadBrand::Playdate, GamepadAxis::LeftZ) => None,
            (GamepadBrand::Playdate, GamepadAxis::RightStickX) => None,
            (GamepadBrand::Playdate, GamepadAxis::RightStickY) => None,
            (GamepadBrand::Playdate, GamepadAxis::RightZ) => None,
            (GamepadBrand::Playdate, GamepadAxis::Other(_)) => None,
            // (
            //     GamepadBrand::PlayStation3
            //     | GamepadBrand::PlayStation4
            //     | GamepadBrand::PlayStation5,
            //     GamepadAxis::LeftStickX | GamepadAxis::LeftStickY,
            // ) => Some("stick_top_l"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadAxis::LeftStickX,
            ) => Some("stick_l_horizontal"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadAxis::LeftStickY,
            ) => Some("stick_l_vertical"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadAxis::LeftZ,
            ) => Some("trigger_l2"),
            // (
            //     GamepadBrand::PlayStation3
            //     | GamepadBrand::PlayStation4
            //     | GamepadBrand::PlayStation5,
            //     GamepadAxis::RightStickX | GamepadAxis::RightStickY,
            // ) => Some("stick_top_r"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadAxis::RightStickX,
            ) => Some("stick_r_horizontal"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadAxis::RightStickY,
            ) => Some("stick_r_vertical"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadAxis::RightZ,
            ) => Some("trigger_r2"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadAxis::Other(_),
            ) => None,
            (GamepadBrand::SteamController, GamepadAxis::LeftStickX) => None,
            (GamepadBrand::SteamController, GamepadAxis::LeftStickY) => None,
            (GamepadBrand::SteamController, GamepadAxis::LeftZ) => None,
            (GamepadBrand::SteamController, GamepadAxis::RightStickX) => None,
            (GamepadBrand::SteamController, GamepadAxis::RightStickY) => None,
            (GamepadBrand::SteamController, GamepadAxis::RightZ) => None,
            (GamepadBrand::SteamController, GamepadAxis::Other(_)) => None,
            // (GamepadBrand::SteamDeck, GamepadAxis::LeftStickX | GamepadAxis::LeftStickY) => Some("stick_top_l"),
            (GamepadBrand::SteamDeck, GamepadAxis::LeftStickX) => Some("stick_l_horizontal"),
            (GamepadBrand::SteamDeck, GamepadAxis::LeftStickY) => Some("stick_l_vertical"),
            (GamepadBrand::SteamDeck, GamepadAxis::LeftZ) => Some("button_l2"),
            // (GamepadBrand::SteamDeck, GamepadAxis::RightStickX | GamepadAxis::RightStickY) => Some("stick_top_r"),
            (GamepadBrand::SteamDeck, GamepadAxis::RightStickX) => Some("stick_r_horizontal"),
            (GamepadBrand::SteamDeck, GamepadAxis::RightStickY) => Some("stick_r_vertical"),
            (GamepadBrand::SteamDeck, GamepadAxis::RightZ) => Some("button_r2"),
            (GamepadBrand::SteamDeck, GamepadAxis::Other(_)) => None,
            // (GamepadBrand::XboxSeries, GamepadAxis::LeftStickX | GamepadAxis::LeftStickY) => Some("stick_top_l"),
            (GamepadBrand::XboxSeries, GamepadAxis::LeftStickX) => Some("stick_l_horizontal"),
            (GamepadBrand::XboxSeries, GamepadAxis::LeftStickY) => Some("stick_l_vertical"),
            (GamepadBrand::XboxSeries, GamepadAxis::LeftZ) => Some("lt"),
            // (GamepadBrand::XboxSeries, GamepadAxis::LeftStickX | GamepadAxis::LeftStickY) => Some("stick_top_l"),
            (GamepadBrand::XboxSeries, GamepadAxis::RightStickX) => Some("stick_r_horizontal"),
            (GamepadBrand::XboxSeries, GamepadAxis::RightStickY) => Some("stick_r_vertical"),
            (GamepadBrand::XboxSeries, GamepadAxis::RightZ) => Some("rt"),
            (GamepadBrand::XboxSeries, GamepadAxis::Other(_)) => None,
        }
    }
}
