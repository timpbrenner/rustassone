# Rustassone

The game Carcassone with a Rust backend and a Vue.js frontend

## Installation
 1.) Install [Rust](https://www.rust-lang.org/en-US/install.html) with `curl https://sh.rustup.rs -sSf | sh`
 2.) Run `cargo build` to install dependencies
 3.) Run `cargo install diesel_cli`
 4.) Put database in env file with command `echo DATABASE_URL=postgres://username@localhost/carcassone > .env`
 5.) Run `diesel setup` and `diesel migration run`
 5.) Run `cargo run` to run the server.
 6.) Proceed to http://localhost:8000
