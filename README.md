# parse

A terminal command to replace pattern matches in a string given with a replacement string given. As a critique on the sed command, replace aims to be more explicit and readable.

## Build Instructions

To build the project, you need to have Rust installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).

Once you have Rust installed, you can build the project using the following command:

```sh
cargo build --release
```

This will create an executable in the `target/release` directory.

## Usage

The `replace` command takes a pattern, a replacement string, and a haystack string to search within. It also supports additional options to control the replacement behavior.

### Basic Usage

```sh
replace --pattern <PATTERN> --replacement <REPLACEMENT> --haystack <HAYSTACK>
```

### Options

- `--pattern <PATTERN>`: The regex pattern to match (required).
- `--replacement <REPLACEMENT>`: The string to replace matches with (required).
- `--haystack <HAYSTACK>`: The string to search within (required).
- `--all`: Replace all matches of the pattern.
- `--every_nth <EVERY_NTH>`: Replace every nth match of the pattern.
- `--nth <NTH>`: Replace only the nth match of the pattern.

### Examples

Replace the first occurrence of "foo" with "bar" in the string "foo foo foo":

```sh
replace --pattern "foo" --replacement "bar" --haystack "foo foo foo"
```

Expected result:

```text
bar foo foo
```

Replace all occurrences of "foo" with "bar" in the string "foo foo foo":

```sh
replace --pattern "foo" --replacement "bar" --haystack "foo foo foo" --all
```

Expected result:

```text
bar bar bar
```

Replace every second occurrence of "foo" with "bar" in the string "foo foo foo foo":

```sh
replace --pattern "foo" --replacement "bar" --haystack "foo foo foo foo" --every_nth 2
```

Expected result:

```text
foo bar foo bar
```

Replace only the third occurrence of "foo" with "bar" in the string "foo foo foo foo":

```sh
replace --pattern "foo" --replacement "bar" --haystack "foo foo foo foo" --nth 3
```

Expected result:

```text
foo foo bar foo
```

### Next Steps

- [ ] Add --file option
- [ ] Add --between option
- [ ] Rename to parse, update README
- [ ] Make replacement optional. If not provided, behave as find.
- [ ] Add --whole_line option to consider the whole line as the match. Whole lines when using - [ ]-between.
- [ ] Add --count option
- [ ] Add support for negative nth and every_nth values
- [ ] Add explanation in README for no --and and --or options. Logic operators should be in the calling code.
