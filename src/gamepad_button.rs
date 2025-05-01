use bevy_input::{gamepad::GamepadButton, keyboard::KeyCode};
#[cfg(feature = "use_kenney_input_prompts")]
use kenney_input_prompts::tokenize_dir::keyboard_and_mouse as ktd;

use crate::Pack;

pub fn from_gamepad_button<'a, 'b>(version: Pack, gamepad_button: GamepadButton) -> Option<&'a [&'b [usize]]> {
    match version {
        #[cfg(feature = "use_kenney_input_prompts")]
        Pack::Kenney => match gamepad_button {
            GamepadButton::South => todo!(),
            GamepadButton::East => todo!(),
            GamepadButton::North => todo!(),
            GamepadButton::West => todo!(),
            GamepadButton::C => todo!(),
            GamepadButton::Z => todo!(),
            GamepadButton::LeftTrigger => todo!(),
            GamepadButton::LeftTrigger2 => todo!(),
            GamepadButton::RightTrigger => todo!(),
            GamepadButton::RightTrigger2 => todo!(),
            GamepadButton::Select => todo!(),
            GamepadButton::Start => todo!(),
            GamepadButton::Mode => todo!(),
            GamepadButton::LeftThumb => todo!(),
            GamepadButton::RightThumb => todo!(),
            GamepadButton::DPadUp => todo!(),
            GamepadButton::DPadDown => todo!(),
            GamepadButton::DPadLeft => todo!(),
            GamepadButton::DPadRight => todo!(),
            GamepadButton::Other(_) => todo!(),
        },
    }
}