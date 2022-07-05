# Description
Fully working example of a **RUST**  _no-std_ console appliation  with print functionality and  Vectors

# Compile on windows

### Toolchain needed
Note: Using the following toolchain requires MinGW/bin in your PATH enviroment variable

```sh
rustup toolchain install nightly-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu`
```

### Compile and Run
```sh
cargo run --release
```


# Dependencies
- [https://github.com/daniel5151/libc_alloc](https://github.com/daniel5151/libc_alloc) 
- [https://github.com/mmastrac/rust-libc-print](https://github.com/mmastrac/rust-libc-print)
