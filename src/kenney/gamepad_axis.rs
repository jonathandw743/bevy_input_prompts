use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadAxis;

use crate::not_found::gamepad_axis::NotFoundGamepadAxis;

use super::{GamepadBrand, KenneyGamepadSettings};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Positive,
    Negative,
    Bidirectional,
    None,
}

/// converts to a Kenney's input prompt representing a [`GamepadAxis`]
#[derive(Clone, Debug)]
pub struct KenneyGamepadAxis {
    pub gamepad_axis: GamepadAxis,
    pub direction: Direction,
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
            "bevy_input_prompts/kenney/kenney_input-prompts/{}/{}/{}_{}{}.{}",
            self.settings.gamepad_brand.directory(),
            self.settings.format.directiory(),
            self.gamepad_brand_prefix(),
            gamepad_axis_name,
            self.outline_name(),
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
    pub fn outline_possible(&self) -> bool {
        match self.gamepad_axis {
            GamepadAxis::LeftZ | GamepadAxis::RightZ => true,
            _ => false,
        }
    }
    pub fn outline_name(&self) -> &'static str {
        if self.settings.outline_if_possible && self.outline_possible() {
            "_outline"
        } else {
            ""
        }
    }
    pub fn gamepad_axis_name(&self) -> Option<&'static str> {
        match (
            self.settings.gamepad_brand,
            self.gamepad_axis,
            self.direction,
        ) {
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::LeftStickX | GamepadAxis::LeftStickY,
                Direction::None,
            ) => Some("stick_top_l"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::LeftStickX,
                Direction::Bidirectional,
            ) => Some("stick_l_horizontal"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::LeftStickX,
                Direction::Positive,
            ) => Some("stick_l_right"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::LeftStickX,
                Direction::Negative,
            ) => Some("stick_l_left"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::LeftStickY,
                Direction::Bidirectional,
            ) => Some("stick_l_vertical"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::LeftStickY,
                Direction::Positive,
            ) => Some("stick_l_up"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::LeftStickY,
                Direction::Negative,
            ) => Some("stick_l_down"),

            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::RightStickX | GamepadAxis::RightStickY,
                Direction::None,
            ) => Some("stick_top_r"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::RightStickX,
                Direction::Bidirectional,
            ) => Some("stick_r_horizontal"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::RightStickX,
                Direction::Positive,
            ) => Some("stick_r_right"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::RightStickX,
                Direction::Negative,
            ) => Some("stick_r_left"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::RightStickY,
                Direction::Bidirectional,
            ) => Some("stick_r_vertical"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::RightStickY,
                Direction::Positive,
            ) => Some("stick_r_up"),
            (
                GamepadBrand::XboxSeries
                | GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5
                | GamepadBrand::SteamDeck,
                GamepadAxis::RightStickY,
                Direction::Negative,
            ) => Some("stick_r_down"),

            _ => None,
        }
    }
}
