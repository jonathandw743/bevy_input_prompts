use bevy_asset::AssetPath;
use bevy_input::mouse::MouseButton;

use super::LightDark;

pub struct XeluMouseButton {
    pub mouse_button: MouseButton,
    pub light_dark: LightDark,
}

impl<'a> Into<AssetPath<'a>> for XeluMouseButton {
    fn into(self) -> AssetPath<'a> {
        let light_dark_name = self.light_dark_name();
        let Some(mouse_button_name) = self.mouse_button_name() else {
            return "unkown.png".into();
        };
        format!(
            "xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/{}/{}_Key_{}.png",
            light_dark_name, mouse_button_name, light_dark_name
        )
        .into()
    }
}

impl XeluMouseButton {
    pub fn light_dark_name(&self) -> &'static str {
        match self.light_dark {
            LightDark::Light => "Light",
            LightDark::Dark => "Dark",
        }
    }
    pub fn mouse_button_name(&self) -> Option<&'static str> {
        match self.mouse_button {
            MouseButton::Left => Some("Mouse_Left"),
            MouseButton::Right => Some("Mouse_Right"),
            MouseButton::Middle => Some("Mouse_Middle"),
            MouseButton::Back | MouseButton::Forward | MouseButton::Other(_) => None,
        }
    }
}
