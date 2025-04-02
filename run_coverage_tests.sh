#!/bin/sh

export RUSTFLAGS="-C instrument-coverage"
cargo test

$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -sparse *.profraw -o default.profdata
$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov show -Xdemangler=rustfilt target/debug/coverage-testing -instr-profile=default.profdata -show-line-counts-or-regions -show-instantiations