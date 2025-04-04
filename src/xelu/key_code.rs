use bevy_asset::AssetPath;
use bevy_input::keyboard::KeyCode;

use crate::not_found::key_code::NotFoundKeyCode;

use super::XeluKeyboardAndMouseSettings;

#[derive(Clone, Debug)]
pub struct XeluKeyCode {
    pub key_code: KeyCode,
    pub settings: XeluKeyboardAndMouseSettings,
}

impl<'a> Into<AssetPath<'a>> for XeluKeyCode {
    fn into(self) -> AssetPath<'a> {
        let Some(key_code_name) = self.key_code_name() else {
            return NotFoundKeyCode {
                key_code: self.key_code,
            }
            .into();
        };
        format!(
            "bevy_input_prompts/xelu/Xelu_Free_Controller&Key_Prompts/Keyboard & Mouse/{}/{}_Key_{}.png",
            self.settings.light_dark.directory(), key_code_name, self.settings.light_dark.directory()
        )
        .into()
    }
}

impl XeluKeyCode {
    pub fn key_code_name(&self) -> Option<&'static str> {
        match self.key_code {
            KeyCode::KeyA => Some("A"),
            KeyCode::KeyB => Some("B"),
            KeyCode::KeyC => Some("C"),
            KeyCode::KeyD => Some("D"),
            KeyCode::KeyE => Some("E"),
            KeyCode::KeyF => Some("F"),
            KeyCode::KeyG => Some("G"),
            KeyCode::KeyH => Some("H"),
            KeyCode::KeyI => Some("I"),
            KeyCode::KeyJ => Some("J"),
            KeyCode::KeyK => Some("K"),
            KeyCode::KeyL => Some("L"),
            KeyCode::KeyM => Some("M"),
            KeyCode::KeyN => Some("N"),
            KeyCode::KeyO => Some("O"),
            KeyCode::KeyP => Some("P"),
            KeyCode::KeyQ => Some("Q"),
            KeyCode::KeyR => Some("R"),
            KeyCode::KeyS => Some("S"),
            KeyCode::KeyT => Some("T"),
            KeyCode::KeyU => Some("U"),
            KeyCode::KeyV => Some("V"),
            KeyCode::KeyW => Some("W"),
            KeyCode::KeyX => Some("X"),
            KeyCode::KeyY => Some("Y"),
            KeyCode::KeyZ => Some("Z"),
            KeyCode::Digit1 => Some("1"),
            KeyCode::Digit2 => Some("2"),
            KeyCode::Digit3 => Some("3"),
            KeyCode::Digit4 => Some("4"),
            KeyCode::Digit5 => Some("5"),
            KeyCode::Digit6 => Some("6"),
            KeyCode::Digit7 => Some("7"),
            KeyCode::Digit8 => Some("8"),
            KeyCode::Digit9 => Some("9"),
            KeyCode::Digit0 => Some("0"),
            KeyCode::Escape => Some("Esc"),
            KeyCode::F1 => Some("F1"),
            KeyCode::F2 => Some("F2"),
            KeyCode::F3 => Some("F3"),
            KeyCode::F4 => Some("F4"),
            KeyCode::F5 => Some("F5"),
            KeyCode::F6 => Some("F6"),
            KeyCode::F7 => Some("F7"),
            KeyCode::F8 => Some("F8"),
            KeyCode::F9 => Some("F9"),
            KeyCode::F10 => Some("F10"),
            KeyCode::F11 => Some("F11"),
            KeyCode::F12 => Some("F12"),
            KeyCode::Minus => Some("Minus"),
            KeyCode::Equal => Some("Plus"),
            KeyCode::Backslash | KeyCode::IntlBackslash | KeyCode::IntlRo | KeyCode::IntlYen => {
                Some("Slash")
            }
            KeyCode::Backspace => Some("Backspace"),
            KeyCode::Tab => Some("Tab"),
            KeyCode::BracketLeft => Some("Bracket_Left"),
            KeyCode::BracketRight => Some("Bracket_Right"),
            KeyCode::Enter => Some("Enter"),
            KeyCode::Semicolon => Some("Semicolon"),
            KeyCode::Quote => Some("Quote"),
            KeyCode::Space => Some("Space"),
            KeyCode::ShiftLeft => Some("Shift"),
            KeyCode::ShiftRight => Some("Shift_Alt"),
            KeyCode::ControlLeft => Some("Ctrl"),
            KeyCode::ControlRight => Some("Ctrl"),
            KeyCode::AltLeft => Some("Alt"),
            KeyCode::AltRight => Some("Alt"),
            KeyCode::SuperLeft => Some("Win"),
            KeyCode::SuperRight => Some("Win"),
            KeyCode::ArrowUp => Some("Arrow_Up"),
            KeyCode::ArrowDown => Some("Arrow_Down"),
            KeyCode::ArrowLeft => Some("Arrow_Left"),
            KeyCode::ArrowRight => Some("Arrow_Right"),

            KeyCode::CapsLock => Some("Caps_Lock"),
            KeyCode::Delete => Some("Del"),
            KeyCode::End => Some("End"),
            KeyCode::Home => Some("Home"),
            KeyCode::Insert => Some("Insert"),
            KeyCode::PageDown => Some("Page_Down"),
            KeyCode::PageUp => Some("Page_Up"),
            KeyCode::NumpadAdd => Some("Plus_Tall"),
            KeyCode::NumpadEnter => Some("Enter_Tall"),
            KeyCode::PrintScreen => Some("Print_Screen"),
            KeyCode::Meta => Some("Win"),
            KeyCode::Comma => Some("Mark_Left"),
            KeyCode::Period => Some("Mark_Right"),
            KeyCode::Slash => Some("Question"),
            KeyCode::Backquote => Some("Tilda"),

            KeyCode::Katakana
            | KeyCode::Hiragana
            | KeyCode::Undo
            | KeyCode::Select
            | KeyCode::Props
            | KeyCode::Paste
            | KeyCode::Open
            | KeyCode::Find
            | KeyCode::Cut
            | KeyCode::Copy
            | KeyCode::Again
            | KeyCode::Suspend
            | KeyCode::Resume
            | KeyCode::Abort
            | KeyCode::Turbo
            | KeyCode::Hyper
            | KeyCode::WakeUp
            | KeyCode::AudioVolumeUp
            | KeyCode::AudioVolumeMute
            | KeyCode::AudioVolumeDown
            | KeyCode::Sleep
            | KeyCode::Power
            | KeyCode::MediaTrackPrevious
            | KeyCode::MediaTrackNext
            | KeyCode::MediaStop
            | KeyCode::MediaSelect
            | KeyCode::MediaPlayPause
            | KeyCode::LaunchMail
            | KeyCode::LaunchApp2
            | KeyCode::LaunchApp1
            | KeyCode::Eject
            | KeyCode::BrowserStop
            | KeyCode::BrowserSearch
            | KeyCode::BrowserRefresh
            | KeyCode::BrowserHome
            | KeyCode::BrowserForward
            | KeyCode::BrowserFavorites
            | KeyCode::BrowserBack
            | KeyCode::Pause
            | KeyCode::ScrollLock
            | KeyCode::FnLock
            | KeyCode::Fn
            | KeyCode::NumpadSubtract
            | KeyCode::NumpadStar
            | KeyCode::NumpadParenRight
            | KeyCode::NumpadParenLeft
            | KeyCode::NumpadMultiply
            | KeyCode::NumpadMemorySubtract
            | KeyCode::NumpadMemoryStore
            | KeyCode::NumpadMemoryRecall
            | KeyCode::NumpadMemoryClear
            | KeyCode::NumpadMemoryAdd
            | KeyCode::NumpadHash
            | KeyCode::NumpadEqual
            | KeyCode::NumpadDivide
            | KeyCode::NumpadDecimal
            | KeyCode::NumpadComma
            | KeyCode::NumpadClearEntry
            | KeyCode::NumpadClear
            | KeyCode::NumpadBackspace
            | KeyCode::Numpad1
            | KeyCode::Numpad2
            | KeyCode::Numpad3
            | KeyCode::Numpad4
            | KeyCode::Numpad5
            | KeyCode::Numpad6
            | KeyCode::Numpad7
            | KeyCode::Numpad8
            | KeyCode::Numpad9
            | KeyCode::Numpad0
            | KeyCode::NumLock
            | KeyCode::Help
            | KeyCode::NonConvert
            | KeyCode::Lang5
            | KeyCode::Lang4
            | KeyCode::Lang3
            | KeyCode::Lang2
            | KeyCode::Lang1
            | KeyCode::KanaMode
            | KeyCode::Convert
            | KeyCode::ContextMenu
            | KeyCode::F13
            | KeyCode::F14
            | KeyCode::F15
            | KeyCode::F16
            | KeyCode::F17
            | KeyCode::F18
            | KeyCode::F19
            | KeyCode::F20
            | KeyCode::F21
            | KeyCode::F22
            | KeyCode::F23
            | KeyCode::F24
            | KeyCode::F25
            | KeyCode::F26
            | KeyCode::F27
            | KeyCode::F28
            | KeyCode::F29
            | KeyCode::F30
            | KeyCode::F31
            | KeyCode::F32
            | KeyCode::F33
            | KeyCode::F34
            | KeyCode::F35
            | KeyCode::Unidentified { .. } => None,
        }
    }
}
