# seer

![seer_logo](resources/logo.png)

A terminal command to <ins>S</ins>earch, <ins>E</ins>dit, <ins>E</ins>valuate, and <ins>R</ins>eplace text.

## Build Instructions

To build the project, you need to have Rust installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).

Once you have Rust installed, you can build the project using the following command:

```sh
cargo build --release
```

This will create an executable in the `target/release` directory.

## Usage

The `seer` command takes a pattern to find and takes searchable text. Additional options can be given to edit, evaluate, and replace that text.

### Basic Usage

```sh
seer --find_exact <PATTERN> --file <FILE_PATH>
```
