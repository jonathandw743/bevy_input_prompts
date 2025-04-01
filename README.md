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

This copies input prompt assets into your `assets` directory (don't worry, nothing will get overwritten).

## contributing

Plug in your input device and run `cargo run --example detection`.

If there are any issues, for example a lot of `unknown.png` showing, or `ERROR bevy_asset::server: Path not found` being logged, open an issue. (Unfortunately I don't have every input device ever made.)

If you want to add your own prompt pack or want to fix a mapping issue, open a pull request!

## assets/xelu/Xelu_Free_Controller&Key_Prompts

All the assets are in the public domain license under Creative Commons 0 (CC0) completely free to use in any personal or commercial project

Made by Nicolae (XELU) Berbece
Updated by Paul Paun (2011)
Readme.txt inside
https://thoseawesomeguys.com/prompts/

## assets/xelu/VectorSource.svg

license under Creative Commons 0 (CC0)

https://github.com/haaldor/Xelu_prompts_SVG