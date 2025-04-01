use bevy_asset::AssetPath;
use bevy_input::mouse::MouseButton;

use super::{FilledOutline, Format, KennySettings};

#[derive(Clone, Debug)]
pub struct KennyMouseButton {
    pub mouse_button: MouseButton,
    pub settings: KennySettings,
}

impl<'a> Into<AssetPath<'a>> for KennyMouseButton {
    fn into(self) -> AssetPath<'a> {
        let format_name = self.format_name();
        let Some(mouse_button_name) = self.mouse_button_name() else {
            return "unknown.png".into();
        };
        let filled_outline_name = self.filled_outline_name();
        format!(
            "kenny/kenney_input-prompts/Keyboard & Mouse/{}/mouse_{}{}.png",
            format_name, mouse_button_name, filled_outline_name,
        )
        .into()
    }
}

impl KennyMouseButton {
    pub fn format_name(&self) -> &'static str {
        match self.settings.format {
            Format::Default => "Default",
            Format::Double => "Double",
            Format::Vector => "Vector",
        }
    }
    pub fn filled_outline_name(&self) -> &'static str {
        match self.settings.filled_outline {
            FilledOutline::Filled => "",
            FilledOutline::Outline => "_outline",
        }
    }
    pub fn mouse_button_name(&self) -> Option<&'static str> {
        match self.mouse_button {
            MouseButton::Left => Some("left"),
            MouseButton::Right => Some("right"),
            MouseButton::Middle => Some("scroll"),
            MouseButton::Back | MouseButton::Forward | MouseButton::Other(_) => None,
        }
    }
}
