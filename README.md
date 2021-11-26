# BeginnerRust
https://www.youtube.com/playlist?list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL

## Installation Guide 
https://www.rust-lang.org/lear/get-started

I first ran the command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

I also ran `rustup update` to make sure there are any updates

### First Steps with Cargo
To start a new package with Cargo, use `cargo new <package name>`
Use the `tree <package name>` to see the tree of files

The file Cargo.toml is called a manifest that contains all the metadata that Cargo needs to compile the package

To run the src/main.rs file we need to use `cargo run` which will compile and run all in one step
