pub mod key_code;
pub mod mouse_button;
pub mod gamepad_button;

use super::copy_assets;

pub enum LightDark {
    Light,
    Dark,
}

pub enum GamepadBrand {
    PS5,
    SteamDeck,
    Switch,
    XboxSeries,
}

pub const ASSET_DIRS: [&'static str; 1] = ["xelu"];

pub fn build() {
    copy_assets(ASSET_DIRS);
}
