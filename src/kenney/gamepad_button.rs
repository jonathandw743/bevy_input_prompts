use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadButton;

use crate::not_found::gamepad_button::NotFoundGamepadButton;

use super::{GamepadBrand, KenneyGamepadSettings};

/// converts to a Kenney's input prompt representing a [`GamepadButton`]
#[derive(Clone, Debug)]
pub struct KenneyGamepadButton {
    pub gamepad_button: GamepadButton,
    pub settings: KenneyGamepadSettings,
}

impl<'a> Into<AssetPath<'a>> for KenneyGamepadButton {
    fn into(self) -> AssetPath<'a> {
        if let (Some(buttont_type), Some(gamepad_button_name)) =
            (self.button_type(), self.gamepad_button_name())
        {
            format!(
                "bevy_input_prompts/kenney/kenney_input-prompts/{}/{}/{}{}{}{}_{}{}.{}",
                self.settings.gamepad_brand.directory(),
                self.settings.format.directiory(),
                self.gamepad_brand_prefix(),
                buttont_type,
                self.round_name(),
                self.color_name(),
                gamepad_button_name,
                self.outline_name(),
                self.settings.format.extension()
            )
            .into()
        } else {
            return NotFoundGamepadButton {
                gamepad_button: self.gamepad_button,
            }
            .into();
        }
    }
}

