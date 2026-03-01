
# rwc

A Rust implementation of the Unix `wc` utility. Counts lines, words, bytes, and characters in one or more files, or from stdin.

## Usage

```sh
rwc [OPTIONS] [FILES]...
```

## Options

| Flag          | Description               |
|---------------|---------------------------|
| `-l, --lines` | Print the line count      |
| `-w, --words` | Print the word count      |
| `-c, --bytes` | Print the byte count      |
| `-m, --chars` | Print the character count |

If no flags are provided, `-l`, `-w`, and `-c` are enabled by default — matching the behaviour of `wc`.

## Input

**From files:**
```sh
rwc file.txt
rwc -l -w file1.txt file2.txt
```

**From stdin:**
```sh
echo "hello world" | rwc
cat file.txt | rwc -l
```

If neither files nor stdin input are provided, the program will exit with an error.

## Example Output

```sh
$ echo "hello world" | rwc
stdin: Byte count: 12
stdin: Line count: 1
stdin: Word count: 2
```

## Building

Requires [Rust](https://rustup.rs).

```sh
cargo build --release
```

The compiled binary will be at `target/release/rwc`.

## Contributing

If you see a problem or improvement that can be made, please open up an issue to discuss it.

## License

Copyright© 2026 Ryan Hendrickson. Released under the MIT License. See [ICENSE](LICENSE) for details.