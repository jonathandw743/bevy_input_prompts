use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadAxis;

use super::GamepadBrand;

pub struct XeluGamepadAxis {
    pub gamepad_axis: GamepadAxis,
    pub gamepad_brand: GamepadBrand,
}

impl<'a> Into<AssetPath<'a>> for XeluGamepadAxis {
    fn into(self) -> AssetPath<'a> {
        let path = match self.gamepad_brand {
            GamepadBrand::PS5 => {
                let gamepad_axis_name = match self.gamepad_axis {
                    GamepadAxis::LeftStickX => "Left_Stick",
                    GamepadAxis::LeftStickY => "Left_Stick",
                    GamepadAxis::RightStickX => "Right_Stick",
                    GamepadAxis::RightStickY => "Right_Stick",
                    GamepadAxis::LeftZ => "L2",
                    GamepadAxis::RightZ => "R2",

                    GamepadAxis::Other { .. } => {
                        return "unknown.png".into();
                    }
                };
                format!("PS5/PS5_{}.png", gamepad_axis_name)
            }
            GamepadBrand::SteamDeck => {
                let gamepad_axis_name = match self.gamepad_axis {
                    GamepadAxis::LeftStickX => "Left_Stick",
                    GamepadAxis::LeftStickY => "Left_Stick",
                    GamepadAxis::RightStickX => "Right_Stick",
                    GamepadAxis::RightStickY => "Right_Stick",
                    GamepadAxis::LeftZ => "L2",
                    GamepadAxis::RightZ => "R2",

                    GamepadAxis::Other { .. } => {
                        return "unknown.png".into();
                    }
                };
                format!("Steam Deck/SteamDeck_{}.png", gamepad_axis_name)
            }
            GamepadBrand::Switch => {
                return "unknown.png".into();
            }
            GamepadBrand::XboxSeries => {
                let gamepad_axis_name = match self.gamepad_axis {
                    GamepadAxis::LeftStickX => "Left_Stick",
                    GamepadAxis::LeftStickY => "Left_Stick",
                    GamepadAxis::RightStickX => "Right_Stick",
                    GamepadAxis::RightStickY => "Right_Stick",
                    GamepadAxis::LeftZ => "LT",
                    GamepadAxis::RightZ => "RT",

                    GamepadAxis::Other { .. } => {
                        return "unknown.png".into();
                    }
                };
                format!("Xbox Series/XboxSeriesX_{}.png", gamepad_axis_name)
            }
        };
        format!("xelu/Xelu_Free_Controller&Key_Prompts/{}", path).into()
    }
}
