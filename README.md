# Crocus

Crocus is a simple, lightweight non-linear video editor. It is written in Rust and ffmpeg, aiming to replace AviUtl in a modern environment.

Crocus is not fast nor feature-rich yet, it's under heavy development and not recommended for production use.

## Features

Some features that Crocus shares with other video editors:

- [ ] Import and export video files
- [ ] Cut, copy, paste, delete, move, and trim clips
- [ ] Keyframes
- [ ] Effects(Filters/Transitions)
- [ ] Lua scripting support

Some features that AviUtl lacks and Crocus has or will have:

- [ ] Modern UI
- [ ] Cross-platform (Windows, macOS, Linux)
- [ ] I18N support
- [ ] Rhai/JavaScript scripting support

## Development

### Prerequisites

- Rust, you can install it from [rustup.rs](https://rustup.rs/)
- ffmpeg, rust-ffmpeg needs some specific steps to build, please refer to [rust-ffmpeg's guide](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building#dependencies) before building Crocus

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
