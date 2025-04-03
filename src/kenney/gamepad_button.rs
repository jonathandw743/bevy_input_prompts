use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadButton;

use super::{GamepadBrand, KenneyGamepadSettings};

#[derive(Clone, Debug)]
struct KenneyGamepadButton {
    pub gamepad_button: GamepadButton,
    pub settings: KenneyGamepadSettings,
}

impl<'a> Into<AssetPath<'a>> for KenneyGamepadButton {
    fn into(self) -> AssetPath<'a> {
        if let Some(buttont_type) = self.button_type() {
            format!(
                "bevy_input_prompts/kenney/kenney_input-prompts/{}/{}/{}_{}{}_{}{}.{}",
                self.settings.gamepad_brand.directory(),
                self.settings.format.directiory(),
                self.settings.gamepad_brand.prefix(),
                buttont_type,
                self.color_name(),
                self.gamepad_button_name(),
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
            | GamepadButton::West => Some("button"),
            GamepadButton::LeftTrigger => Some("lb"),
            GamepadButton::LeftTrigger2 => Some("lt"),
            GamepadButton::RightTrigger => Some("rb"),
            GamepadButton::RightTrigger2 => Some("rt"),
            GamepadButton::Mode => Some("guide"),
            GamepadButton::LeftThumb | GamepadButton::RightThumb => Some("stick"),
            GamepadButton::DPadUp
            | GamepadButton::DPadDown
            | GamepadButton::DPadLeft
            | GamepadButton::DPadRight => Some("dpad"),
            GamepadButton::C | GamepadButton::Z | GamepadButton::Other(_) => None,
        }
    }
    pub fn outline_possible(&self) -> bool {
        match self.gamepad_button {
            GamepadButton::South => todo!(),
            GamepadButton::East => todo!(),
            GamepadButton::North => todo!(),
            GamepadButton::West => todo!(),
            GamepadButton::LeftTrigger => todo!(),
            GamepadButton::LeftTrigger2 => todo!(),
            GamepadButton::RightTrigger => todo!(),
            GamepadButton::RightTrigger2 => todo!(),
            GamepadButton::Select => todo!(),
            GamepadButton::Start => todo!(),
            GamepadButton::Mode => todo!(),
            GamepadButton::LeftThumb => todo!(),
            GamepadButton::RightThumb => todo!(),
            GamepadButton::DPadUp => todo!(),
            GamepadButton::DPadDown => todo!(),
            GamepadButton::DPadLeft => todo!(),
            GamepadButton::DPadRight => todo!(),

            GamepadButton::C => todo!(),
            GamepadButton::Z => todo!(),
            GamepadButton::Other(_) => todo!(),
        }
    }
    pub fn outline_name(&self) -> &'static str {
        if self.settings.outline_if_bossible && self.outline_possible() {
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
    pub fn gamepad_button_name(&self) -> &'static str {
        todo!()
    }
}
