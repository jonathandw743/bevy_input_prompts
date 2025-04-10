use bevy_asset::AssetPath;
use bevy_input::keyboard::KeyCode;

use crate::not_found::key_code::NotFoundKeyCode;

use super::KenneyKeyboardAndMouseSettings;

/// converts to a Kenney's input prompt representing a [`KeyCode`]
#[derive(Clone, Debug)]
pub struct KenneyKeyCode {
    pub key_code: KeyCode,
    pub settings: KenneyKeyboardAndMouseSettings,
}

impl<'a> Into<AssetPath<'a>> for KenneyKeyCode {
    fn into(self) -> AssetPath<'a> {
        let Some(key_code_name) = self.key_code_name() else {
            return NotFoundKeyCode {
                key_code: self.key_code,
            }
            .into();
        };
        format!(
            "bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/{}/keyboard_{}{}{}{}.{}",
            self.settings.format.directiory(),
            self.arrow_arrows_name(),
            key_code_name,
            self.icon_name(),
            self.outline_name(),
            self.settings.format.extension()
        )
        .into()
    }
}

impl KenneyKeyCode {
    pub fn outline_name(&self) -> &'static str {
        if self.settings.outline {
            "_outline"
        } else {
            ""
        }
    }
    pub fn icon_possible(&self) -> bool {
        match self.key_code {
            KeyCode::Tab => true,
            KeyCode::Space => true,
            KeyCode::ShiftLeft => true,
            KeyCode::ShiftRight => true,
            KeyCode::CapsLock => true,
            KeyCode::Backspace => true,
            _ => false,
        }
    }
    pub fn alternative_icon_possible(&self) -> bool {
        match self.key_code {
            KeyCode::Tab => true,
            KeyCode::Backspace => true,
            _ => false,
        }
    }
    pub fn icon_name(&self) -> &'static str {
        if self.settings.alternative_icon_if_possible && self.alternative_icon_possible() {
            return "_icon_alternative";
        }
        if self.settings.icon_if_possible && self.icon_possible() {
            return "_icon";
        }
        ""
    }
    pub fn arrow_arrows_name(&self) -> &'static str {
        match (self.settings.arrows_if_possible, self.key_code) {
            (false, KeyCode::ArrowDown) => "arrow_",
            (false, KeyCode::ArrowUp) => "arrow_",
            (false, KeyCode::ArrowLeft) => "arrow_",
            (false, KeyCode::ArrowRight) => "arrow_",
            (true, KeyCode::ArrowDown) => "arrows_",
            (true, KeyCode::ArrowUp) => "arrows_",
            (true, KeyCode::ArrowLeft) => "arrows_",
            (true, KeyCode::ArrowRight) => "arrows_",
            _ => "",
        }
    }
    pub fn key_code_name(&self) -> Option<&'static str> {
        match self.key_code {
            KeyCode::Unidentified(_native_key_code) => None,
            KeyCode::Backquote => Some("tilde"),
            KeyCode::Backslash => Some("slash_back"),
            KeyCode::BracketLeft => Some("bracket_open"),
            KeyCode::BracketRight => Some("bracket_close"),
            KeyCode::Comma => Some("comma"),
            KeyCode::Digit0 => Some("0"),
            KeyCode::Digit1 => Some("1"),
            KeyCode::Digit2 => Some("2"),
            KeyCode::Digit3 => Some("3"),
            KeyCode::Digit4 => Some("4"),
            KeyCode::Digit5 => Some("5"),
            KeyCode::Digit6 => Some("6"),
            KeyCode::Digit7 => Some("7"),
            KeyCode::Digit8 => Some("8"),
            KeyCode::Digit9 => Some("9"),
            KeyCode::Equal => Some("equals"),
            KeyCode::IntlBackslash | KeyCode::IntlRo | KeyCode::IntlYen => Some("slash_back"),
            KeyCode::KeyA => Some("a"),
            KeyCode::KeyB => Some("b"),
            KeyCode::KeyC => Some("c"),
            KeyCode::KeyD => Some("d"),
            KeyCode::KeyE => Some("e"),
            KeyCode::KeyF => Some("f"),
            KeyCode::KeyG => Some("g"),
            KeyCode::KeyH => Some("h"),
            KeyCode::KeyI => Some("i"),
            KeyCode::KeyJ => Some("j"),
            KeyCode::KeyK => Some("k"),
            KeyCode::KeyL => Some("l"),
            KeyCode::KeyM => Some("m"),
            KeyCode::KeyN => Some("n"),
            KeyCode::KeyO => Some("o"),
            KeyCode::KeyP => Some("p"),
            KeyCode::KeyQ => Some("q"),
            KeyCode::KeyR => Some("r"),
            KeyCode::KeyS => Some("s"),
            KeyCode::KeyT => Some("t"),
            KeyCode::KeyU => Some("u"),
            KeyCode::KeyV => Some("v"),
            KeyCode::KeyW => Some("w"),
            KeyCode::KeyX => Some("x"),
            KeyCode::KeyY => Some("y"),
            KeyCode::KeyZ => Some("z"),
            KeyCode::Minus => Some("minus"),
            KeyCode::Period => Some("period"),
            KeyCode::Quote => Some("quote"),
            KeyCode::Semicolon => Some("semicolon"),
            KeyCode::Slash => Some("slash_forward"),
            KeyCode::AltLeft => Some("alt"),
            KeyCode::AltRight => Some("alt"),
            KeyCode::Backspace => Some("backspace"),
            KeyCode::CapsLock => Some("capslock"),
            KeyCode::ContextMenu => None,
            KeyCode::ControlLeft => Some("ctrl"),
            KeyCode::ControlRight => Some("ctrl"),
            KeyCode::Enter => Some("enter"),
            KeyCode::SuperLeft => Some("win"),
            KeyCode::SuperRight => Some("win"),
            KeyCode::ShiftLeft => Some("shift"),
            KeyCode::ShiftRight => Some("shift"),
            KeyCode::Space => Some("space"),
            KeyCode::Tab => Some("tab"),
            KeyCode::Convert => None,
            KeyCode::KanaMode => None,
            KeyCode::Lang1 => None,
            KeyCode::Lang2 => None,
            KeyCode::Lang3 => None,
            KeyCode::Lang4 => None,
            KeyCode::Lang5 => None,
            KeyCode::NonConvert => None,
            KeyCode::Delete => Some("delete"),
            KeyCode::End => Some("end"),
            KeyCode::Help => None,
            KeyCode::Home => Some("home"),
            KeyCode::Insert => Some("insert"),
            KeyCode::PageDown => Some("page_down"),
            KeyCode::PageUp => Some("page_up"),
            KeyCode::ArrowDown => Some("down"),
            KeyCode::ArrowLeft => Some("left"),
            KeyCode::ArrowRight => Some("right"),
            KeyCode::ArrowUp => Some("up"),
            KeyCode::NumLock => Some("numlock"),
            KeyCode::Numpad0 => None,
            KeyCode::Numpad1 => None,
            KeyCode::Numpad2 => None,
            KeyCode::Numpad3 => None,
            KeyCode::Numpad4 => None,
            KeyCode::Numpad5 => None,
            KeyCode::Numpad6 => None,
            KeyCode::Numpad7 => None,
            KeyCode::Numpad8 => None,
            KeyCode::Numpad9 => None,
            KeyCode::NumpadAdd => Some("numpad_plus"),
            KeyCode::NumpadBackspace => None,
            KeyCode::NumpadClear => None,
            KeyCode::NumpadClearEntry => None,
            KeyCode::NumpadComma => None,
            KeyCode::NumpadDecimal => None,
            KeyCode::NumpadDivide => None,
            KeyCode::NumpadEnter => Some("numpad_enter"),
            KeyCode::NumpadEqual => None,
            KeyCode::NumpadHash => None,
            KeyCode::NumpadMemoryAdd => None,
            KeyCode::NumpadMemoryClear => None,
            KeyCode::NumpadMemoryRecall => None,
            KeyCode::NumpadMemoryStore => None,
            KeyCode::NumpadMemorySubtract => None,
            KeyCode::NumpadMultiply => None,
            KeyCode::NumpadParenLeft => None,
            KeyCode::NumpadParenRight => None,
            KeyCode::NumpadStar => None,
            KeyCode::NumpadSubtract => None,
            KeyCode::Escape => Some("escape"),
            KeyCode::Fn => Some("function"),
            KeyCode::FnLock => None,
            KeyCode::PrintScreen => Some("printscreen"),
            KeyCode::ScrollLock => None,
            KeyCode::Pause => None,
            KeyCode::BrowserBack => None,
            KeyCode::BrowserFavorites => None,
            KeyCode::BrowserForward => None,
            KeyCode::BrowserHome => None,
            KeyCode::BrowserRefresh => None,
            KeyCode::BrowserSearch => None,
            KeyCode::BrowserStop => None,
            KeyCode::Eject => None,
            KeyCode::LaunchApp1 => None,
            KeyCode::LaunchApp2 => None,
            KeyCode::LaunchMail => None,
            KeyCode::MediaPlayPause => None,
            KeyCode::MediaSelect => None,
            KeyCode::MediaStop => None,
            KeyCode::MediaTrackNext => None,
            KeyCode::MediaTrackPrevious => None,
            KeyCode::Power => None,
            KeyCode::Sleep => None,
            KeyCode::AudioVolumeDown => None,
            KeyCode::AudioVolumeMute => None,
            KeyCode::AudioVolumeUp => None,
            KeyCode::WakeUp => None,
            KeyCode::Meta => Some("win"),
            KeyCode::Hyper => None,
            KeyCode::Turbo => None,
            KeyCode::Abort => None,
            KeyCode::Resume => None,
            KeyCode::Suspend => None,
            KeyCode::Again => None,
            KeyCode::Copy => None,
            KeyCode::Cut => None,
            KeyCode::Find => None,
            KeyCode::Open => None,
            KeyCode::Paste => None,
            KeyCode::Props => None,
            KeyCode::Select => None,
            KeyCode::Undo => None,
            KeyCode::Hiragana => None,
            KeyCode::Katakana => None,
            KeyCode::F1 => Some("f1"),
            KeyCode::F2 => Some("f2"),
            KeyCode::F3 => Some("f3"),
            KeyCode::F4 => Some("f4"),
            KeyCode::F5 => Some("f5"),
            KeyCode::F6 => Some("f6"),
            KeyCode::F7 => Some("f7"),
            KeyCode::F8 => Some("f8"),
            KeyCode::F9 => Some("f9"),
            KeyCode::F10 => Some("f10"),
            KeyCode::F11 => Some("f11"),
            KeyCode::F12 => Some("f12"),
            KeyCode::F13 => None,
            KeyCode::F14 => None,
            KeyCode::F15 => None,
            KeyCode::F16 => None,
            KeyCode::F17 => None,
            KeyCode::F18 => None,
            KeyCode::F19 => None,
            KeyCode::F20 => None,
            KeyCode::F21 => None,
            KeyCode::F22 => None,
            KeyCode::F23 => None,
            KeyCode::F24 => None,
            KeyCode::F25 => None,
            KeyCode::F26 => None,
            KeyCode::F27 => None,
            KeyCode::F28 => None,
            KeyCode::F29 => None,
            KeyCode::F30 => None,
            KeyCode::F31 => None,
            KeyCode::F32 => None,
            KeyCode::F33 => None,
            KeyCode::F34 => None,
            KeyCode::F35 => None,
        }
    }
}
