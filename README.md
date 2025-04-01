# bevy_input_prompts

Mappings from bevy input types to popular input prompt asset paths.

```rust
let arrow_left_handle = asset_server.load(
    bevy_input_prompts::xelu::xelu_key_code::XeluKeyCode {
        key_code: KeyCode::ArrowLeft,
        light_dark: LightDark::Dark,
    }
);

let mouse_right_handle = asset_server.load(
    bevy_input_prompts::xelu::xelu_mouse_button::XeluMouseButton {
        key_code: KeyCode::ArrowLeft,
        light_dark: LightDark::Dark,
    }
);
```

## adding assets to your project

Make sure to call the relevant build function in your `build.rs` for example:

```rust
// build.rs

fn main() {
    bevy_input_prompts::xelu::build();
}
```

This copies input prompt assets into your `assets` directory (don't worry, nothing will get overwritten).

## assets/xelu/Xelu_Free_Controller&Key_Prompts

All the assets are in the public domain license under Creative Commons 0 (CC0) completely free to use in any personal or commercial project

Made by Nicolae (XELU) Berbece
Updated by Paul Paun (2011)
Readme.txt inside
https://thoseawesomeguys.com/prompts/

## assets/xelu/VectorSource.svg

license under Creative Commons 0 (CC0)

https://github.com/haaldor/Xelu_prompts_SVG