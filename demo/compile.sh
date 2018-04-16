#!/usr/bin/env bash
cargo build --release &&
rm demo.bin
arm-none-eabi-objcopy -S -O binary ./target/thumbv6m-none-eabi/release/demo demo.bin && 
cp demo.bin /Volumes/DAPLINK/
