#!/usr/bin/bash

espflash save-image --chip esp32 target/xtensa-esp32-espidf/debug/factory target/xtensa-esp32-espidf/debug/factory.bin
espflash save-image --chip esp32 target/xtensa-esp32-espidf/debug/main target/xtensa-esp32-espidf/debug/main.bin
#espflash write-bin 0x10000 target/xtensa-esp32-espidf/debug/factory.bin
#cargo espflash flash --bin factory --baud 460800
espflash write-bin 0x110000 target/xtensa-esp32-espidf/debug/main.bin --baud 460800 --no-stub
cargo espflash monitor