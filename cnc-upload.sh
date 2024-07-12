#!/usr/bin/bash
cargo espflash flash --baud 460800 --bin cnc-factory --release
espflash save-image --chip esp32 target/xtensa-esp32-espidf/release/cnc-factory target/xtensa-esp32-espidf/release/cnc-factory.bin
espflash save-image --chip esp32 target/xtensa-esp32-espidf/release/cnc-ota target/xtensa-esp32-espidf/release/cnc-ota.bin
#espflash write-bin 0x10000 target/xtensa-esp32-espidf/debug/factory.bin

espflash write-bin 0x130000 target/xtensa-esp32-espidf/release/cnc-ota.bin --baud 460800
cargo espflash monitor