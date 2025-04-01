# bevy_input_prompts

Mappings from bevy input types to popular input prompt asset paths.

```rust
let arrow_left_handle = asset_server.load(XeluKeyCode {
        key_code: KeyCode::ArrowLeft,
        light_dark: LightDark::Dark,
});

let mouse_right_handle = asset_server.load(XeluMouseButton {
        key_code: KeyCode::ArrowLeft,
        light_dark: LightDark::Dark,
});

let a_button_handle = asset_server.load(XeluGamepadButton {
        gamepad_button: GamepadButton::South,
        gamepad_brand: GamepadBrand::XboxSeries,
});
```

## adding assets to your project

Make sure to call the build function in your `build.rs` for example:

```rust
// build.rs

fn main() {
    bevy_input_prompts::build();
}
```

This copies input prompt assets into your `assets` directory if they don't exist (don't worry, nothing will get overwritten).

Or, copy assets manually in the same structure as the `assets` directory.

## contributing

Plug in your input device and run `cargo run --example detection`.

If there are any issues, for example a lot of `unknown.png` showing, or `ERROR bevy_asset::server: Path not found` being logged, open an issue. (Unfortunately I don't have every input device ever made.)

If you want to add your own prompt pack or want to fix a mapping issue, open a pull request!

## lisences

The lisence for this project can be found in `LICENSE`. Assets in `assets` are not under this lisence.

Lisences for included assets can be found in the relevant sub directories of `assets`.