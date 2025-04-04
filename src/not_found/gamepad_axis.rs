use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadAxis;

#[derive(Clone, Debug)]
pub struct NotFoundGamepadAxis {
    pub gamepad_axis: GamepadAxis,
}

impl<'a> Into<AssetPath<'a>> for NotFoundGamepadAxis {
    fn into(self) -> AssetPath<'a> {
        format!(
            "bevy_input_prompts/not_found/GamepadButton/{}.png",
            self.gamepad_axis_name(),
        )
        .into()
    }
}

impl NotFoundGamepadAxis {
    pub fn gamepad_axis_name(&self) -> String {
        match self.gamepad_axis {
            GamepadAxis::Other(_) => "Other".into(),
            gamepad_axis => format!("{:?}", gamepad_axis),
        }
    }
}
