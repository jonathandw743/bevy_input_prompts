use bevy_asset::AssetPath;
use bevy_input::{keyboard::KeyCode, mouse::MouseButton};
use bevy_input_prompts::{
    kenney::{
        Format, KenneyKeyboardAndMouseSettings, key_code::KenneyKeyCode,
        mouse_button::KenneyMouseButton,
    },
    xelu::{
        LightDark, XeluKeyboardAndMouseSettings, key_code::XeluKeyCode,
        mouse_button::XeluMouseButton,
    },
};
use std::collections::HashSet;
use walkdir::WalkDir;

fn all_file_paths(path: &'static str) -> HashSet<String> {
    let mut all_file_paths = HashSet::new();
    for entry in WalkDir::new(format!("assets/{}", path))
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_file() {
            all_file_paths.insert(
                entry
                    .into_path()
                    .strip_prefix("assets/")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }
    }
    all_file_paths
}

fn constructed_file_path<'a>(p: impl Into<AssetPath<'a>>) -> String {
    p.into().to_string()
}

fn log(all_file_paths: HashSet<String>, constructed_file_paths: HashSet<String>) {
    println!("file paths missed:");
    for file_path in &all_file_paths {
        if constructed_file_paths.contains(file_path) {
            continue;
        }
        println!("{}", file_path);
    }
    println!("constructed file paths that don't exist:");
    for file_path in &constructed_file_paths {
        if all_file_paths.contains(file_path) {
            continue;
        }
        println!("{}", file_path);
    }
    println!(
        "constructed file paths: {}, all file paths: {}",
        constructed_file_paths.len(),
        all_file_paths.len()
    );
}

fn xelu_keyboard_and_mouse() -> (HashSet<String>, HashSet<String>) {
    let all_file_paths = all_file_paths(
        "bevy_input_prompts/xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/",
    );
    let mut constructed_file_paths = HashSet::new();
    for light_dark in [LightDark::Dark, LightDark::Light] {
        for key_code in KEY_CODES {
            let c = constructed_file_path(XeluKeyCode {
                key_code,
                settings: XeluKeyboardAndMouseSettings { light_dark },
            });
            if !c.contains("not_found") {
                constructed_file_paths.insert(c);
            }
        }
        for mouse_button in MOUSE_BUTTONS {
            let c = constructed_file_path(XeluMouseButton {
                mouse_button,
                settings: XeluKeyboardAndMouseSettings { light_dark },
            });
            if !c.contains("not_found") {
                constructed_file_paths.insert(c);
            }
        }
    }
    (all_file_paths, constructed_file_paths)
}

fn kenney_keyboard_and_mouse() -> (HashSet<String>, HashSet<String>) {
    let all_file_paths =
        all_file_paths("bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/");
    let mut constructed_file_paths = HashSet::new();
    for outline in [false, true] {
        for icon_if_possible in [false, true] {
            for arrows_if_possible in [false, true] {
                for format in [Format::Default, Format::Double, Format::Vector] {
                    for key_code in KEY_CODES {
                        let c = constructed_file_path(KenneyKeyCode {
                            key_code,
                            settings: KenneyKeyboardAndMouseSettings {
                                outline,
                                format,
                                icon_if_possible,
                                arrows_if_possible,
                            },
                        });
                        if !c.contains("not_found") {
                            constructed_file_paths.insert(c);
                        }
                    }
                    for mouse_button in MOUSE_BUTTONS {
                        let c = constructed_file_path(KenneyMouseButton {
                            mouse_button,
                            settings: KenneyKeyboardAndMouseSettings {
                                outline,
                                format,
                                icon_if_possible,
                                arrows_if_possible,
                            },
                        });
                        if !c.contains("not_found") {
                            constructed_file_paths.insert(c);
                        }
                    }
                }
            }
        }
    }
    (all_file_paths, constructed_file_paths)
}

fn main() {
    let (all_file_paths, constructed_file_paths) = xelu_keyboard_and_mouse();
    log(all_file_paths, constructed_file_paths);
    let (all_file_paths, constructed_file_paths) = kenney_keyboard_and_mouse();
    log(all_file_paths, constructed_file_paths);
}

const MOUSE_BUTTONS: [MouseButton; 5] = [
    // MouseButton::Other(_),
    MouseButton::Back,
    MouseButton::Forward,
    MouseButton::Left,
    MouseButton::Middle,
    MouseButton::Right,
];

