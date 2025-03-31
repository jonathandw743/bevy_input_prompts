use bevy_asset::{embedded_asset, load_internal_asset, Asset, AssetPath, AssetServer, Handle};
use bevy_input::keyboard::KeyCode;

use super::{xelu_key_code_name::key_code_name, LightDark};

pub struct XeluKeyCode {
    pub key_code: KeyCode,
    pub light_dark: LightDark,
}

impl XeluKeyCode {
    pub fn default_path(&self) -> &'static str {
        match self.light_dark {
            LightDark::Dark => {
                "xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Blanks/Blank_Black_Normal.png"
            }
            LightDark::Light => {
                "xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Blanks/Blank_White_Normal.png"
            }
        }
    }
    pub fn format_key_name(&self, key_name: &str) -> String {
        match self.light_dark {
            LightDark::Light => format!("xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Light/{}_Key_Light.png", key_name),
            LightDark::Dark => format!("xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/Dark/{}_Key_Dark.png", key_name),
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