#!/bin/sh

cd "$(cd "$(dirname "$0")" && pwd)"
cargo build --release
cp ./target/release/weather_ssh ./
./weather_ssh