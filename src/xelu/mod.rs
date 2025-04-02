pub mod gamepad_axis;
pub mod gamepad_button;
pub mod key_code;
pub mod mouse_button;

#[derive(Default, Clone, Copy, Debug)]
pub enum LightDark {
    Light,
    #[default]
    Dark,
}

impl LightDark {
    pub fn directory(&self) -> &'static str {
        match self {
            LightDark::Light => "Light",
            LightDark::Dark => "Dark",
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum GamepadBrand {
    PS5,
    SteamDeck,
    Switch,
    #[default]
    XboxSeries,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct XeluGamepadSettings {
    pub gamepad_brand: GamepadBrand,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct XeluKeyboardAndMouseSettings {
    pub light_dark: LightDark,
}

pub const ASSET_DIRS: [&'static str; 1] = ["xelu"];
