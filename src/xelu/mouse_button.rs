use bevy_asset::AssetPath;
use bevy_input::mouse::MouseButton;

use super::LightDark;

pub struct XeluMouseButton {
    pub mouse_button: MouseButton,
    pub light_dark: LightDark,
}

impl XeluMouseButton {
    pub fn default_path(&self) -> &'static str {
        match self.light_dark {
            LightDark::Dark => {
                "unknown.png"
                // "xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Blanks/Blank_Black_Mouse.png"
            }
            LightDark::Light => {
                "unknown.png"
                // "xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Blanks/Blank_White_Mouse.png"
            }
        }
    }
    pub fn format_mouse_button_name(&self, key_name: &str) -> String {
        match self.light_dark {
            LightDark::Light => format!(
                "xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Light/{}_Key_Light.png",
                key_name
            ),
            LightDark::Dark => format!(
                "xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Dark/{}_Key_Dark.png",
                key_name
            ),
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

pub fn mouse_button_name(mouse_button: MouseButton) -> Option<&'static str> {
    match mouse_button {
        MouseButton::Left => Some("Mouse_Left"),
        MouseButton::Right => Some("Mouse_Right"),
        MouseButton::Middle => Some("Mouse_Middle"),
        MouseButton::Back | MouseButton::Forward | MouseButton::Other(_) => None,
    }
}
