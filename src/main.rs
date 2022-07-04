#![no_std]
#![no_main]

//extern crate no_std_compat as std;

use core::panic::PanicInfo;





#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut x = 6;
    x= x*2;
    x=x+1;

   
    

    loop {}
}