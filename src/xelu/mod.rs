use crate::vendor_ids::*;
use bevy_input::gamepad::Gamepad;

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

impl GamepadBrand {
    pub fn directory(&self) -> &'static str {
        match self {
            GamepadBrand::PS5 => "PS5",
            GamepadBrand::SteamDeck => "Steam Deck",
            GamepadBrand::Switch => "Switch",
            GamepadBrand::XboxSeries => "Xbox Series",
        }
    }
    pub fn prefix(&self) -> &'static str {
        match self {
            GamepadBrand::PS5 => "PS5",
            GamepadBrand::SteamDeck => "SteamDeck",
            GamepadBrand::Switch => "Switch",
            GamepadBrand::XboxSeries => "XboxSeriesX",
        }
    }
}

/// settings to configure a Xelu gamepad input prompt
#[derive(Default, Clone, Copy, Debug)]
pub struct XeluGamepadSettings {
    pub gamepad_brand: GamepadBrand,
}

/// settings to configure a Xelu heyboard and mouse input prompt
#[derive(Default, Clone, Copy, Debug)]
pub struct XeluKeyboardAndMouseSettings {
    pub light_dark: LightDark,
}

pub const ASSET_DIRS: [&'static str; 1] = ["bevy_input_prompts/xelu"];

impl GamepadBrand {
    pub fn from_vendor_id(vendor_id: u16) -> Option<Self> {
        if vendor_id == XBOX || vendor_id == XBOX_THIRD_PARTY {
            return Some(Self::XboxSeries);
        }
        // this will also match steam controllers but this is an ok estimation
        if vendor_id == VALVE {
            return Some(Self::SteamDeck);
        }
        if vendor_id == SONY_COMPUTER_ENTERTAINMENT_AMERICA
            || vendor_id == SONY_COMPUTER_ENTERTAINMENT_EUROPE
            || vendor_id == SONY_CORPORATION
            || vendor_id == SONY_MOBILE_COMMUNICATIONS
        {
            return Some(Self::PS5);
        }
        if vendor_id == NINTENDO_CO_LTD || vendor_id == NINTENDO_OF_AMERICA {
            return Some(Self::Switch);
        }
        return None;
    }
    pub fn from_gamepad(gamepad: &Gamepad) -> Option<Self> {
        if let Some(vendor_id) = gamepad.vendor_id() {
            if let Some(gamepad_brand) = Self::from_vendor_id(vendor_id) {
                return Some(gamepad_brand);
            }
        }
        return None;
    }
}
