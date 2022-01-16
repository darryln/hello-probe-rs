#![no_main]
#![no_std]

use hello as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    loop {
        hello::exit();
    }
}
