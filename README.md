# ludum-dare-48

## Development Resources

Documentation:

 - [Raylib C documentation](https://www.raylib.com/cheatsheet/cheatsheet.html)
 - [Raylib C examples](https://www.raylib.com/examples.html)
 - [Raylib Rust documentation](https://docs.rs/raylib/3.5.0/raylib/)
 - [Raylib Rust examples](https://github.com/deltaphc/raylib-rs/tree/master/samples)
 - ["Are We Game Yet?"](https://arewegameyet.rs/#ecosystem)
 - [`cross` cross-compiler tool](https://github.com/rust-embedded/cross)

Core libraries:

 - [`raylib-rs`](https://github.com/deltaphc/raylib-rs)
 - [`serde`](https://serde.rs/)
 - [`serialstudio-rs`](https://github.com/Ewpratten/serialstudio-rs)

### VSCode Setup

If using VSCode, disable the `Rust` extension, and install everything in the **Workspace Recommendations** (You will see this list by searching `@recommended` in the extensions panel)

### Attaching to the in-game profiler

When the game is ran in its `dev` profile (using `cargo run`), the internal profiler is exposed on `127.0.0.1:8019`.

To connect to this, install [Serial Studio](https://www.alex-spataru.com/serial-studio.html) from [here](https://github.com/Serial-Studio/Serial-Studio/releases/tag/v1.0.20), and point it at `127.0.0.1:8019` by selecting the `auto` communication mode, then selecting `Network>TCP` for the connection type. 

### Cross-compiling builds for other platforms

On linux, you can run `./bundle/create-releases.sh` to automatically cross-compile to all supported platforms as long as you have [`cross`](https://github.com/rust-embedded/cross) installed on your system. This is also done by the [Bundle CI task](https://github.com/Ewpratten/ludum-dare-48/actions/workflows/bundle.yml) every time code is pushed to `master`. 