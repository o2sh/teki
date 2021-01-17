# Teki ([敵](https://en.wiktionary.org/wiki/%E6%95%B5))

[![crates][s1]][l1] ![MIT][s2]

[s1]: https://img.shields.io/crates/v/teki.svg
[l1]: https://crates.io/crates/teki
[s2]: https://img.shields.io/badge/license-MIT-blue.svg

Touhou-style shoot'em up written in Rust using [legion](https://github.com/amethyst/legion) and [sdl2](https://github.com/Rust-SDL2/rust-sdl2).

<h3 align="center"><img src="resources/teki.gif" height="500px"></h3>

### Building

Since teki depends on SDL2, you first need to
[install the SDL2 development libraries](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries).

Once SDL2 is set up, you can build and run the app simply using:

```
cargo run
```

### How to play (Control)

  * Arrow key : Move left, right, up or down
  * Space key : Shoot a bullet
