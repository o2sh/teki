# Teki ([敵](https://en.wiktionary.org/wiki/%E6%95%B5))

Teki (敵, lit. Enemy) is a free and open-source fangame of the Tōhō series, written in Rust using [SDL2](https://github.com/Rust-SDL2/rust-sdl2) and OpenGL.

<h3 align="center"><img src="resources/teki.gif" height="400px"></h3>

<p align="center"><a href="https://o2sh.github.io/teki/">Play online!</a></p>

### Building

Since teki depends on SDL2, you first need to
[install the SDL2 development libraries](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries).

Once SDL2 is set up, you can build and run the app simply using:

```
cargo run
```

### How to play (Control)

  * The Arrow Keys move the character around
  * Z causes a short barrage of shots to be fired; it may be held down for rapidfire
  * Esc pauses the game and brings you to the in-game menu