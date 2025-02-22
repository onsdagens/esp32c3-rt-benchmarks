#![no_main]
#![no_std]
#![feature(naked_functions)]
use core::{arch::asm, cell::RefCell, panic::PanicInfo};
use critical_section::Mutex;
use esp32c3_hal::{
    clock::ClockControl,
    interrupt::CpuInterrupt,
    interrupt::{self},
    peripherals::{self, Peripherals},
    prelude::*,
    system::{SoftwareInterrupt, SoftwareInterruptControl},
    timer::TimerGroup,
    Rtc,
};
use esp_println::println;

//use panic_rtt_target as _;
//use rtt_target::{rprintln, rtt_init_print};

static SWINT: Mutex<RefCell<Option<SoftwareInterruptControl>>> = Mutex::new(RefCell::new(None));
#[entry]
unsafe fn main() -> ! {
    //    rtt_init_print!();
    println!("init");
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clockctrl = system.clock_control;
    let sw_int = system.software_interrupt_control;
    let clocks = ClockControl::boot_defaults(clockctrl).freeze();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDTs.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    critical_section::with(|cs| SWINT.borrow_ref_mut(cs).replace(sw_int));
    interrupt::enable(
        peripherals::Interrupt::FROM_CPU_INTR0,
        interrupt::Priority::Priority3,
        CpuInterrupt::Interrupt1,
    )
    .unwrap();
    unsafe {
        asm!(
            "
        csrrwi x0, 0x7e0, 1 #what to count, for cycles write 1 for instructions write 2
        csrrwi x0, 0x7e1, 0 #disable counter
        csrrwi x0, 0x7e2, 0 #reset counter
        "
        );
    }
    println!("MPC:{}", unsafe { fetch_performance_timer() });
    //interrupt is raised from assembly for max timer granularity.
    unsafe {
        asm!(
            "
        li t0, 0x600C0028 #FROM_CPU_INTR0 address
        li t1, 1    #Flip flag
        csrrwi x0, 0x7e1, 1 #enable timer
        sw t1, 0(t0) #trigger FROM_CPU_INTR0
        "
        )
    }

    println!("Returned");
    loop {}
}

#[no_mangle]
fn cpu_int_1_handler() {
    unsafe { asm!("csrrwi x0, 0x7e1, 0 #disable timer") }
    critical_section::with(|cs| {
        SWINT
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .reset(SoftwareInterrupt::SoftwareInterrupt0);
    });
    println!("Performance counter:{}", unsafe {
        fetch_performance_timer()
    });
}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
unsafe extern "C" fn fetch_performance_timer() -> i32 {
    let x;
    asm!(
        "
    csrr {reg}, 0x7e2
    ",
        reg = out(reg) x,
    );
    x
}

// This is to satisfy the linker due to a broken PAC dependency.
// One could also fork the PAC and fix it upstream,
// but this crate at this point relies on a 1 year old version of the PAC,
// and we are not touching this at runtime in any case.
#[allow(non_snake_case)]
extern "C" {
    fn DefaultHandler();
}
#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn USB_DEVICE() {
    unsafe { DefaultHandler() }
}
