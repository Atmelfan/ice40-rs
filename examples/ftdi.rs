use std::{fs, path::PathBuf, thread::sleep, time::Duration};
use structopt::StructOpt;

use ftdi_embedded_hal as hal;

#[derive(Debug, StructOpt)]
#[structopt(name = concat!(env!("CARGO_PKG_NAME"), "/ftdi"), about = "FTDI demo")]

struct Opt {
    /// Set speed
    #[structopt(short, long, default_value = "3000000", help = "Bus frequency")]
    frequency: u32,

    /// Input file
    #[structopt(parse(from_os_str), help = "Binary file")]
    binary: PathBuf,
}

struct DummyDelay;
impl embedded_hal::delay::DelayNs for DummyDelay {
    fn delay_ns(&mut self, ns: u32) {
        sleep(Duration::from_nanos(ns.into()))
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    let bitstream = fs::read(opt.binary).expect("Failed to read binary file");
    log::info!("Read binary file, size = {}", bitstream.len());

    let device: libftd2xx::Ft2232h = libftd2xx::Ftdi::new()
        .unwrap()
        .try_into()
        .expect("Failed to open device");
    log::info!("Connected to FT2232");

    let hal = hal::FtHal::init_freq(device, opt.frequency).expect("Failed to init device");
    let spi = hal.spi().unwrap();
    let ss = hal.ad4().unwrap();
    let done = hal.adi6().unwrap();
    let reset = hal.ad7().unwrap();

    log::info!("Configuring device...");
    let mut device = ice40::Device::new(spi, ss, done, reset);
    device
        .configure(&mut DummyDelay, &bitstream[..])
        .expect("Failed to configure FPGA");
    log::info!("done!");
}
