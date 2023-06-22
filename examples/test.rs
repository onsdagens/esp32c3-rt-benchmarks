#![no_main]
#![no_std]
#![feature(naked_functions)]
#![feature(linkage)]
// /#![feature(linkage)]
use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};
use esp32c3_hal::{entry, prelude::ram};
use rtt_target::{rprintln, rtt_init_print};
#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();
    rprintln!("init");
    asm!(
        "
    la ra, _start_trap8
    jalr ra, ra
    "
    );
    loop {}
}

#[export_name = "_start_trap8"]
#[ram]
fn hello() {
    rprintln!("hi!");
}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
#[naked]
unsafe extern "C" fn fetch_performance_timer() -> i32 {
    asm!(
        "
    csrr a0, 0x7e2
    jr ra
    ",
        options(noreturn)
    );
}
// #[ram]
// #[export_name="_weaksym"]
// #[linkage="weak"]
// fn _weaksym(){
//     loop{}
// }
// global_asm!("
// .section .trap, \"ax\"
// .weak _weaksym
// #.set _weaksym,_some_label

// #_weaksym:
//     j _weaksym
// ");
