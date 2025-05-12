# bevy_input_prompts

Mappings from bevy input types to popular input prompt asset paths along with all extra options for prompts in included packs.

| Bevy Version | Plugin Version |
| -----------: | -------------: |
|         0.16 |            0.2 |

```rust
(GamepadButton::South, GamepadBrand::XboxSeries).file_path_extra(Pack::Kenney, &[_color, _outline])
```

## features

There is one feature flag for each pack. For example, use `cargo add bevy_input_prompts --features kenney_input_prompts`.

## adding assets to your project

Make sure to call the `copy_assets` function in your `build.rs` for example:

```rust
// build.rs

fn main() {
    bevy_input_prompts::copy_assets();
}
```

And (for example) use `cargo add bevy_input_prompts --build --features kenney_input_prompts`.

This copies input prompt assets into your `assets` directory if they don't exist (don't worry, nothing will get overwritten).

## contributing

Plug in your input device and run some examples.

If there are any issues, for example a lot of `ERROR bevy_asset::server: Path not found` being logged, open an issue. (Unfortunately, I don't have every input device.)

If you want to add your own prompt pack or want to fix a mapping issue, open a pull request!

These are some specific things that I can't do myself:

- Testing different gamepad brand detection
- Testing mappings for different controllers
- Testing different keyboard locales (for `Key` and `KeyboardInput` not `KeyCode`).