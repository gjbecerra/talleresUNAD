#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    loop {
        if board.buttons.button_a.is_low().unwrap() {
            rprintln!("Boton A presionado.");
        }
        if board.buttons.button_b.is_low().unwrap() {
            rprintln!("Boton B presionado.");
        }
    }
}