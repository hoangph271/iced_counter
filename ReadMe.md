A simple counter application created for learning ***Iced***...!

### TODOs:
- [ ] ThemeMode subscription should pause/cancel when ThemeMode::Light or ThemeMode::Dark is selected

### Lessons learned:

These are the size in bytes of the executable when I altered the `[profile.release]` section of `Cargo.toml`,
by enabling features, one by one from top to bottom:

```toml
# Pop!_OS jammy 22.04 x86_64
# Commit be25472aef61a6f4ea15c5982321fe31b28c83e2

[profile.release]
# without any optimization: 18_528
strip = true              # 14_508
lto = true                # 12_136
panic = "abort"           # 10_728
codegen-units = 1         # 10_160
```
