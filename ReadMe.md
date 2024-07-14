A simple counter application created for learning ***Iced***...!

### TODOs:
- [ ] ThemeMode subscription should pause/cancel when ThemeMode::Light or ThemeMode::Dark is selected
- [x] more styling for system information sentence
- [x] Fix ThemeMode PickList behavior
  - [x] ThemeMode::SystemDefault is not persisted when the system theme mode changes
- [x] toggle auto `AutoIncrement`
- [x] `Increment` every second because why not...?
- [x] Adapt system light/dark theme change

### Lessons learned:

As per commit `e3127f208a86396d6956d3877bb9448eb6735364`:
These are the size in bytes of the executable when I altered the `[profile.release]` section of `Cargo.toml`,
by enabling features, one by one from top to bottom:

```toml
# Pop!_OS jammy 22.04 x86_64
[profile.release]
# without any optimization: 21_128
strip = true              # 16_560
lto = true                # 13_640
codegen-units = 1         # 12_672
panic = "abort"           # 11_308
```
