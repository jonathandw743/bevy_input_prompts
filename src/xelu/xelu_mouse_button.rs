use bevy_asset::AssetPath;
use bevy_input::mouse::MouseButton;

use super::{xelu_mouse_button_name::mouse_button_name, LightDark};

pub struct XeluMouseButton {
    mouse_button: MouseButton,
    light_dark: LightDark,
}

impl XeluMouseButton {
    fn default_path(&self) -> &'static str {
        match self.light_dark {
            LightDark::Dark => {
                "Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Blanks/Blank_Black_Mouse.png"
            }
            LightDark::Light => {
                "Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Blanks/Blank_White_Mouse.png"
            }
        }
    }
    fn format_mouse_button_name(&self, key_name: &str) -> String {
        match self.light_dark {
            LightDark::Light => format!("Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Light/{}_Key_Light.png", key_name),
            LightDark::Dark => format!("Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Dark/{}_Key_Dark.png", key_name),
        }
    }
}

impl<'a> Into<AssetPath<'a>> for XeluMouseButton {
    fn into(self) -> AssetPath<'a> {
        match mouse_button_name(self.mouse_button) {
            Some(key_name) => self.format_mouse_button_name(key_name).into(),
            None => self.default_path().into(),
        }
    }
}