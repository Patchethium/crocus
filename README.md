# Crocus

Crocus is a simple, lightweight non-linear video editor. It is written in Rust and ffmpeg, aiming to replace AviUtl in a modern environment.

> [!WARNING]  
> :construction: **WIP, DO NOT USE IN PRODUCTION** :construction:

## Roadmap

Wanna check where we are? Please refer to [ROADMAP.md](docs/ROADMAP.md).

## Features

Some features that AviUtl lacks and Crocus has or will have:

- [ ] Modern UI
- [ ] Cross-platform (Windows, macOS, Linux)
- [ ] I18N support
- [ ] Rhai/JavaScript scripting support
- [ ] Continuous support (the last update of AviUtl was in 2019, 6 years ago)

## Development

### Prerequisites

- Rust, you can install it from [rustup.rs](https://rustup.rs/)
- ffmpeg, rust-ffmpeg needs some specific steps to build, please refer to [rust-ffmpeg's guide](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building#dependencies) before building Crocus
- mold & clang if you're using Linux

### Build

```bash
# development build
cargo run
# tests
cargo test
# release build
cargo build --release
```

## License

Crocus is licensed under GPLv2 or later, see [LICENSE](LICENSE) for details.
