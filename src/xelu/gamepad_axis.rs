use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadAxis;

use super::{GamepadBrand, XeluGamepadSettings};

#[derive(Clone, Debug)]
pub struct XeluGamepadAxis {
    pub gamepad_axis: GamepadAxis,
    pub settings: XeluGamepadSettings,
}

impl<'a> Into<AssetPath<'a>> for XeluGamepadAxis {
    fn into(self) -> AssetPath<'a> {
        let Some(gamepad_axis_name) = self.gamepad_axis_name() else {
            return "bevy_input_prompts/unknown.png".into();
        };
        format!(
            "bevy_input_prompts/xelu/Xelu_Free_Controller&Key_Prompts/{}/{}_{}.png",
            self.settings.gamepad_brand.directory(),
            self.settings.gamepad_brand.prefix(),
            gamepad_axis_name
        ).into()
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
            (GamepadBrand::PS5 | GamepadBrand::SteamDeck, GamepadAxis::LeftZ) => Some("L2"),
            (GamepadBrand::PS5 | GamepadBrand::SteamDeck, GamepadAxis::RightZ) => Some("R2"),
            (GamepadBrand::XboxSeries | GamepadBrand::Switch, GamepadAxis::LeftZ) => Some("LT"),
            (GamepadBrand::XboxSeries | GamepadBrand::Switch, GamepadAxis::RightZ) => Some("RT"),
            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadAxis::Other(_),
            ) => None,
        }
    }
}
