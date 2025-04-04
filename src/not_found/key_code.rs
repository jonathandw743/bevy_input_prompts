use bevy_asset::AssetPath;
use bevy_input::keyboard::KeyCode;

#[derive(Clone, Debug)]
pub struct NotFoundKeyCode {
    pub key_code: KeyCode,
}

impl<'a> Into<AssetPath<'a>> for NotFoundKeyCode {
    fn into(self) -> AssetPath<'a> {
        format!(
            "bevy_input_prompts/not_found/KeyCode/{}.png",
            self.key_code_name(),
        )
        .into()
    }
}

impl NotFoundKeyCode {
    pub fn key_code_name(&self) -> String {
        match self.key_code {
            KeyCode::Unidentified(_) => "Unidentified".into(),
            key_code => format!("{:?}", key_code),
        }
    }
}
