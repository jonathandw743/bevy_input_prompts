use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadButton;

#[derive(Clone, Debug)]
pub struct NotFoundGamepadButton {
    pub gamepad_button: GamepadButton,
}

impl<'a> Into<AssetPath<'a>> for NotFoundGamepadButton {
    fn into(self) -> AssetPath<'a> {
        format!(
            "bevy_input_prompts/not_found/GamepadButton/{}.png",
            self.gamepad_button_name(),
        )
        .into()
    }
}

impl NotFoundGamepadButton {
    pub fn gamepad_button_name(&self) -> String {
        match self.gamepad_button {
            GamepadButton::Other(_) => "Other".into(),
            gamepad_button => format!("{:?}", gamepad_button),
        }
    }
}
