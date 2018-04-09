# `FRDM-KL25Z using RUST`

This project aims to follow Rust's naming conventions: https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

## Getting it to run:

### Install toolchain (Mac instructions):

#### Install arm toolchain
brew cask install gcc-arm-embedded

#### Install tools used to talk to the board
brew install minicom openocd

#### Use Rust nightly since some features needed are not stable yet
rustup default nightly

#### Install Xargo for an easy cross compiling solution
cargo install xargo

#### Get rust-src so we can get pretty source maps
rustup component add rust-src

### Compile and Run

#### Compile the ELF using the following inside the lab01 folder:

xargo build --target thumbv6m-none-eabi --release

#### Generate a .bin file from the elf:

arm-none-eabi-objcopy -S -O binary ./target/thumbv6m-none-eabi/release/lab01 lab01.bin

#### Either Flash 

Copy the bin file to the FRDM-KL25Z mounted as USB mass storage with the MBED 0243 Firmware or latter

#### Or Flash and Debug

openocd -f board/frdm-kl25z.cfg **will block, leave terminal open**

arm-none-eabi-gdb target/thumbv6m-none-eabi/release/cortex-m-quickstart -tui 


**Type continue and see the program run**


## What works:

GPIOs of PORTs

Serial communication with interrupts

# Based upon:

# [cortex-m-quickstart](https://docs.rs/cortex-m-quickstart) and [Blinky demo for FRDM-KL25Z](https://github.com/0xc0170/frdm-kl25z-rust)

Special thanks to Japaric and the Rust embedded community for helping me create a proper bin file: [Flash Config Bin Issue](https://github.com/japaric/cortex-m-quickstart/issues/25)

Also my teacher, [Denis Loubach](https://github.com/dloubach) for letting me use RUST over C for this project.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
