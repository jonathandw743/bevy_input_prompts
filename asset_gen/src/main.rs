use bevy_input::gamepad::{GamepadAxis, GamepadButton};
use bevy_input::{keyboard::KeyCode, mouse::MouseButton};
use image::Rgba;
use rusttype::{Font, Rect, Scale, point};
use std::fs::{self, File};
use std::io::Read;

fn blend_pixels(background: &Rgba<u8>, foreground: &Rgba<u8>) -> Rgba<u8> {
    let alpha = foreground[3] as f32 / 255.0; // Alpha from foreground
    let inv_alpha = 1.0 - alpha;

    let r = (foreground[0] as f32 * alpha + background[0] as f32 * inv_alpha) as u8;
    let g = (foreground[1] as f32 * alpha + background[1] as f32 * inv_alpha) as u8;
    let b = (foreground[2] as f32 * alpha + background[2] as f32 * inv_alpha) as u8;

    Rgba([r, g, b, 255])
}

fn process(dir: &'static str, text: &str, font: &Font) {
    let mut img_rgba = image::open("assets/blank.png")
        .expect("Failed to load image")
        .to_rgba8();

    let scale = Scale { x: 50.0, y: 50.0 };
    let color = Rgba([255, 255, 255, 255]);

    let (width, height) = img_rgba.dimensions();

    let mut total_bb: Option<Rect<i32>> = None;
    for glyph in font.layout(text, scale, point(0.0, 0.0)) {
        if let Some(new_bb) = glyph.pixel_bounding_box() {
            if let Some(bb) = &mut total_bb {
                bb.min = point(bb.min.x.min(new_bb.min.x), bb.min.y.min(new_bb.min.y));
                bb.max = point(bb.max.x.max(new_bb.max.x), bb.max.y.max(new_bb.max.y));
            } else {
                total_bb = Some(new_bb);
            }
        }
    }
    let total_bb = total_bb.expect("no text bounding boxes");

    let text_position = point(
        width as f32 * 0.5 - total_bb.max.x as f32 * 0.5,
        height as f32 * 0.5 - total_bb.max.y as f32 * 0.5,
    );

    for glyph in font.layout(text, scale, text_position) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x + bounding_box.min.x as u32;
                let y = y + bounding_box.min.y as u32;
                let image_pixel = img_rgba.get_pixel(x, y);
                let text_pixel = Rgba([color.0[0], color.0[1], color.0[2], (v * 255.0) as u8]);
                let blended_pixel = blend_pixels(image_pixel, &text_pixel);
                img_rgba.put_pixel(x, y, blended_pixel)
            });
        }
    }

    fs::create_dir_all(format!("assets/bevy_input_prompts/not_found/{}", dir)).expect("creating asset directory failed");
    img_rgba
        .save(format!(
            "assets/bevy_input_prompts/not_found/{}/{}.png",
            dir, text
        ))
        .expect("Failed to save image");
}

fn main() {
    let mut font_file =
        File::open("assets/FiraMono-Regular.ttf").expect("Failed to open font file");
    let mut font_data = Vec::new();
    font_file
        .read_to_end(&mut font_data)
        .expect("Failed to read font file");
    let font = Font::try_from_bytes(&font_data).expect("Error loading font");

    for key_code in KEY_CODES {
        process("KeyCode", &format!("{:?}", key_code), &font);
    }
    process("KeyCode", "Unidentified", &font);
    for mouse_button in MOUSE_BUTTONS {
        process("MouseButton", &format!("{:?}", mouse_button), &font);
    }
    process("MouseButton", "Other", &font);
    for gamepad_axis in GAMEPAD_AXES {
        process("GamepadAxis", &format!("{:?}", gamepad_axis), &font);
    }
    process("GamepadAxis", "Other", &font);
    for gamepad_button in GAMEPAD_BUTTONS {
        process("GamepadButton", &format!("{:?}", gamepad_button), &font);
    }
    process("GamepadButton", "Other", &font);
}

const MOUSE_BUTTONS: [MouseButton; 5] = [
    // MouseButton::Other(_),
    MouseButton::Back,
    MouseButton::Forward,
    MouseButton::Left,
    MouseButton::Middle,
    MouseButton::Right,
];

const GAMEPAD_AXES: [GamepadAxis; 6] = [
    GamepadAxis::LeftStickX,
    GamepadAxis::LeftStickY,
    GamepadAxis::LeftZ,
    GamepadAxis::RightStickX,
    GamepadAxis::RightStickY,
    GamepadAxis::RightZ,
    // GamepadAxis::Other(_),
];

const GAMEPAD_BUTTONS: [GamepadButton; 19] = [
    GamepadButton::South,
    GamepadButton::East,
    GamepadButton::North,
    GamepadButton::West,
    GamepadButton::C,
    GamepadButton::Z,
    GamepadButton::LeftTrigger,
    GamepadButton::LeftTrigger2,
    GamepadButton::RightTrigger,
    GamepadButton::RightTrigger2,
    GamepadButton::Select,
    GamepadButton::Start,
    GamepadButton::Mode,
    GamepadButton::LeftThumb,
    GamepadButton::RightThumb,
    GamepadButton::DPadUp,
    GamepadButton::DPadDown,
    GamepadButton::DPadLeft,
    GamepadButton::DPadRight,
    // GamepadButton::Other(())
];

const KEY_CODES: [KeyCode; 194] = [
    // KeyCode::Unidentified(_),
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
