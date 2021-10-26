# ArduboyRust

Running Rust on the [Arduboy](https://arduboy.com/) miniature game system.

## Creating and building an Arduboy crate

Prerequisite: rustup

The idea is to compile our Rust code as a static library which can then be linked into an Arduino C++ project.

1. Create a new library crate:

```
cargo new --lib my-crate
cd my-crate
```

2. Add the following to your crate’s `Cargo.toml`:

```toml
[lib]
crate-type = ["staticlib"]
```

3. From this repo, copy the `.cargo` directory and the `arduboy.json` and `rust-toolchain.toml` files all into your crate's root directory.

4. Replace the contents of your `lib.rs` with the following:

```rust
#![no_std]

#[no_mangle]
pub unsafe extern "C" fn setup() {

}

#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {

}
```

5. Finally, compile the crate:
```
cargo build --release
```

If the crate compiles successfully, the built static library will be at `./target/arduboy/release/lib*.a`. Of course, this code doesn't do anything yet; the `arduboy-rs` directory in this repo is a rust library that provides some basic functionality you can use. See the `examples` directory for example use of the crate.

## Linking the static library to the Arduino C++ project

Prerequisite: [PlatformIO IDE](https://platformio.org/install/ide) or [PlatformIO Core](https://platformio.org/install/cli)

The `ArduboyRustHost` directory in this repo contains a [PlatformIO](https://platformio.org/) project that acts as a proxy between Rust and any C++ Arduboy libraries we wish to use. If you look at `ArduboyRustHost/src/main.cpp`, you'll see it's just a bunch of function re-exports in an `extern "C"` block. The `arduboy-rs` directory in this repo is a rust library crate that consumes those re-exports, providing an API we can use in our crate from before.

To link in our static library, copy the static library we built (`./target/arduboy/release/lib*.a`) over to the directory `ArduboyRustHost/lib`. Then, modify the `build_flags` setting in `ArduboyRustHost/platformio.ini` to reflect the name of the file we copied. An example of how the `platformio.ini` might look (in this case the static library file is called `libdig.a`):

```ini
[env:arduboy]
platform = atmelavr
board = arduboy
framework = arduino

build_flags =
	-L lib -l libdig

lib_deps =
    Arduboy2@^5.2.1
    ArduboyTones@^1.0.3
```

The `ArduboyRustHost` project can then be built and uploaded to an Arduboy as usual, either through the PlatformIO IDE, or through the commandline through `pio` (ran from inside the `ArduboyRustHost` directory:

To build: `pio run`

To build and upload: `pio run -t upload`

To clean build files: `pio run -t clean`

---

## Credits

Thanks to @simon-i1-h who wrote [arduboy-hello-rs](https://github.com/simon-i1-h/arduboy-hello-rs), the proof of concept that inspired me to try Rust on the Arduboy.

## License

Licensed under either of

- Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
