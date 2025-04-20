[package]
name = "stabl"
version = "0.1.2"
edition = "2021"

[dependencies]
chrono = "0.4.40"
regex = "1.11.1"
ittapi = "0.3"

[profile.release]
debug = 1

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(test)'] }

[build]
rustflags = ["-C", "symbol-mangling-version=v0"]