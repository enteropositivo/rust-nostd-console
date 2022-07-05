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


# Code
src/main.rs

```rust
#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

// no-std print functions
use libc_print::*;

// allocator
use libc_alloc::LibcAlloc;
#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

// std compatibility
use no_std_compat::{prelude::v1::*, vec::Vec};


#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {

    libc_println!("no-std @Entero{}", "Positivo");

    let mut vec_test: Vec<u8> = Vec::new();
    vec_test.push(10);
    vec_test.push(20);
    vec_test.push(30);

    for v in vec_test{
        libc_println!("Vector-Item = {}", v);
    }

    0
}


#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    libc_println!("panic!:: {}", _info.to_string().as_str());
    
    loop {}
}
```
