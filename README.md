# seer

![seer_logo](resources/logo.png)

A terminal command to <ins>S</ins>earch, <ins>E</ins>dit, <ins>E</ins>valuate, and <ins>R</ins>eplace text.

## Processing Phases

Seer operates in 4 distinct phases:
1. **Source** - Define where text comes from (file, directory, pipe, etc.)
2. **Find** - Locate patterns within the source text 
3. **Transform** - Modify or analyze the found patterns (edit/evaluate operations)
4. **Output** - Display or save the results

## Build Instructions

To build the project, you need to have Rust installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).

Once you have Rust installed, you can build the project using the following command:

```sh
cargo build --release
```

This will create an executable in the `target/release` directory.

## Usage

The `seer` command takes a pattern to find and takes searchable text. Additional Transform phase operations can be applied to edit, evaluate, and replace that text.

Transform operations include:
- **Edit operations**: `append`, `prepend`, `replace-with` (modify found text)
- **Evaluate operations**: `count`, `count-by-source` (analyze found patterns)

Note: Edit and Evaluate operations cannot be used together in the same command.

### Basic Usage

```sh
seer --find_exact <PATTERN> --file <FILE_PATH>
```
