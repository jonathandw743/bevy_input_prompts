pub mod gamepad_button;
pub mod key_code;
pub mod mouse_button;

#[derive(Default, Clone, Copy, Debug)]
pub enum Format {
    #[default]
    Default,
    Double,
    Vector,
}

impl Format {
    pub fn directiory(&self) -> &'static str {
        match self {
            Format::Default => "Default",
            Format::Double => "Double",
            Format::Vector => "Vector",
        }
    }
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Default => "png",
            Format::Double => "png",
            Format::Vector => "svg",
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum GamepadBrand {
    Generic,
    Switch,
    Wii,
    WiiU,
    Playdate,
    PlayStation,
    SteamController,
    SteamDeck,
    #[default]
    XboxSeries,
}

impl GamepadBrand {
    pub fn directory(&self) -> &'static str {
        match self {
            GamepadBrand::Generic => "Generic",
            GamepadBrand::Switch => "Nintendo Switch",
            GamepadBrand::Wii => "Nintendo Wii",
            GamepadBrand::WiiU => "Nintendo WiiU",
            GamepadBrand::Playdate => "Playdate",
            GamepadBrand::PlayStation => "Playstation Series",
            GamepadBrand::SteamController => "Steam Controller",
            GamepadBrand::SteamDeck => "Steam Deck",
            GamepadBrand::XboxSeries => "Xbox Series",
        }
    }
    pub fn prefix(&self) -> &'static str {
        match self {
            GamepadBrand::Generic => "generic",
            GamepadBrand::Switch => "switch",
            GamepadBrand::Wii => "wii",
            GamepadBrand::WiiU => "wiiu",
            GamepadBrand::Playdate => "playdate",
            GamepadBrand::PlayStation => "playstation",
            GamepadBrand::SteamController => "steam",
            GamepadBrand::SteamDeck => "steamdeck",
            GamepadBrand::XboxSeries => "xbox",
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct KenneyKeyboardAndMouseSettings {
    pub outline: bool,
    pub format: Format,
}

#[derive(Clone, Copy, Debug)]
pub struct KenneyGamepadSettings {
    pub outline_if_bossible: bool,
    pub format: Format,
    // only for xbox, steam deck and playstation
    pub color_if_possible: bool,
    pub gamepad_brand: GamepadBrand,
}

impl Default for KenneyGamepadSettings {
    fn default() -> Self {
        Self {
            outline_if_bossible: Default::default(),
            format: Default::default(),
            color_if_possible: true,
            gamepad_brand: Default::default(),
        }
    }
}

pub const ASSET_DIRS: [&'static str; 1] = ["kenney"];
