use bevy_asset::AssetPath;
use bevy_input::mouse::MouseButton;

use super::KenneyKeyboardAndMouseSettings;

#[derive(Clone, Debug)]
pub struct KenneyMouseButton {
    pub mouse_button: MouseButton,
    pub settings: KenneyKeyboardAndMouseSettings,
}

impl<'a> Into<AssetPath<'a>> for KenneyMouseButton {
    fn into(self) -> AssetPath<'a> {
        let Some(mouse_button_name) = self.mouse_button_name() else {
            return "bevy_input_prompts/unknown.png".into();
        };
        format!(
            "bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/{}/mouse_{}{}.{}",
            self.settings.format.directiory(),
            mouse_button_name,
            self.outline_name(),
            self.settings.format.extension()
        )
        .into()
    }
}

impl KenneyMouseButton {
    pub fn outline_name(&self) -> &'static str {
        if self.settings.outline {
            "_outline"
        } else {
            ""
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
