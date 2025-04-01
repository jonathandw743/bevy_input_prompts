pub mod key_code;
pub mod mouse_button;

use super::copy_assets;

pub enum LightDark {
    Light,
    Dark,
}

pub const ASSET_DIRS: [&'static str; 1] = ["xelu"];

pub fn build() {
    copy_assets(ASSET_DIRS);
}
