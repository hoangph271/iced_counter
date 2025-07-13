# `iced_counter`

A scope creeped counter application created for learning [***iced_rs***](https://github.com/iced-rs/iced)...!

## TODOs

- [ ] ThemeMode subscription should pause/cancel when ThemeMode::Light or ThemeMode::Dark is selected

## Lessons learned

These are the size in bytes of the executable when I altered the `[profile.release]` section of `Cargo.toml`,
by enabling features, one by one from top to bottom:

```toml
# Pop!_OS Noble Numbat 24.04 x86_64
# Commit fca0381d02a3e165860ee158d9bd64e8e1677f8a

[profile.release]
# without any optimization: 24_084
strip = true              # 19_120
lto = true                # 16_348
panic = "abort"           # 14_504
codegen-units = 1         # 13_592
```
