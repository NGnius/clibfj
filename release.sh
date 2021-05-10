#!/bin/bash
cargo build --target x86_64-pc-windows-gnu --release
strip target/x86_64-pc-windows-gnu/release/libfj.dll -o libfj.dll
