#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::hal::temp::Temp;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = microbit::Peripherals::take().unwrap(); 
    let mut temp = Temp::new(dp.TEMP);

    loop {
        let t = temp.measure();
        rprintln!("Temp: {} °C", t);
    }
}