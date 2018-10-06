#![no_std]
#![no_main]

extern crate panic_abort;
extern crate stm32f103xx_hal as hal;

use self::hal::prelude::*;
use self::hal::stm32f103xx;
use self::hal::i2c;

use cortex_m_rt::entry;

// Work around bug where including the mode in the main function would make
// all errors warnings point to the #[entry] macro and hide the content of the
// main function from the debugger.
fn get_mode() -> i2c::Mode {
    return i2c::Mode::Standard { frequency: 100000 };
}

#[entry]
fn main() -> ! {
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = i2c::BlockingI2c::i2c2(
        dp.I2C2,
        (scl, sda),
        i2c::Mode::Standard { frequency: 100000 },
//        get_mode(),
        clocks,
        &mut rcc.apb1,
        10000,
        3,
        20000,
        20000,
    );
    loop {}
}
