use fs_extra::error::Error;

pub mod product_ids;
pub mod vendor_ids;

pub mod gamepad_button;
pub mod key_code;

#[cfg(feature = "use_kenney_input_prompts")]
pub use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4 as kenney_tokenize;

pub enum Pack {
    #[cfg(feature = "use_kenney_input_prompts")]
    Kenney,
}

pub fn first_file_path<T: tokenize_dir::ToIter>(pack: Pack, files: T) -> &'static str {
    let file_index = files.file_indices()[0];
    match pack {
        #[cfg(feature = "use_kenney_input_prompts")]
        Pack::Kenney => kenney_input_prompts::tokenize_dir::FILE_PATHS[file_index],
    }
}

pub fn copy_assets() -> Result<(), Error> {
    #[cfg(feature = "use_kenney_input_prompts")]
    kenney_input_prompts::copy_assets("assets/bevy_input_prompts")?;
    Ok(())
}

// impl GamepadBrand {
//     pub fn from_vendor_id(vendor_id: u16) -> Option<Self> {
//         if vendor_id == XBOX || vendor_id == XBOX_THIRD_PARTY {
//             return Some(Self::XboxSeries);
//         }
//         if vendor_id == VALVE {
//             return Some(Self::SteamDeck);
//         }
//         if vendor_id == SONY_COMPUTER_ENTERTAINMENT_AMERICA
//             || vendor_id == SONY_COMPUTER_ENTERTAINMENT_EUROPE
//             || vendor_id == SONY_CORPORATION
//             || vendor_id == SONY_MOBILE_COMMUNICATIONS
//         {
//             return Some(Self::PlayStation5);
//         }
//         if vendor_id == NINTENDO_CO_LTD || vendor_id == NINTENDO_OF_AMERICA {
//             return Some(Self::Switch);
//         }
//         return None;
//     }
//     pub fn from_product_id(product_id: u16) -> Option<Self> {
//         if product_id == STEAM_CONTROLLER_0476
//                 || product_id == STEAM_CONTROLLER_1102
//                 || product_id == STEAM_CONTROLLER_1142
//                 || product_id == STEAM_CONTROLLER_11FC
//                 || product_id == STEAM_CONTROLLER_1201
//                 || product_id == STEAM_VIRTUAL_GAMEPAD
//             {
//                 return Some(Self::SteamController);
//             }
//             if product_id == DUALSHOCK3_SIXAXIS || product_id == SPLIT_FISH_FRAG_FX {
//                 return Some(Self::PlayStation3)
//             }
//             if product_id == DUALSHOCK4_05C4 || product_id == STRIKE_PACK_FPS_DOMINATOR || product_id == DUALSHOCK4_09CC || product_id == DUALSHOCK4_USB_RECEIVER {
//                 return Some(Self::PlayStation4);
//             }
//             return None;
//     }
//     pub fn from_gamepad(gamepad: &Gamepad) -> Option<Self> {
//         if let Some(product_id) = gamepad.product_id() {
//             if let Some(gamepad_brand) = Self::from_product_id(product_id) {
//                 return Some(gamepad_brand);
//             }
//         }
//         if let Some(vendor_id) = gamepad.vendor_id() {
//             if let Some(gamepad_brand) = Self::from_vendor_id(vendor_id) {
//                 return Some(gamepad_brand);
//             }
//         }
//         return None;
//     }
// }

// impl GamepadBrand {
//     pub fn from_vendor_id(vendor_id: u16) -> Option<Self> {
//         if vendor_id == XBOX || vendor_id == XBOX_THIRD_PARTY {
//             return Some(Self::XboxSeries);
//         }
//         // this will also match steam controllers but this is an ok estimation
//         if vendor_id == VALVE {
//             return Some(Self::SteamDeck);
//         }
//         if vendor_id == SONY_COMPUTER_ENTERTAINMENT_AMERICA
//             || vendor_id == SONY_COMPUTER_ENTERTAINMENT_EUROPE
//             || vendor_id == SONY_CORPORATION
//             || vendor_id == SONY_MOBILE_COMMUNICATIONS
//         {
//             return Some(Self::PS5);
//         }
//         if vendor_id == NINTENDO_CO_LTD || vendor_id == NINTENDO_OF_AMERICA {
//             return Some(Self::Switch);
//         }
//         return None;
//     }
//     pub fn from_gamepad(gamepad: &Gamepad) -> Option<Self> {
//         if let Some(vendor_id) = gamepad.vendor_id() {
//             if let Some(gamepad_brand) = Self::from_vendor_id(vendor_id) {
//                 return Some(gamepad_brand);
//             }
//         }
//         return None;
//     }
// }
