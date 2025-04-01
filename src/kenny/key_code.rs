use bevy_asset::AssetPath;
use bevy_input::keyboard::KeyCode;

use super::{FilledOutline, Format};

pub struct KennyKeyCode {
    pub key_code: KeyCode,
    pub filled_outline: FilledOutline,
    pub format: Format,
}

impl<'a> Into<AssetPath<'a>> for KennyKeyCode {
    fn into(self) -> AssetPath<'a> {
        let format_name = self.format_name();
        let Some(key_code_name) = self.key_code_name() else {
            return "unknown.png".into();
        };
        let filled_outline_name = self.filled_outline_name();
        format!(
            "kenny/kenney_input-prompts/Keyboard & Mouse/{}/mouse{}{}.png",
            format_name, key_code_name, filled_outline_name,
        )
        .into()
    }
}

impl KennyKeyCode {
    pub fn format_name(&self) -> &'static str {
        match self.format {
            Format::Default => "Default",
            Format::Double => "Double",
            Format::Vector => "Vector",
        }
    }
    pub fn filled_outline_name(&self) -> &'static str {
        match self.filled_outline {
            FilledOutline::Filled => "",
            FilledOutline::Outline => "_outline",
        }
    }
    pub fn key_code_name(&self) -> Option<&'static str> {
        match self.key_code {
            _ => todo!()
        }
    }
}
