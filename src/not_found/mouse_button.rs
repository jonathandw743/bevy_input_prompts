use bevy_asset::AssetPath;
use bevy_input::mouse::MouseButton;


#[derive(Clone, Debug)]
pub struct NotFoundMouseButton {
    pub mouse_button: MouseButton,
}

impl<'a> Into<AssetPath<'a>> for NotFoundMouseButton {
    fn into(self) -> AssetPath<'a> {
        format!(
            "bevy_input_prompts/not_found/MouseButton/{}.png",
            self.mouse_button_name()
        )
        .into()
    }
}

impl NotFoundMouseButton {
    pub fn mouse_button_name(&self) -> String {
        match self.mouse_button {
            MouseButton::Other(_) => "Other".into(),
            mouse_button => format!("{:?}", mouse_button),
        }
    }
}
