pub mod gamepad_button;
pub mod key_code;
pub mod mouse_button;

#[derive(Default, Clone, Copy, Debug)]
pub enum Format {
    /// 64x64 pixels
    #[default]
    Default,
    /// 128 x 128 pixels
    Double,
    /// SVG
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
            GamepadBrand::PlayStation => "PlayStation Series",
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

/// settings to configure a Kenney keyboard and mouse input prompt
#[derive(Default, Clone, Copy, Debug)]
pub struct KenneyKeyboardAndMouseSettings {
    /// outline icons
    pub outline: bool,
    /// icon format
    pub format: Format,
}

/// settings to configure a Kenney gamepad input prompt
#[derive(Clone, Copy, Debug)]
pub struct KenneyGamepadSettings {
    /// round icons if possible, only for xbox dpad
    pub round_if_possible: bool,
    /// outline icons if possible, available for most icons except sticks
    pub outline_if_possible: bool,
    /// icon format
    pub format: Format,
    /// color icons if possible, only for xbox series A B X Y, steam controller A B X Y and playstation cross circle square triangle
    pub color_if_possible: bool,
    /// brand of gamepad
    pub gamepad_brand: GamepadBrand,
}

impl Default for KenneyGamepadSettings {
    fn default() -> Self {
        Self {
            round_if_possible: false,
            outline_if_possible: false,
            format: Default::default(),
            color_if_possible: true,
            gamepad_brand: Default::default(),
        }
    }
}

pub const ASSET_DIRS: [&'static str; 1] = ["bevy_input_prompts/kenney"];
