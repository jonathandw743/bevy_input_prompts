pub mod mouse_button;
pub mod key_code;

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