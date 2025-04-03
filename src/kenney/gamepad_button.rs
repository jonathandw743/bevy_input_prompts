use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadButton;

use super::{GamepadBrand, KenneyGamepadSettings};

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
                "bevy_input_prompts/kenney/kenney_input-prompts/{}/{}/{}{}{}{}{}_{}{}.{}",
                self.settings.gamepad_brand.directory(),
                self.settings.format.directiory(),
                self.settings.gamepad_brand.prefix(),
                self.gamepad_brand_prefix_extra(),
                buttont_type,
                self.round_name(),
                self.color_name(),
                gamepad_button_name,
                self.outline_name(),
                self.settings.format.extension()
            )
            .into()
        } else {
            "bevy_input_prompts/unknown.png".into()
        }
    }
}

impl KenneyGamepadButton {
    pub fn color_possible(&self) -> bool {
        match self.settings.gamepad_brand {
            GamepadBrand::SteamController
            | GamepadBrand::PlayStation
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
        match self.gamepad_button {
            GamepadButton::Select
            | GamepadButton::Start
            | GamepadButton::South
            | GamepadButton::East
            | GamepadButton::North
            | GamepadButton::West => Some("_button"),
            GamepadButton::LeftTrigger
            | GamepadButton::LeftTrigger2
            | GamepadButton::RightTrigger
            | GamepadButton::RightTrigger2 => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "_trigger",
                GamepadBrand::SteamController => todo!(),
                GamepadBrand::SteamDeck => "_button",
                GamepadBrand::XboxSeries => "",
            }),
            GamepadButton::Mode => Some(""),
            GamepadButton::LeftThumb | GamepadButton::RightThumb => Some("_stick"),
            GamepadButton::DPadUp
            | GamepadButton::DPadDown
            | GamepadButton::DPadLeft
            | GamepadButton::DPadRight => Some("_dpad"),
            GamepadButton::C | GamepadButton::Z | GamepadButton::Other(_) => None,
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
            | GamepadBrand::PlayStation
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
    pub fn gamepad_brand_prefix_extra(&self) -> &'static str {
        match self.settings.gamepad_brand {
            GamepadBrand::PlayStation => {},
            _ => {
                return "";
            }
        }
        match self.gamepad_button {
            GamepadButton::Start | GamepadButton::Select => "3",
            GamepadButton::Mode => "5",
            _ => ""
        }
    }
    pub fn gamepad_button_name(&self) -> Option<&'static str> {
        match self.gamepad_button {
            GamepadButton::South => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "cross",
                GamepadBrand::SteamController => todo!(),
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "a",
            }),
            GamepadButton::East => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "circle",
                GamepadBrand::SteamController => todo!(),
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "b",
            }),
            GamepadButton::North => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "triangle",
                GamepadBrand::SteamController => todo!(),
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "y",
            }),
            GamepadButton::West => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "square",
                GamepadBrand::SteamController => todo!(),
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "x",
            }),
            GamepadButton::LeftTrigger => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "l1",
                GamepadBrand::SteamController => "l1",
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "lb",
            }),
            GamepadButton::LeftTrigger2 => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "l2",
                GamepadBrand::SteamController => "l2",
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "lt",
            }),
            GamepadButton::RightTrigger => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "r1",
                GamepadBrand::SteamController => "r1",
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "rb",
            }),
            GamepadButton::RightTrigger2 => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "r2",
                GamepadBrand::SteamController => "r2",
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "rb",
            }),
            GamepadButton::Select => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "select",
                GamepadBrand::SteamController => todo!(),
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "view",
            }),
            GamepadButton::Start => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "start",
                GamepadBrand::SteamController => todo!(),
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "menu",
            }),
            GamepadButton::Mode => Some(match self.settings.gamepad_brand {
                GamepadBrand::Generic => todo!(),
                GamepadBrand::Switch => todo!(),
                GamepadBrand::Wii => todo!(),
                GamepadBrand::WiiU => todo!(),
                GamepadBrand::Playdate => todo!(),
                GamepadBrand::PlayStation => "touchpad",
                GamepadBrand::SteamController => todo!(),
                GamepadBrand::SteamDeck => todo!(),
                GamepadBrand::XboxSeries => "guide",
            }),
            GamepadButton::LeftThumb => Some("side_l"),
            GamepadButton::RightThumb => Some("side_r"),
            GamepadButton::DPadUp => Some("up"),
            GamepadButton::DPadDown => Some("down"),
            GamepadButton::DPadLeft => Some("left"),
            GamepadButton::DPadRight => Some("right"),

            GamepadButton::C | GamepadButton::Z | GamepadButton::Other(_) => None,
        }
    }
}
