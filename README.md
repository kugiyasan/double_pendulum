# Double Pendulum
An optimized double pendulum simulator written in Rust and ggez

## Installation
You'll need a working Rust toolchain and git installed on your computer.
```sh
git clone https://github.com/kugiyasan/double_pendulum.git
cd double_pendulum
cargo run
# If you want an optimized build on UNIX-like systems
RUSTFLAGS="-C target-cpu=native" cargo run --release
# If you want an optimized build on windows
set RUSTFLAGS=-C target-cpu=native
cargo run --release
```

## Controls
All the controls are listed at the bottom of `src/mainstate.rs`

C: Create a new `DoublePendulum`

R: Reset the simulation back to one pendulum

T: Toggle the trail

Q: Quit the program
