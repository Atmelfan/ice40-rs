#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate ice40;
extern crate panic_rtt_target;
extern crate rtt_target;
extern crate stm32h7xx_hal;

use cortex_m_rt::entry;
use rtt_target::rprintln;
use stm32h7xx_hal::delay::Delay;
use stm32h7xx_hal::gpio::Speed;
use stm32h7xx_hal::hal::delay::DelayNs;
use stm32h7xx_hal::pac::Peripherals;
use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::spi::{self, Spi};

const FPGA_CONFIG: &[u8] = include_bytes!("../../data/ice40_simple_configuration.bin");

const SPI_PIN_SPEED: Speed = Speed::Medium;

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    #[allow(clippy::empty_loop)]
    let dp = Peripherals::take().unwrap_or_else(|| loop {});
    #[allow(clippy::empty_loop)]
    let cp = cortex_m::Peripherals::take().unwrap_or_else(|| loop {});
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc
        .sys_ck(240.MHz())
        .pll1_q_ck(100.MHz())
        .freeze(pwrcfg, &dp.SYSCFG);

    // GPIO
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Status-indicating LEDs
    let mut led_green = gpiob.pb0.into_push_pull_output();
    let mut led_yellow = gpioe.pe1.into_push_pull_output();
    let mut led_red = gpiob.pb14.into_push_pull_output();

    // PA15 output -> SS
    let mut ss = gpioa.pa15.into_push_pull_output();
    // PB9 output -> CRESET
    let mut creset = gpiob.pb9.into_push_pull_output();
    // PA6 input -> CDONE
    let cdone = gpioa.pa6.into_pull_down_input();

    // Set up SPI
    let (sck, miso, mosi) = (
        gpiob.pb3.into_alternate::<5>().speed(SPI_PIN_SPEED),
        gpiob.pb4.into_alternate::<5>().speed(SPI_PIN_SPEED),
        gpiob.pb5.into_alternate::<5>().speed(SPI_PIN_SPEED),
    );
    let spi: Spi<_, _, u8> = dp.SPI1.spi(
        (sck, miso, mosi),
        spi::MODE_0,
        20.MHz(),
        ccdr.peripheral.SPI1,
        &ccdr.clocks,
    );
    rprintln!("Initialized GPIO and SPI");
    ss.set_high();
    creset.set_high();
    let mut delay = Delay::new(cp.SYST, ccdr.clocks);
    let mut ice40 = ice40::Device::new(spi, ss, cdone, creset);
    rprintln!("Created ice40");

    loop {
        led_green.set_low();
        led_yellow.set_high();
        led_red.set_low();
        let status = ice40.configure(&mut delay, FPGA_CONFIG);
        match status {
            Ok(()) => {
                rprintln!("Successfully configured FPGA");
                led_yellow.set_low();
                led_green.set_high();
            }
            Err(e) => {
                rprintln!("Failed to configure FPGA: {:?}", e);
                led_yellow.set_low();
                led_red.set_high();
            }
        }
        delay.delay_ms(500);
    }
}
