#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_probe as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    for x in 1..=5 { 
        rprintln!("{}", x)
    }

    rprintln!("finished!");

    loop {
        asm::bkpt();
        //asm::udf();
    }
}
