# To_Do Application using web assembly.
- To Do Application using WASM and Yew framework(Elm programmed in rustlang).
- It uses model-view-controller(MVC) programming model and reactive programming as Elm does.

## Install Dependencies
- $ cargo install cargo-web
- $ rustup target add asmjs-unknown-emscripten
- $ rustup target add wasm32-unknown-emscripten

## Run Application (Optimized Build)
- $ cargo web start --target-webasm-emscripten=wasm32-unknown-emscripten --release

[![Rust](https://github.com/shikharvashistha/to_do/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/shikharvashistha/to_do/actions/workflows/rust.yml)
