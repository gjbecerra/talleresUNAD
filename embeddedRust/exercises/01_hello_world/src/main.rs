#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    
    rprintln!("Hola mundo!");

    loop {}
}