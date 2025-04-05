use crate::vendor_ids::*;
use crate::product_ids::*;
use bevy_input::gamepad::Gamepad;

pub mod gamepad_button;
pub mod gamepad_axis;
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
    PlayStation3,
    PlayStation4,
    PlayStation5,
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
            GamepadBrand::PlayStation3
            | GamepadBrand::PlayStation4
            | GamepadBrand::PlayStation5 => "PlayStation Series",
            GamepadBrand::SteamController => "Steam Controller",
            GamepadBrand::SteamDeck => "Steam Deck",
            GamepadBrand::XboxSeries => "Xbox Series",
        }
    }
}

/// settings to configure a Kenney keyboard and mouse input prompt
#[derive(Default, Clone, Copy, Debug)]
pub struct KenneyKeyboardAndMouseSettings {
    /// outline prompt
    pub outline: bool,
    /// prompt format
    pub format: Format,
    /// use icon version if possible, for example a `->|` icon instead of `TAB`
    pub icon_if_possible: bool,
    /// use prompt with multiple arrows and one highlighted instead of a single arrow prompt if possible
    pub arrows_if_possible: bool,
}

/// settings to configure a Kenney gamepad input prompt
#[derive(Clone, Copy, Debug)]
pub struct KenneyGamepadSettings {
    /// round icons if possible, only for xbox dpad
    /// xbox dpad can either be round or outlined, `round_if_possible` will override `outlineif_possible` in this case 
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

impl GamepadBrand {
    pub fn from_vendor_id(vendor_id: u16) -> Option<Self> {
        if vendor_id == XBOX || vendor_id == XBOX_THIRD_PARTY {
            return Some(Self::XboxSeries);
        }
        if vendor_id == VALVE {
            return Some(Self::SteamDeck);
        }
        if vendor_id == SONY_COMPUTER_ENTERTAINMENT_AMERICA
            || vendor_id == SONY_COMPUTER_ENTERTAINMENT_EUROPE
            || vendor_id == SONY_CORPORATION
            || vendor_id == SONY_MOBILE_COMMUNICATIONS
        {
            return Some(Self::PlayStation5);
        }
        if vendor_id == NINTENDO_CO_LTD || vendor_id == NINTENDO_OF_AMERICA {
            return Some(Self::Switch);
        }
        return None;
    }
    pub fn from_product_id(product_id: u16) -> Option<Self> {
        if product_id == STEAM_CONTROLLER_0476
                || product_id == STEAM_CONTROLLER_1102
                || product_id == STEAM_CONTROLLER_1142
                || product_id == STEAM_CONTROLLER_11FC
                || product_id == STEAM_CONTROLLER_1201
                || product_id == STEAM_VIRTUAL_GAMEPAD
            {
                return Some(Self::SteamController);
            }
            if product_id == DUALSHOCK3_SIXAXIS || product_id == SPLIT_FISH_FRAG_FX {
                return Some(Self::PlayStation3)
            }
            if product_id == DUALSHOCK4_05C4 || product_id == STRIKE_PACK_FPS_DOMINATOR || product_id == DUALSHOCK4_09CC || product_id == DUALSHOCK4_USB_RECEIVER {
                return Some(Self::PlayStation4);
            }
            return None;
    }
    pub fn from_gamepad(gamepad: &Gamepad) -> Option<Self> {
        if let Some(product_id) = gamepad.product_id() {
            if let Some(gamepad_brand) = Self::from_product_id(product_id) {
                return Some(gamepad_brand);
            }
        }
        if let Some(vendor_id) = gamepad.vendor_id() {
            if let Some(gamepad_brand) = Self::from_vendor_id(vendor_id) {
                return Some(gamepad_brand);
            }
        }
        return None;
    }
}
