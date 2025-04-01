pub mod mouse_button;
pub mod key_code;

use super::copy_assets;

pub enum FilledOutline {
    Filled,
    Outline,
}

pub enum Format {
    Default,
    Double,
    Vector,
}

pub const ASSET_DIRS: [&'static str; 1] = ["kenny"];

pub fn build() {
    copy_assets(ASSET_DIRS);
}
