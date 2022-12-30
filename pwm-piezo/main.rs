#![no_std]
#![no_main]

use std::thread::sleep;

use esp32c3_hal as _; // For linker symbol `_start_trap_hal`
use esp32c3_hal::{clock::ClockControl, pac::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO};

use esp_backtrace as _;

#[riscv_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    // Disable the RTC and TIMG watchdog timers
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    esp_println::println!("Started");

    // Set GPIO7 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio7.into_push_pull_output();

    led.set_high().unwrap();

    loop {}
}
