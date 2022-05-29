# count-files

A simple command line tool to count files by extension.

[![Rust](https://github.com/yinguobing/count-files/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/yinguobing/count-files/actions/workflows/rust.yml)

## How to use
Open a terminal and run:
```bash
count-files /path/to/target_dir
```

Output sample:
```bash
Done in 7 seconds.
+-----------+---------+------------+
| File type |   Count | Total size |
+==================================+
| jpg       | 1322497 |  112.48GiB |
|-----------+---------+------------|
| txt       |  372302 |   97.68MiB |
|-----------+---------+------------|
| JPG       |     147 |   26.04MiB |
|-----------+---------+------------|
| bmp       |     130 |  714.12MiB |
|-----------+---------+------------|
| tar       |       9 |  114.88GiB |
+-----------+---------+------------+
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
