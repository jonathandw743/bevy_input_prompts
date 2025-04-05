# bevy_input_prompts

Mappings from bevy input types to popular input prompt asset paths.

| Bevy Version | Plugin Version |
| -----------: | -------------: |
|         0.15 |            0.1 |

```rust
let arrow_left_handle = asset_server.load(XeluKeyCode {
    key_code: KeyCode::ArrowLeft,
    settings: default(),
});

let mouse_right_handle = asset_server.load(XeluMouseButton {
    key_code: KeyCode::ArrowLeft,
    settings: default(),
});

let a_button_handle = asset_server.load(XeluGamepadButton {
    gamepad_button: GamepadButton::South,
    settings: default(),
});
```

## adding assets to your project

Make sure to call the build function in your `build.rs` for example:

```rust
// build.rs

fn main() {
    // to copy all assets
    bevy_input_prompts::build_all();
    // to copy just xelu's assets
    bevy_input_prompts::build_xelu();
    // to copy just kenney's assets
    bevy_input_prompts::build_kenney();
}
```

And include this crate in `Cargo.toml` `[build-dependencies]`.

This copies input prompt assets into your `assets` directory if they don't exist (don't worry, nothing will get overwritten).

Or, copy assets manually in the same structure as the `assets` directory.

## contributing

Plug in your input device and run `cargo run --example interactive` to test all input prompts or `cargo run --example identification` to test gamepad identification.

If there are any issues, for example a lot of `unknown.png` showing, or `ERROR bevy_asset::server: Path not found` being logged, open an issue. (Unfortunately, I don't have every input device ever made.)

If you want to add your own prompt pack or want to fix a mapping issue, open a pull request!

## goals

The end stage for this project would look like:

- A way to programmatically access the majority of assets from all the prompt packs
- Support for all devices which have assets in the packs.
- Good defaults
- Gamepad detection using product/vendor IDs, mapping to branded assets
- Good unknown input/device handling

This will take a lot of work. My short-term goals are:

- A way to programmatically access most of Xelu's and Kenney's assets for keyboard and mouse, Xbox and PlayStation
- No panicking or path not found for Xbox, keyboard and mouse or PlayStation
- Basic unknown input/device handling

## licenses

The license for this project can be found in `LICENSE`. Assets in `assets` are not under this license.

Licenses for included assets can be found in the relevant sub directories of `assets`.