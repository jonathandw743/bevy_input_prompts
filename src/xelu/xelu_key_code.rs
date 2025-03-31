use bevy_asset::AssetPath;
use bevy_input::keyboard::KeyCode;

use super::{xelu_key_code_name::key_code_name, LightDark};

pub struct XeluKeyCode {
    key_code: KeyCode,
    light_dark: LightDark,
}

impl XeluKeyCode {
    fn default_path(&self) -> &'static str {
        match self.light_dark {
            LightDark::Dark => {
                "Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Blanks/Blank_Black_Normal.png"
            }
            LightDark::Light => {
                "Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Blanks/Blank_White_Normal.png"
            }
        }
    }
    fn format_key_name(&self, key_name: &str) -> String {
        match self.light_dark {
            LightDark::Light => format!("Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Light/{}_Key_Light.png", key_name),
            LightDark::Dark => format!("Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Dark/{}_Key_Dark.png", key_name),
        }
    }
}

impl<'a> Into<AssetPath<'a>> for XeluKeyCode {
    fn into(self) -> AssetPath<'a> {
        match key_code_name(self.key_code) {
            Some(key_name) => self.format_key_name(key_name).into(),
            None => self.default_path().into(),
        }
    }
}