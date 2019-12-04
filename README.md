# ArduboyRust

Running Rust on the [Arduboy](https://arduboy.com/) miniature game system.

## Building Rust with AVR support

Rust does not currently officially support the AVR architecture the Arduboy uses, though it hopefully will at some point ([see this issue](https://github.com/rust-lang/rust/issues/44052)). However, there is [a fork](https://github.com/avr-rust/rust) of the compiler with AVR support, which we’ll use.

### Prerequisites

You will need the following to build the compiler:

- Enough disk space. I don’t recall exactly how much is needed, but 120GB should be enough.
- Enough free system memory. 8GB free should be enough.
- The dependencies listed [here](https://github.com/rust-lang/rust/#installing-from-source).

### Building

1. Get the compiler source files
```
mkdir avr
cd avr
git clone https://github.com/avr-rust/rust.git -b avr-support --recursive
cd rust
```

2. Place the `config.toml` from this repo inside the current directory (the `rust` folder). On Windows, you might want to uncomment and set the `build =` `"``…``"` field to `x86_64-pc-windows-gnu` if you want to build using the GNU ABI.

3. Start the build using

```
python x.py build
```

This will take a while.

4. Assuming the build finished successfully, copy cargo from the `stage0` folder to the `stage2` folder:

```
cp build/<target-triple>/stage0/bin/cargo[.exe] build/<target-triple>/stage2/bin/
```

5. The only files we need to keep are the `build/<target-triple>/stage2` folder and the `src` folder. You might want to move these folders to the `avr` folder we made at the start. You can delete everything else in the current `rust` folder.

6. Register the toolchain with rustup (adjust the path if you moved/renamed the `stage2` folder):

```
rustup toolchain link avr build/<target-triple>/stage2
```

## Building a crate

The idea is to compile our Rust code as a static library, and then link it from an Arduino C++ project.

1. Install `[cargo-xbuild](https://crates.io/crates/cargo-xbuild)` if you don’t have it:

```
cargo install cargo-xbuild
```

2. Create a new library crate

3. Add the following fields to your crate’s `Cargo.toml`:

```toml
[lib]
crate-type = ["staticlib"]

[profile.release]
codegen-units = 1
lto = true
opt-level = "z"
panic = "abort"
```

4. Place the `arduboy.json` from this repo in your crate root.

5. Replace the contents of your `lib.rs` with the following:

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

6. Set the environment variable `XARGO_RUST_SRC` to the `src` folder of the `avr-rust` repo you cloned above.

7. Finally, compile the crate:
```
cargo +avr xb --target=arduboy.json --release
```

If the crate compiles successfully, the built static library will be at `target/arduboy/release/lib*.a`.

## Linking the static library to an Arduino C++ project

For creating the Arduino C++ project, I recommend using [PlatformIO](https://platformio.org/).

### Using PlatformIO (recommended)

After creating the PlatformIO project, to link in our static library, use the `build_flags` setting in the project `platformio.ini`. Assuming you placed the built static library file `lib*.a` inside the project `lib` folder, you would add `-L lib -l lib*` to the build flags. An example of how the `platformio.ini` might look (in this case the static library file is called `libdig.a`):

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

### Using the Arduino IDE (hacky, not recommended)

I edited `%LOCALAPPDATA%/Arduino15/packages/arduino/hardware/avr/1.8.1/platform.txt` and added the path to the built `*.a` file in this section like this:

```
## Combine gc-sections, archives, and objects
recipe.c.combine.pattern="{compiler.path}{compiler.c.elf.cmd}" {compiler.c.elf.flags} -mmcu={build.mcu} {compiler.c.elf.extra_flags} -o "{build.path}/{build.project_name}.elf" {object_files} "{build.path}/{archive_file}" C:/Users/seekr/projects/rust/Arduboy/arduboy-hello-rs/target/arduboy/release/libhello.a "-L{build.path}" -lm
```

Note that this will apply to all sketches, so remove it when compiling other sketches.

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
