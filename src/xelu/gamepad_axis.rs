use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadAxis;

use crate::not_found::gamepad_axis::NotFoundGamepadAxis;

use super::{GamepadBrand, XeluGamepadSettings};

/// converts to a Xelu's input prompt representing a [`GamepadAxis`]
#[derive(Clone, Debug)]
pub struct XeluGamepadAxis {
    pub gamepad_axis: GamepadAxis,
    pub settings: XeluGamepadSettings,
}

impl<'a> Into<AssetPath<'a>> for XeluGamepadAxis {
    fn into(self) -> AssetPath<'a> {
        let Some(gamepad_axis_name) = self.gamepad_axis_name() else {
            return NotFoundGamepadAxis {
                gamepad_axis: self.gamepad_axis,
            }
            .into();
        };
        format!(
            "bevy_input_prompts/xelu/Xelu_Free_Controller&Key_Prompts/{}/{}_{}.png",
            self.settings.gamepad_brand.directory(),
            self.settings.gamepad_brand.prefix(),
            gamepad_axis_name
        )
        .into()
    }
}

impl XeluGamepadAxis {
    pub fn gamepad_axis_name(&self) -> Option<&'static str> {
        match (self.settings.gamepad_brand, self.gamepad_axis) {
            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadAxis::LeftStickX | GamepadAxis::LeftStickY,
            ) => Some("Left_Stick"),
            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadAxis::RightStickX | GamepadAxis::RightStickY,
            ) => Some("Right_Stick"),
            _ => None,
        }
    }
}
