use bevy_input::mouse::MouseButton;

pub fn mouse_button_name(mouse_button: MouseButton) -> Option<&'static str> {
    match mouse_button {
        MouseButton::Left => Some("Mouse_Left"),
        MouseButton::Right => Some("Mouse_Right"),
        MouseButton::Middle => Some("Mouse_Middle"),
        MouseButton::Back | MouseButton::Forward | MouseButton::Other(_) => None,
    }
}
