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
        let path = match self.settings.gamepad_brand {
            GamepadBrand::PS5 => {
                let gamepad_button_name = match self.gamepad_button {
                    GamepadButton::South => "Cross",
                    GamepadButton::East => "Circle",
                    GamepadButton::North => "Triangle",
                    GamepadButton::West => "Square",
                    GamepadButton::LeftTrigger => "L1",
                    GamepadButton::LeftTrigger2 => "L2",
                    GamepadButton::RightTrigger => "R1",
                    GamepadButton::RightTrigger2 => "R2",
                    GamepadButton::Select => "Share",
                    GamepadButton::Start => "Options",
                    GamepadButton::LeftThumb => "Left_Stick_Click",
                    GamepadButton::RightThumb => "Right_Stick_Click",
                    GamepadButton::DPadUp => "Dpad_Up",
                    GamepadButton::DPadDown => "Dpad_Down",
                    GamepadButton::DPadLeft => "Dpad_Left",
                    GamepadButton::DPadRight => "Dpad_Right",

                    GamepadButton::C
                    | GamepadButton::Z
                    | GamepadButton::Mode
                    | GamepadButton::Other { .. } => {
                        return "bevy_input_prompts/unknown.png".into()
                    },
                };
                format!(
                    "PS5/PS5_{}.png",
                    gamepad_button_name
                )
            },
            GamepadBrand::SteamDeck => {
                let gamepad_button_name = match self.gamepad_button {
                    GamepadButton::South => "A",
                    GamepadButton::East => "B",
                    GamepadButton::North => "Y",
                    GamepadButton::West => "X",
                    GamepadButton::LeftTrigger => "L1",
                    GamepadButton::LeftTrigger2 => "L2",
                    GamepadButton::RightTrigger => "R1",
                    GamepadButton::RightTrigger2 => "R2",
                    GamepadButton::Select => "Inventory",
                    GamepadButton::Start => "Menu",
                    GamepadButton::LeftThumb => "Left_Stick_Click",
                    GamepadButton::RightThumb => "Right_Stick_Click",
                    GamepadButton::DPadUp => "Dpad_Up",
                    GamepadButton::DPadDown => "Dpad_Down",
                    GamepadButton::DPadLeft => "Dpad_Left",
                    GamepadButton::DPadRight => "Dpad_Right",

                    GamepadButton::C
                    | GamepadButton::Z
                    | GamepadButton::Mode
                    | GamepadButton::Other { .. } => {
                        return "bevy_input_prompts/unknown.png".into()
                    },
                };
                format!(
                    "Steam Deck/SteamDeck_{}.png",
                    gamepad_button_name
                )
            },
            GamepadBrand::Switch => {
                let gamepad_button_name = match self.gamepad_button {
                    GamepadButton::South => "A",
                    GamepadButton::East => "B",
                    GamepadButton::North => "Y",
                    GamepadButton::West => "X",
                    GamepadButton::LeftTrigger => "LB",
                    GamepadButton::LeftTrigger2 => "LT",
                    GamepadButton::RightTrigger => "RB",
                    GamepadButton::RightTrigger2 => "RT",
                    GamepadButton::Select => "Square",
                    GamepadButton::Start => "Home",
                    GamepadButton::LeftThumb => "Left_Stick_Click",
                    GamepadButton::RightThumb => "Right_Stick_Click",
                    GamepadButton::DPadUp => "Dpad_Up",
                    GamepadButton::DPadDown => "Dpad_Down",
                    GamepadButton::DPadLeft => "Dpad_Left",
                    GamepadButton::DPadRight => "Dpad_Right",

                    GamepadButton::C
                    | GamepadButton::Z
                    | GamepadButton::Mode
                    | GamepadButton::Other { .. } => {
                        return "bevy_input_prompts/unknown.png".into()
                    },
                };
                format!(
                    "Switch/Switch_{}.png",
                    gamepad_button_name
                )
            },
            GamepadBrand::XboxSeries => {
                let gamepad_button_name = match self.gamepad_button {
                    GamepadButton::South => "A",
                    GamepadButton::East => "B",
                    GamepadButton::North => "Y",
                    GamepadButton::West => "X",
                    GamepadButton::LeftTrigger => "LB",
                    GamepadButton::LeftTrigger2 => "LT",
                    GamepadButton::RightTrigger => "RB",
                    GamepadButton::RightTrigger2 => "RT",
                    GamepadButton::Select => "View",
                    GamepadButton::Start => "Menu",
                    GamepadButton::LeftThumb => "Left_Stick_Click",
                    GamepadButton::RightThumb => "Right_Stick_Click",
                    GamepadButton::DPadUp => "Dpad_Up",
                    GamepadButton::DPadDown => "Dpad_Down",
                    GamepadButton::DPadLeft => "Dpad_Left",
                    GamepadButton::DPadRight => "Dpad_Right",

                    GamepadButton::C
                    | GamepadButton::Z
                    | GamepadButton::Mode
                    | GamepadButton::Other { .. } => {
                        return "bevy_input_prompts/unknown.png".into()
                    },
                };
                format!(
                    "Xbox Series/XboxSeriesX_{}.png",
                    gamepad_button_name
                )
            }
        };
        format!("bevy_input_prompts/xelu/Xelu_Free_Controller&Key_Prompts/{}", path).into()
    }
}
