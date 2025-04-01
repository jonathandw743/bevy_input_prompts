pub mod key_code;
pub mod mouse_button;
pub mod gamepad_button;
pub mod gamepad_axis;

#[derive(Default, Clone, Copy, Debug)]
pub enum LightDark {
    Light,
    #[default]
    Dark,
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