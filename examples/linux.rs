use hal::{spidev::SpidevOptions, sysfs_gpio::Direction};
use std::{fs, path::PathBuf, thread::sleep, time::Duration};
use structopt::StructOpt;

use embedded_hal::prelude::*;
use linux_embedded_hal as hal;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(
        long,
        parse(from_os_str),
        default_value = "/dev/spidev0.0",
        help = "SPI bus"
    )]
    spi: PathBuf,

    #[structopt(long, default_value = "8", help = "SS pin")]
    ss: u64,

    #[structopt(long, default_value = "25", help = "CRESET pin")]
    creset: u64,

    #[structopt(long, default_value = "24", help = "CDONE pin")]
    cdone: u64,

    /// Set speed
    #[structopt(short, long, default_value = "3000000", help = "Bus frequency")]
    frequency: u32,

    /// Input file
    #[structopt(parse(from_os_str), help = "Binary file")]
    binary: PathBuf,
}

struct DummyDelay;
impl embedded_hal::blocking::delay::DelayUs<u16> for DummyDelay {
    fn delay_us(&mut self, us: u16) {
        sleep(Duration::from_micros(us.into()))
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    let spiopt = SpidevOptions::new()
        .max_speed_hz(opt.frequency)
        .lsb_first(false)
        .build();

    let bitstream = fs::read(opt.binary).expect("Failed to read binary file");
    log::info!("Read binary file, size = {}", bitstream.len());

    let mut spi = hal::Spidev::open(opt.spi).expect("Failed to open SPI bus");
    spi.configure(&spiopt).expect("Failed to configure SPI bus");
    let ss = hal::Pin::new(opt.ss);
    ss.export().expect("Failed to export SS pin");
    ss.set_direction(Direction::Out).unwrap();
    let done = hal::Pin::new(opt.cdone);
    done.export().expect("Failed to export CDONE pin");
    done.set_direction(Direction::Out).unwrap();
    let reset = hal::Pin::new(opt.creset);
    reset.export().expect("Failed to export CRESET pin");
    reset.set_direction(Direction::Out).unwrap();

    log::info!("Configuring device...");
    let mut device = ice40::Fpga::new(spi, ss, done, reset, DummyDelay);
    device
        .configure(&bitstream[..])
        .expect("Failed to configure FPGA");
    log::info!("done!");
}
