# Description
Fully working example of a **RUST**  _no-std_ console application with print functionality and Vectors

# Compiling

### Prerequisites for Windows users (MinGW-64) 
1- Install _msys2-x86_64-latest.exe_ from [https://repo.msys2.org/distrib/](https://repo.msys2.org/distrib/)

2- Once installed open MSYS console and install the compiler
```sh
pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-gdb mingw-w64-x86_64-make
```
3- Set C:\msys64\mingw64\bin\  (os where you installed MingW64)  folder on your PATH enviroment variable

```sh
set PATH=%PATH%;C:\msys64\mingw64\bin\
```


### Install Rust GNU Toolchain 

```sh
rustup toolchain install nightly-x86_64-pc-windows-gnu
rustup default nightly-x86_64-pc-windows-gnu`
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
