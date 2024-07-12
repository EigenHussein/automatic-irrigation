
cargo clean --package esp-idf-hal
cargo clean --package esp-idf-svc
cargo clean --package esp-idf-sys


cargo clean --package esp-idf-hal --release
cargo clean --package esp-idf-svc --release
cargo clean --package esp-idf-sys --release