impl KenneyGamepadButton {
    pub fn gamepad_brand_prefix(&self) -> &'static str {
        match (self.settings.gamepad_brand, self.gamepad_button) {
            (GamepadBrand::Generic, _) => "generic",
            (GamepadBrand::Switch, _) => "switch",
            (GamepadBrand::Wii, _) => "wii",
            (GamepadBrand::WiiU, _) => "wiiu",
            (GamepadBrand::Playdate, _) => "playdate",
            (
                GamepadBrand::PlayStation3,
                GamepadButton::Select | GamepadButton::Start | GamepadButton::Mode,
            ) => "playstation3",
            (
                GamepadBrand::PlayStation4,
                GamepadButton::Select | GamepadButton::Start | GamepadButton::Mode,
            ) => "playstation4",
            (
                GamepadBrand::PlayStation5,
                GamepadButton::Select | GamepadButton::Start | GamepadButton::Mode,
            ) => "playstation5",
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                _,
            ) => "playstation",
            (GamepadBrand::SteamController, _) => "steam",
            (GamepadBrand::SteamDeck, _) => "steamdeck",
            (GamepadBrand::XboxSeries, _) => "xbox",
        }
    }
    pub fn color_possible(&self) -> bool {
        match self.settings.gamepad_brand {
            GamepadBrand::SteamController
            | GamepadBrand::PlayStation3
            | GamepadBrand::PlayStation4
            | GamepadBrand::PlayStation5
            | GamepadBrand::XboxSeries => {}

            GamepadBrand::Generic
            | GamepadBrand::Switch
            | GamepadBrand::Wii
            | GamepadBrand::WiiU
            | GamepadBrand::Playdate
            | GamepadBrand::SteamDeck => {
                return false;
            }
        }
        match self.gamepad_button {
            GamepadButton::South
            | GamepadButton::East
            | GamepadButton::North
            | GamepadButton::West => {}

            GamepadButton::C
            | GamepadButton::Z
            | GamepadButton::LeftTrigger
            | GamepadButton::LeftTrigger2
            | GamepadButton::RightTrigger
            | GamepadButton::RightTrigger2
            | GamepadButton::Select
            | GamepadButton::Start
            | GamepadButton::Mode
            | GamepadButton::LeftThumb
            | GamepadButton::RightThumb
            | GamepadButton::DPadUp
            | GamepadButton::DPadDown
            | GamepadButton::DPadLeft
            | GamepadButton::DPadRight
            | GamepadButton::Other(_) => {
                return false;
            }
        }
        return true;
    }
    pub fn color_name(&self) -> &'static str {
        if self.settings.color_if_possible && self.color_possible() {
            "_color"
        } else {
            ""
        }
    }
    pub fn button_type(&self) -> Option<&'static str> {
        match (self.settings.gamepad_brand, self.gamepad_button) {
            (
                _,
                GamepadButton::Select
                | GamepadButton::Start
                | GamepadButton::South
                | GamepadButton::East
                | GamepadButton::North
                | GamepadButton::West,
            ) => Some("_button"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::LeftTrigger
                | GamepadButton::LeftTrigger2
                | GamepadButton::RightTrigger
                | GamepadButton::RightTrigger2,
            ) => Some("_trigger"),
            (
                GamepadBrand::SteamDeck,
                GamepadButton::LeftTrigger
                | GamepadButton::LeftTrigger2
                | GamepadButton::RightTrigger
                | GamepadButton::RightTrigger2,
            ) => Some("_button"),
            (
                GamepadBrand::XboxSeries,
                GamepadButton::LeftTrigger
                | GamepadButton::LeftTrigger2
                | GamepadButton::RightTrigger
                | GamepadButton::RightTrigger2,
            ) => Some(""),
            (_, GamepadButton::Mode) => Some(""),
            (_, GamepadButton::LeftThumb | GamepadButton::RightThumb) => Some("_stick"),
            (
                _,
                GamepadButton::DPadUp
                | GamepadButton::DPadDown
                | GamepadButton::DPadLeft
                | GamepadButton::DPadRight,
            ) => Some("_dpad"),
            _ => None,
        }
    }
    pub fn outline_possible(&self) -> bool {
        match self.gamepad_button {
            GamepadButton::C
            | GamepadButton::Z
            | GamepadButton::Other(_)
            | GamepadButton::LeftThumb
            | GamepadButton::RightThumb => {
                return false;
            }

            GamepadButton::South
            | GamepadButton::East
            | GamepadButton::North
            | GamepadButton::West
            | GamepadButton::LeftTrigger
            | GamepadButton::LeftTrigger2
            | GamepadButton::RightTrigger
            | GamepadButton::RightTrigger2
            | GamepadButton::Select
            | GamepadButton::Start
            | GamepadButton::Mode
            | GamepadButton::DPadUp
            | GamepadButton::DPadDown
            | GamepadButton::DPadLeft
            | GamepadButton::DPadRight => {}
        }
        return true;
    }
    pub fn outline_name(&self) -> &'static str {
        if self.settings.outline_if_possible
            && self.outline_possible()
            && !(self.settings.round_if_possible && self.round_possible())
        {
            "_outline"
        } else {
            ""
        }
    }
    pub fn round_possible(&self) -> bool {
        match self.settings.gamepad_brand {
            GamepadBrand::Generic
            | GamepadBrand::Switch
            | GamepadBrand::Wii
            | GamepadBrand::WiiU
            | GamepadBrand::Playdate
            | GamepadBrand::PlayStation3
            | GamepadBrand::PlayStation4
            | GamepadBrand::PlayStation5
            | GamepadBrand::SteamController
            | GamepadBrand::SteamDeck => {
                return false;
            }
            GamepadBrand::XboxSeries => {}
        }

        match self.gamepad_button {
            GamepadButton::South
            | GamepadButton::East
            | GamepadButton::North
            | GamepadButton::West
            | GamepadButton::C
            | GamepadButton::Z
            | GamepadButton::LeftTrigger
            | GamepadButton::LeftTrigger2
            | GamepadButton::RightTrigger
            | GamepadButton::RightTrigger2
            | GamepadButton::Select
            | GamepadButton::Start
            | GamepadButton::Mode
            | GamepadButton::LeftThumb
            | GamepadButton::RightThumb
            | GamepadButton::Other(_) => {
                return false;
            }
            GamepadButton::DPadUp
            | GamepadButton::DPadDown
            | GamepadButton::DPadLeft
            | GamepadButton::DPadRight => {}
        }
        return true;
    }
    pub fn round_name(&self) -> &'static str {
        if self.settings.round_if_possible && self.round_possible() {
            "_round"
        } else {
            ""
        }
    }
    pub fn gamepad_button_name(&self) -> Option<&'static str> {
        match (self.settings.gamepad_brand, self.gamepad_button) {
            (_, GamepadButton::LeftThumb) => Some("side_l"),
            (_, GamepadButton::RightThumb) => Some("side_r"),
            (_, GamepadButton::DPadUp) => Some("up"),
            (_, GamepadButton::DPadDown) => Some("down"),
            (_, GamepadButton::DPadLeft) => Some("left"),
            (_, GamepadButton::DPadRight) => Some("right"),

            (GamepadBrand::Generic, GamepadButton::South) => None,
            (GamepadBrand::Generic, GamepadButton::East) => None,
            (GamepadBrand::Generic, GamepadButton::North) => None,
            (GamepadBrand::Generic, GamepadButton::West) => None,
            (GamepadBrand::Generic, GamepadButton::C) => None,
            (GamepadBrand::Generic, GamepadButton::Z) => None,
            (GamepadBrand::Generic, GamepadButton::LeftTrigger) => None,
            (GamepadBrand::Generic, GamepadButton::LeftTrigger2) => None,
            (GamepadBrand::Generic, GamepadButton::RightTrigger) => None,
            (GamepadBrand::Generic, GamepadButton::RightTrigger2) => None,
            (GamepadBrand::Generic, GamepadButton::Select) => None,
            (GamepadBrand::Generic, GamepadButton::Start) => None,
            (GamepadBrand::Generic, GamepadButton::Mode) => None,
            (GamepadBrand::Generic, GamepadButton::Other(_)) => None,
            (GamepadBrand::Switch, GamepadButton::South) => None,
            (GamepadBrand::Switch, GamepadButton::East) => None,
            (GamepadBrand::Switch, GamepadButton::North) => None,
            (GamepadBrand::Switch, GamepadButton::West) => None,
            (GamepadBrand::Switch, GamepadButton::C) => None,
            (GamepadBrand::Switch, GamepadButton::Z) => None,
            (GamepadBrand::Switch, GamepadButton::LeftTrigger) => None,
            (GamepadBrand::Switch, GamepadButton::LeftTrigger2) => None,
            (GamepadBrand::Switch, GamepadButton::RightTrigger) => None,
            (GamepadBrand::Switch, GamepadButton::RightTrigger2) => None,
            (GamepadBrand::Switch, GamepadButton::Select) => None,
            (GamepadBrand::Switch, GamepadButton::Start) => None,
            (GamepadBrand::Switch, GamepadButton::Mode) => None,
            (GamepadBrand::Switch, GamepadButton::Other(_)) => None,
            (GamepadBrand::Wii, GamepadButton::South) => None,
            (GamepadBrand::Wii, GamepadButton::East) => None,
            (GamepadBrand::Wii, GamepadButton::North) => None,
            (GamepadBrand::Wii, GamepadButton::West) => None,
            (GamepadBrand::Wii, GamepadButton::C) => None,
            (GamepadBrand::Wii, GamepadButton::Z) => None,
            (GamepadBrand::Wii, GamepadButton::LeftTrigger) => None,
            (GamepadBrand::Wii, GamepadButton::LeftTrigger2) => None,
            (GamepadBrand::Wii, GamepadButton::RightTrigger) => None,
            (GamepadBrand::Wii, GamepadButton::RightTrigger2) => None,
            (GamepadBrand::Wii, GamepadButton::Select) => None,
            (GamepadBrand::Wii, GamepadButton::Start) => None,
            (GamepadBrand::Wii, GamepadButton::Mode) => None,
            (GamepadBrand::Wii, GamepadButton::Other(_)) => None,
            (GamepadBrand::WiiU, GamepadButton::South) => None,
            (GamepadBrand::WiiU, GamepadButton::East) => None,
            (GamepadBrand::WiiU, GamepadButton::North) => None,
            (GamepadBrand::WiiU, GamepadButton::West) => None,
            (GamepadBrand::WiiU, GamepadButton::C) => None,
            (GamepadBrand::WiiU, GamepadButton::Z) => None,
            (GamepadBrand::WiiU, GamepadButton::LeftTrigger) => None,
            (GamepadBrand::WiiU, GamepadButton::LeftTrigger2) => None,
            (GamepadBrand::WiiU, GamepadButton::RightTrigger) => None,
            (GamepadBrand::WiiU, GamepadButton::RightTrigger2) => None,
            (GamepadBrand::WiiU, GamepadButton::Select) => None,
            (GamepadBrand::WiiU, GamepadButton::Start) => None,
            (GamepadBrand::WiiU, GamepadButton::Mode) => None,
            (GamepadBrand::WiiU, GamepadButton::Other(_)) => None,
            (GamepadBrand::Playdate, GamepadButton::South) => None,
            (GamepadBrand::Playdate, GamepadButton::East) => None,
            (GamepadBrand::Playdate, GamepadButton::North) => None,
            (GamepadBrand::Playdate, GamepadButton::West) => None,
            (GamepadBrand::Playdate, GamepadButton::C) => None,
            (GamepadBrand::Playdate, GamepadButton::Z) => None,
            (GamepadBrand::Playdate, GamepadButton::LeftTrigger) => None,
            (GamepadBrand::Playdate, GamepadButton::LeftTrigger2) => None,
            (GamepadBrand::Playdate, GamepadButton::RightTrigger) => None,
            (GamepadBrand::Playdate, GamepadButton::RightTrigger2) => None,
            (GamepadBrand::Playdate, GamepadButton::Select) => None,
            (GamepadBrand::Playdate, GamepadButton::Start) => None,
            (GamepadBrand::Playdate, GamepadButton::Mode) => None,
            (GamepadBrand::Playdate, GamepadButton::Other(_)) => None,
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::South,
            ) => Some("cross"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::East,
            ) => Some("circle"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::North,
            ) => Some("triangle"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::West,
            ) => Some("square"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::C,
            ) => None,
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::Z,
            ) => None,
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::LeftTrigger,
            ) => Some("l1"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::LeftTrigger2,
            ) => Some("l2"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::RightTrigger,
            ) => Some("r1"),
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::RightTrigger2,
            ) => Some("r2"),
            (GamepadBrand::PlayStation3, GamepadButton::Select) => Some("select"),
            (GamepadBrand::PlayStation4, GamepadButton::Select) => Some("share"),
            (GamepadBrand::PlayStation5, GamepadButton::Select) => Some("create"),
            (GamepadBrand::PlayStation3, GamepadButton::Start) => Some("start"),
            (GamepadBrand::PlayStation4 | GamepadBrand::PlayStation5, GamepadButton::Start) => {
                Some("options")
            }
            (GamepadBrand::PlayStation3, GamepadButton::Mode) => None,
            (GamepadBrand::PlayStation4 | GamepadBrand::PlayStation5, GamepadButton::Mode) => {
                Some("touchpad")
            }
            (
                GamepadBrand::PlayStation3
                | GamepadBrand::PlayStation4
                | GamepadBrand::PlayStation5,
                GamepadButton::Other(_),
            ) => None,
            (GamepadBrand::SteamController, GamepadButton::South) => None,
            (GamepadBrand::SteamController, GamepadButton::East) => None,
            (GamepadBrand::SteamController, GamepadButton::North) => None,
            (GamepadBrand::SteamController, GamepadButton::West) => None,
            (GamepadBrand::SteamController, GamepadButton::C) => None,
            (GamepadBrand::SteamController, GamepadButton::Z) => None,
            (GamepadBrand::SteamController, GamepadButton::LeftTrigger) => None,
            (GamepadBrand::SteamController, GamepadButton::LeftTrigger2) => None,
            (GamepadBrand::SteamController, GamepadButton::RightTrigger) => None,
            (GamepadBrand::SteamController, GamepadButton::RightTrigger2) => None,
            (GamepadBrand::SteamController, GamepadButton::Select) => None,
            (GamepadBrand::SteamController, GamepadButton::Start) => None,
            (GamepadBrand::SteamController, GamepadButton::Mode) => None,
            (GamepadBrand::SteamController, GamepadButton::Other(_)) => None,
            (GamepadBrand::SteamDeck, GamepadButton::South) => None,
            (GamepadBrand::SteamDeck, GamepadButton::East) => None,
            (GamepadBrand::SteamDeck, GamepadButton::North) => None,
            (GamepadBrand::SteamDeck, GamepadButton::West) => None,
            (GamepadBrand::SteamDeck, GamepadButton::C) => None,
            (GamepadBrand::SteamDeck, GamepadButton::Z) => None,
            (GamepadBrand::SteamDeck, GamepadButton::LeftTrigger) => None,
            (GamepadBrand::SteamDeck, GamepadButton::LeftTrigger2) => None,
            (GamepadBrand::SteamDeck, GamepadButton::RightTrigger) => None,
            (GamepadBrand::SteamDeck, GamepadButton::RightTrigger2) => None,
            (GamepadBrand::SteamDeck, GamepadButton::Select) => None,
            (GamepadBrand::SteamDeck, GamepadButton::Start) => None,
            (GamepadBrand::SteamDeck, GamepadButton::Mode) => None,
            (GamepadBrand::SteamDeck, GamepadButton::Other(_)) => None,
            (GamepadBrand::XboxSeries, GamepadButton::South) => Some("a"),
            (GamepadBrand::XboxSeries, GamepadButton::East) => Some("b"),
            (GamepadBrand::XboxSeries, GamepadButton::North) => Some("y"),
            (GamepadBrand::XboxSeries, GamepadButton::West) => Some("x"),
            (GamepadBrand::XboxSeries, GamepadButton::C) => None,
            (GamepadBrand::XboxSeries, GamepadButton::Z) => None,
            (GamepadBrand::XboxSeries, GamepadButton::LeftTrigger) => Some("lb"),
            (GamepadBrand::XboxSeries, GamepadButton::LeftTrigger2) => Some("lt"),
            (GamepadBrand::XboxSeries, GamepadButton::RightTrigger) => Some("rb"),
            (GamepadBrand::XboxSeries, GamepadButton::RightTrigger2) => Some("rt"),
            (GamepadBrand::XboxSeries, GamepadButton::Select) => Some("view"),
            (GamepadBrand::XboxSeries, GamepadButton::Start) => Some("menu"),
            (GamepadBrand::XboxSeries, GamepadButton::Mode) => Some("guide"),
            (GamepadBrand::XboxSeries, GamepadButton::Other(_)) => None,
        }
    }
}
