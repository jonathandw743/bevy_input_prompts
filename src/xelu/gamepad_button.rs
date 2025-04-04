use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadButton;

use super::{GamepadBrand, XeluGamepadSettings};

#[derive(Clone, Debug)]
pub struct XeluGamepadButton {
    pub gamepad_button: GamepadButton,
    pub settings: XeluGamepadSettings,
}

impl<'a> Into<AssetPath<'a>> for XeluGamepadButton {
    fn into(self) -> AssetPath<'a> {
        let Some(gamepad_button_name) = self.gamepad_button_name() else {
            return "bevy_input_prompts/unknown.png".into();
        };
        format!(
            "bevy_input_prompts/xelu/Xelu_Free_Controller&Key_Prompts/{}/{}_{}.png",
            self.settings.gamepad_brand.directory(),
            self.settings.gamepad_brand.prefix(),
            gamepad_button_name
        )
        .into()
    }
}

impl XeluGamepadButton {
    pub fn gamepad_button_name(&self) -> Option<&'static str> {
        match (self.settings.gamepad_brand, self.gamepad_button) {
            (GamepadBrand::PS5, GamepadButton::South) => Some("Cross"),
            (GamepadBrand::PS5, GamepadButton::East) => Some("Circle"),
            (GamepadBrand::PS5, GamepadButton::North) => Some("Triangle"),
            (GamepadBrand::PS5, GamepadButton::West) => Some("Square"),

            (
                GamepadBrand::SteamDeck | GamepadBrand::Switch | GamepadBrand::XboxSeries,
                GamepadButton::South,
            ) => Some("A"),
            (
                GamepadBrand::SteamDeck | GamepadBrand::Switch | GamepadBrand::XboxSeries,
                GamepadButton::East,
            ) => Some("B"),
            (
                GamepadBrand::SteamDeck | GamepadBrand::Switch | GamepadBrand::XboxSeries,
                GamepadButton::North,
            ) => Some("Y"),
            (
                GamepadBrand::SteamDeck | GamepadBrand::Switch | GamepadBrand::XboxSeries,
                GamepadButton::West,
            ) => Some("X"),

            (GamepadBrand::PS5 | GamepadBrand::SteamDeck, GamepadButton::LeftTrigger) => Some("L1"),
            (GamepadBrand::PS5 | GamepadBrand::SteamDeck, GamepadButton::RightTrigger) => {
                Some("R1")
            }

            (GamepadBrand::PS5 | GamepadBrand::SteamDeck, GamepadButton::LeftTrigger2) => {
                Some("L2")
            }
            (GamepadBrand::PS5 | GamepadBrand::SteamDeck, GamepadButton::RightTrigger2) => {
                Some("R2")
            }

            (GamepadBrand::XboxSeries | GamepadBrand::Switch, GamepadButton::LeftTrigger) => {
                Some("LB")
            }
            (GamepadBrand::XboxSeries | GamepadBrand::Switch, GamepadButton::RightTrigger) => {
                Some("RB")
            }

            (GamepadBrand::XboxSeries | GamepadBrand::Switch, GamepadButton::LeftTrigger2) => {
                Some("LT")
            }
            (GamepadBrand::XboxSeries | GamepadBrand::Switch, GamepadButton::RightTrigger2) => {
                Some("RT")
            }

            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadButton::LeftThumb,
            ) => Some("Left_Stick_Click"),
            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadButton::RightThumb,
            ) => Some("Right_Stick_Click"),

            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadButton::DPadUp,
            ) => Some("Dpad_Up"),
            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadButton::DPadDown,
            ) => Some("Dpad_Down"),
            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadButton::DPadLeft,
            ) => Some("Dpad_Left"),
            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadButton::DPadRight,
            ) => Some("Dpad_Right"),

            (GamepadBrand::PS5, GamepadButton::Select) => Some("Share"),
            (GamepadBrand::PS5, GamepadButton::Start) => Some("Options"),
            (GamepadBrand::SteamDeck, GamepadButton::Select) => Some("Inventory"),
            (GamepadBrand::SteamDeck, GamepadButton::Start) => Some("Menu"),
            (GamepadBrand::Switch, GamepadButton::Select) => Some("Square"),
            (GamepadBrand::Switch, GamepadButton::Start) => Some("Home"),
            (GamepadBrand::XboxSeries, GamepadButton::Select) => Some("View"),
            (GamepadBrand::XboxSeries, GamepadButton::Start) => Some("Menu"),

            (
                GamepadBrand::PS5
                | GamepadBrand::SteamDeck
                | GamepadBrand::Switch
                | GamepadBrand::XboxSeries,
                GamepadButton::C | GamepadButton::Z | GamepadButton::Mode | GamepadButton::Other(_),
            ) => None,
        }
    }
}
