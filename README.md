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

## About the name

I use flower names for my side projects.

`Crocus` (/ˈkroʊkəs/) is a genus of seasonal flowering plants in the family Iridaceae (Iris family). Its another famous name is `saffron`, a spice derived from the flower's stigmas, used in many cuisines, such as risotto alla milanese. :yum:

<p><a href="https://commons.wikimedia.org/wiki/File:Crocus-sp.-4637.jpg#/media/File:Crocus-sp.-4637.jpg"><img src="https://upload.wikimedia.org/wikipedia/commons/b/b8/Crocus-sp.-4637.jpg" alt="3 stamens and style"></a><br>By <a href="//commons.wikimedia.org/wiki/User:Bartiebert" class="mw-redirect" title="User:Bartiebert">Danny S.</a> - <span class="int-own-work" lang="en">Own work</span>, <a href="https://creativecommons.org/licenses/by-sa/3.0" title="Creative Commons Attribution-Share Alike 3.0">CC BY-SA 3.0</a>, <a href="https://commons.wikimedia.org/w/index.php?curid=14576633">Link</a></p>
