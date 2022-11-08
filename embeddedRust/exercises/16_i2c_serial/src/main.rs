#![no_main]
#![no_std]

use core::str;

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

use microbit::hal::prelude::*;
use lsm303agr::{AccelOutputDataRate, MagOutputDataRate, Lsm303agr};
use heapless::Vec;
use nb::block;
use core::fmt::Write;

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        let mut buffer: Vec<u8, 32> = Vec::new();

        loop {
            let byte = block!(serial.read()).unwrap();
            nb::block!(serial.write(byte)).unwrap();
            nb::block!(serial.flush()).unwrap();

            if byte == 13 {
                write!(serial, "\r\n").unwrap();
                break;
            }

            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer lleno\r\n").unwrap();
                break;
            }
        }

        if str::from_utf8(&buffer).unwrap().trim() == "acc" {
            while !sensor.accel_status().unwrap().xyz_new_data  {
            }

            let data = sensor.accel_data().unwrap();
            write!(serial, "Acelerómetro: x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
        } else if str::from_utf8(&buffer).unwrap().trim() == "mag" {
            while !sensor.mag_status().unwrap().xyz_new_data  {
            }

            let data = sensor.mag_data().unwrap();
            write!(serial, "Magnetómetro: x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
        } else {
            write!(serial, "error: comando incorrecto\r\n").unwrap();
        }
    }
}