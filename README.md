# Double Pendulum
An optimized double pendulum simulator written in Rust and ggez

## Installation
You'll need a working Rust toolchain and git installed on your computer.
```sh
git clone https://github.com/kugiyasan/double_pendulum.git
cd double_pendulum
cargo run
# If you want an optimized build on Linux or MacOS
RUSTFLAGS="-C target-cpu=native" cargo run --release
# If you want an optimized build on Windows
set RUSTFLAGS=-C target-cpu=native
cargo run --release
```

## Controls
All the controls are listed at the bottom of `src/mainstate.rs`

- C: Create a new `DoublePendulum`
- R: Reset the simulation back to one pendulum
- T: Toggle the trail
- Q: Quit the program

## Settings
You can change every constants declared at the top of each file to modify various things, such as the screen resolution and the framerate.

## Known bugs
On linux, you can't move the cursor over the program window or it will crash. This is a [known issue](https://github.com/ggez/ggez/issues/843). You can either don't move your mouse over the program or use rustc <= 1.47. Ironically, I'm using `VecDeque::make_contiguous`, which is a new [feature](https://github.com/rust-lang/rust/issues/70929) since 1.48, so you'll also need to add `#![feature(deque_make_contiguous)]` at the top of `src/pendulum.rs` in you use that second fix.
