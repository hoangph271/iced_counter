# `iced_counter`

A scope creeped counter application created for learning [***iced_rs***](https://github.com/iced-rs/iced)...!

## Features

The app (`omni_app`) is composed of independent feature modules, all enabled by default:

| Feature | Description |
|---|---|
| `counter` | Increment/decrement counter with auto-increment (1/sec), allow-negative toggle, and reset |
| `system_info` | Displays system name, kernel, and OS version at startup |
| `instax_framer` | Pick a JPG/PNG image file and display it in a fixed frame |
| `omni_themes` | Theme picker - supports Default, Gruvbox, and Solarized in light/dark/system-default modes |

Build with a subset of features:

```sh
cargo run --no-default-features --features counter,system_info
```

## Lessons learned

These are the size in bytes of the executable when I altered the `[profile.release]` section of `Cargo.toml`,
by adding optimization config, one by one from top to bottom:

```toml
# Pop!_OS Noble Numbat 24.04 x86_64
# Commit fca0381d02a3e165860ee158d9bd64e8e1677f8a
# App was a minimal counter with no extra dependencies at this point

[profile.release]
# without any optimization: 24_084
strip = true              # 19_120
lto = true                # 16_348
panic = "abort"           # 14_504
codegen-units = 1         # 13_592
```

As the app scope creeped, heavy dependencies were added (`nokhwa`, `image`, `rfd`, `native-dialog`),
and the fully optimized release binary grew accordingly:

```toml
# Pop!_OS Noble Numbat 24.04 x86_64
# Commit c0f4bb772e6422c556b49fb07a752cb187e446d1

[profile.release]
# without any optimization: 27_625_336
strip = true              # 21_097_304
lto = true                # 17_787_760
panic = "abort"           # 15_794_160
codegen-units = 1         # 14_784_736
```
