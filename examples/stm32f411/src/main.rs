#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate ice40;
extern crate panic_rtt_target;
extern crate rtt_target;
extern crate stm32f4xx_hal;

use cortex_m_rt::entry;
use rtt_target::rprintln;
use stm32f4xx_hal::hal::delay::DelayNs;
use stm32f4xx_hal::pac::Peripherals;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::spi::{Mode, Phase, Polarity};

const FPGA_CONFIG: &[u8] = include_bytes!("../../data/ice40_simple_configuration.bin");

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    #[allow(clippy::empty_loop)]
    let dp = Peripherals::take().unwrap_or_else(|| loop {});
    #[allow(clippy::empty_loop)]
    let cp = cortex_m::Peripherals::take().unwrap_or_else(|| loop {});

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(100.MHz()).pclk2(100.MHz()).freeze();

    // GPIO
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();

    // Status-indicating LEDs
    let mut led_green = gpiod.pd12.into_push_pull_output();
    let mut led_orange = gpiod.pd13.into_push_pull_output();
    let mut led_red = gpiod.pd14.into_push_pull_output();

    // PB8 output -> SS
    let mut ss = gpioc.pc6.into_push_pull_output();
    // PB9 output -> CRESET
    let mut creset = gpioc.pc8.into_push_pull_output();
    // PC9 input -> CDONE
    let cdone = gpioc.pc9.into_pull_down_input();

    // Set up SPI
    let (sck, miso, mosi) = (
        gpioc.pc7.into_alternate::<5>(),
        gpioc.pc2.into_alternate::<5>(),
        gpioc.pc3.into_alternate::<5>(),
    );
    let spi = dp.SPI2.spi(
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleHigh,
            phase: Phase::CaptureOnSecondTransition,
        },
        15.MHz(),
        &clocks,
    );
    rprintln!("Initialized GPIO and SPI");
    ss.set_high();
    creset.set_high();
    let mut delay = cp.SYST.delay(&clocks);
    delay.delay_ms(10);

    let mut ice40 = ice40::Device::new(spi, ss, cdone, creset);
    rprintln!("Created ice40");

    loop {
        led_green.set_low();
        led_orange.set_high();
        led_red.set_low();
        let status = ice40.configure(&mut delay, FPGA_CONFIG);
        match status {
            Ok(()) => {
                rprintln!("Successfully configured FPGA");
                led_orange.set_low();
                led_green.set_high();
            }
            Err(e) => {
                rprintln!("Failed to configure FPGA: {:?}", e);
                led_orange.set_low();
                led_red.set_high();
            }
        }
        delay.delay_ms(500);
    }
}
