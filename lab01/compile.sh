xargo build --target thumbv6m-none-eabi --release &&
rm lab01.bin
arm-none-eabi-objcopy -S -O binary ./target/thumbv6m-none-eabi/release/lab01 lab01.bin && 
cp lab01.bin /Volumes/DAPLINK/
