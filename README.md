# count_files
A simple command line tool to count files by extension.

## How to use
Copy the executable file to your `$PATH` and run:

```bash
count_files /path/to/target_dir
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

## Building
count_files is written in Rust, you need to [install Rust](https://www.rust-lang.org/tools/install) to compile it.

To build:

```bash
git clone https://github.com/yinguobing/count_files.git
cd count_files
cargo build --release
```