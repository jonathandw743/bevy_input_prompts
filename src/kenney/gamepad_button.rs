use bevy_asset::AssetPath;
use bevy_input::gamepad::GamepadButton;

use super::KenneyGamepadSettings;

#[derive(Clone, Debug)]
struct KenneyGamepadButton {
    pub gamepad_button: GamepadButton,
    pub settings: KenneyGamepadSettings,
}

impl<'a> Into<AssetPath<'a>> for KenneyGamepadButton {
    fn into(self) -> AssetPath<'a> {
        format!(
            "kenney/kenney_input-prompts/{}/{}/{}_{}{}_{}{}.{}",
            self.settings.gamepad_brand.directory(),
            self.settings.format.directiory(),
            self.settings.gamepad_brand.prefix(),
            self.button_type(),
            self.color_name(),
            self.gamepad_button_name(),
            self.outline_name(),
            self.settings.format.extension()
        )
        .into()
    }
}

impl KenneyGamepadButton {
    pub fn color_possible(&self) -> bool {
        todo!()
    }
    pub fn color_name(&self) -> &'static str {
        if self.settings.color_if_possible && self.color_possible() {
            "_color"
        } else {
            ""
        }
    }
    pub fn button_type(&self) -> &'static str {
        todo!()
    }
    pub fn outline_possible(&self) -> bool {
        todo!()
    }
    pub fn outline_name(&self) -> &'static str {
        if self.settings.outline_if_bossible && self.outline_possible() {
            "_outline"
        } else {
            ""
        }
    }
    pub fn gamepad_button_name(&self) -> &'static str {
        todo!()
    }
}
