> A minimalist reimplementation of common Unix core utilities — in Rust for educational purposes.
## Prerequisites
- Rust toolchain (Install from https://rustup.rs/)

## Run
```bash
git clone https://github.com/0xVergil/rust_shell.git
cd rust_shell
cargo r
```

## Supported Commands
- cd
- echo
- ls
   - -a
   <!-- - -l -->
- cat
- mkdir
- exit
- pwd
   <!-- - L -->
   <!-- - P -->
- rm
   - -r
- rmdir
- cp
   - -r/R
- mv
- clear
<!-- - wc
- head & tail
   - -n
   - -F -->


## Try edge cases

- Files starting with - (`rm -- -file`)

- Recursive deletion (`rm -r dir`)

- cd to previous directory (`cd -`)

<!-- - Combined flags (`ls -lFa`) -->


## License
MIT — free to use, learn from, and modify.