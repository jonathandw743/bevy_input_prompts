use bevy_input::gamepad::Gamepad;

use crate::product_ids::*;
use crate::vendor_ids::*;

#[derive(Clone, Copy)]
pub enum GamepadBrand {
    Generic,
    Gamecube,
    Switch,
    Switch2,
    Wii,
    WiiU,
    PS3,
    PS4,
    PS5,
    Playdate,
    SteamController,
    SteamDeck,
    Xbox,
}

impl GamepadBrand {
    pub fn from_vendor_id(vendor_id: u16) -> Option<Self> {
        if vendor_id == XBOX || vendor_id == XBOX_THIRD_PARTY {
            return Some(Self::Xbox);
        }
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
    pub fn from_product_id(product_id: u16) -> Option<Self> {
        // match product_id {
        //     GAME_CUBE_CONTROLLER_ADAPTER => Some(Self::Gamecube),
        //     STEAM_DECK => Some(Self::SteamDeck),
        //     DUALSHOCK3_SIXAXIS | SPLIT_FISH_FRAG_FX => Some(Self::PS3),

        // }
        if product_id == GAME_CUBE_CONTROLLER_ADAPTER {
            return Some(Self::Gamecube);
        }
        if product_id == STEAM_DECK {
            return Some(Self::SteamDeck);
        }
        if product_id == DUALSHOCK3_SIXAXIS || product_id == SPLIT_FISH_FRAG_FX {
            return Some(Self::PS3);
        }
        if product_id == DUALSHOCK4_05C4
        || product_id == STRIKE_PACK_FPS_DOMINATOR
        || product_id == DUALSHOCK4_09CC
        || product_id == DUALSHOCK4_USB_RECEIVER
        {
            return Some(Self::PS4);
        }
        if product_id == DUAL_SENSE_WIRELESS_CONTROLLER {
            return Some(Self::PS5);
        }
        if product_id == STEAM_CONTROLLER_0476
        || product_id == STEAM_CONTROLLER_1102
        || product_id == STEAM_CONTROLLER_1142
            || product_id == STEAM_CONTROLLER_11FC
            || product_id == STEAM_CONTROLLER_1201
            || product_id == STEAM_VIRTUAL_GAMEPAD
        {
            return Some(Self::SteamController);
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