# count-files
A simple command line tool to count files by extension.

## How to use
Open a terminal and run:
```bash
count-files /path/to/target_dir
```

Output sample:
```bash
Done in 0 seconds.
+-----------+-------+
| File type | Count |
+===================+
| jpg       |    29 |
|-----------+-------|
| jpeg      |     7 |
|-----------+-------|
| txt       |     6 |
|-----------+-------|
| gz        |     4 |
|-----------+-------|
| png       |     2 |
+-----------+-------+
```

## Installation
If you have Rust installed, then use cargo:

```bash
cargo install count-files
```

Or, you can build it manually following the next section, then copy the executable file into the `$PATH` directory.

```bash
# Install for all users on Ubuntu.
sudo cp target/release/count-files /usr/local/bin
```

## Building
count-files is written in Rust, you need to [install Rust](https://www.rust-lang.org/tools/install) to compile it.

To build:

```bash
git clone https://github.com/yinguobing/count-files.git
cd count-files
cargo build --release
```

The executable file `count-files` could be found in `target/release` directory.
