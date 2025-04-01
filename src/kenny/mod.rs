pub mod mouse_button;
pub mod key_code;

#[derive(Default, Clone, Copy, Debug)]
pub enum FilledOutline {
    #[default]
    Filled,
    Outline,
}

#[derive(Default, Clone, Copy, Debug)]
pub enum Format {
    #[default]
    Default,
    Double,
    Vector,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct KennySettings {
    pub filled_outline: FilledOutline,
    pub format: Format,
}

pub const ASSET_DIRS: [&'static str; 1] = ["kenny"];