#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use blog_os::println;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    blog_os::gdt::init();
    blog_os::interrupts::init_idt();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