const KEY_CODES: [KeyCode; 194] = [
    // KeyCode::Unidentified(native_key_code),
    KeyCode::Backquote,
    KeyCode::Backslash,
    KeyCode::BracketLeft,
    KeyCode::BracketRight,
    KeyCode::Comma,
    KeyCode::Digit0,
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
    KeyCode::Equal,
    KeyCode::IntlBackslash,
    KeyCode::IntlRo,
    KeyCode::IntlYen,
    KeyCode::KeyA,
    KeyCode::KeyB,
    KeyCode::KeyC,
    KeyCode::KeyD,
    KeyCode::KeyE,
    KeyCode::KeyF,
    KeyCode::KeyG,
    KeyCode::KeyH,
    KeyCode::KeyI,
    KeyCode::KeyJ,
    KeyCode::KeyK,
    KeyCode::KeyL,
    KeyCode::KeyM,
    KeyCode::KeyN,
    KeyCode::KeyO,
    KeyCode::KeyP,
    KeyCode::KeyQ,
    KeyCode::KeyR,
    KeyCode::KeyS,
    KeyCode::KeyT,
    KeyCode::KeyU,
    KeyCode::KeyV,
    KeyCode::KeyW,
    KeyCode::KeyX,
    KeyCode::KeyY,
    KeyCode::KeyZ,
    KeyCode::Minus,
    KeyCode::Period,
    KeyCode::Quote,
    KeyCode::Semicolon,
    KeyCode::Slash,
    KeyCode::AltLeft,
    KeyCode::AltRight,
    KeyCode::Backspace,
    KeyCode::CapsLock,
    KeyCode::ContextMenu,
    KeyCode::ControlLeft,
    KeyCode::ControlRight,
    KeyCode::Enter,
    KeyCode::SuperLeft,
    KeyCode::SuperRight,
    KeyCode::ShiftLeft,
    KeyCode::ShiftRight,
    KeyCode::Space,
    KeyCode::Tab,
    KeyCode::Convert,
    KeyCode::KanaMode,
    KeyCode::Lang1,
    KeyCode::Lang2,
    KeyCode::Lang3,
    KeyCode::Lang4,
    KeyCode::Lang5,
    KeyCode::NonConvert,
    KeyCode::Delete,
    KeyCode::End,
    KeyCode::Help,
    KeyCode::Home,
    KeyCode::Insert,
    KeyCode::PageDown,
    KeyCode::PageUp,
    KeyCode::ArrowDown,
    KeyCode::ArrowLeft,
    KeyCode::ArrowRight,
    KeyCode::ArrowUp,
    KeyCode::NumLock,
    KeyCode::Numpad0,
    KeyCode::Numpad1,
    KeyCode::Numpad2,
    KeyCode::Numpad3,
    KeyCode::Numpad4,
    KeyCode::Numpad5,
    KeyCode::Numpad6,
    KeyCode::Numpad7,
    KeyCode::Numpad8,
    KeyCode::Numpad9,
    KeyCode::NumpadAdd,
    KeyCode::NumpadBackspace,
    KeyCode::NumpadClear,
    KeyCode::NumpadClearEntry,
    KeyCode::NumpadComma,
    KeyCode::NumpadDecimal,
    KeyCode::NumpadDivide,
    KeyCode::NumpadEnter,
    KeyCode::NumpadEqual,
    KeyCode::NumpadHash,
    KeyCode::NumpadMemoryAdd,
    KeyCode::NumpadMemoryClear,
    KeyCode::NumpadMemoryRecall,
    KeyCode::NumpadMemoryStore,
    KeyCode::NumpadMemorySubtract,
    KeyCode::NumpadMultiply,
    KeyCode::NumpadParenLeft,
    KeyCode::NumpadParenRight,
    KeyCode::NumpadStar,
    KeyCode::NumpadSubtract,
    KeyCode::Escape,
    KeyCode::Fn,
    KeyCode::FnLock,
    KeyCode::PrintScreen,
    KeyCode::ScrollLock,
    KeyCode::Pause,
    KeyCode::BrowserBack,
    KeyCode::BrowserFavorites,
    KeyCode::BrowserForward,
    KeyCode::BrowserHome,
    KeyCode::BrowserRefresh,
    KeyCode::BrowserSearch,
    KeyCode::BrowserStop,
    KeyCode::Eject,
    KeyCode::LaunchApp1,
    KeyCode::LaunchApp2,
    KeyCode::LaunchMail,
    KeyCode::MediaPlayPause,
    KeyCode::MediaSelect,
    KeyCode::MediaStop,
    KeyCode::MediaTrackNext,
    KeyCode::MediaTrackPrevious,
    KeyCode::Power,
    KeyCode::Sleep,
    KeyCode::AudioVolumeDown,
    KeyCode::AudioVolumeMute,
    KeyCode::AudioVolumeUp,
    KeyCode::WakeUp,
    KeyCode::Meta,
    KeyCode::Hyper,
    KeyCode::Turbo,
    KeyCode::Abort,
    KeyCode::Resume,
    KeyCode::Suspend,
    KeyCode::Again,
    KeyCode::Copy,
    KeyCode::Cut,
    KeyCode::Find,
    KeyCode::Open,
    KeyCode::Paste,
    KeyCode::Props,
    KeyCode::Select,
    KeyCode::Undo,
    KeyCode::Hiragana,
    KeyCode::Katakana,
    KeyCode::F1,
    KeyCode::F2,
    KeyCode::F3,
    KeyCode::F4,
    KeyCode::F5,
    KeyCode::F6,
    KeyCode::F7,
    KeyCode::F8,
    KeyCode::F9,
    KeyCode::F10,
    KeyCode::F11,
    KeyCode::F12,
    KeyCode::F13,
    KeyCode::F14,
    KeyCode::F15,
    KeyCode::F16,
    KeyCode::F17,
    KeyCode::F18,
    KeyCode::F19,
    KeyCode::F20,
    KeyCode::F21,
    KeyCode::F22,
    KeyCode::F23,
    KeyCode::F24,
    KeyCode::F25,
    KeyCode::F26,
    KeyCode::F27,
    KeyCode::F28,
    KeyCode::F29,
    KeyCode::F30,
    KeyCode::F31,
    KeyCode::F32,
    KeyCode::F33,
    KeyCode::F34,
    KeyCode::F35,
];
