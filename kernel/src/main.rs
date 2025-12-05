#![no_std]
#![no_main]

mod utils;
mod boot;

use crate::utils::console;

/// clear BSS segment
pub fn clear_bss() {
    unsafe extern "C" {
        unsafe fn sbss();
        unsafe fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn main() {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}
