use std::{fs, path::PathBuf, thread::sleep, time::Duration};
use structopt::StructOpt;

use ftdi_embedded_hal as hal;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "example",
    about = "Configure a ice40 device over an FTDI2232 USB-SPI bridge."
)]
struct Opt {
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

    let bitstream = fs::read(opt.binary).expect("Failed to read binary file");
    log::info!("Read binary file, size = {}", bitstream.len());

    let device = ftdi::find_by_vid_pid(0x0403, 0x6010)
        .interface(ftdi::Interface::A)
        .open()
        .expect("Failed to open device");
    log::info!("Connected to FT2232");

    let hal = hal::FtHal::init_freq(device, opt.frequency).expect("Failed to init device");
    let spi = hal.spi().unwrap();
    let ss = hal.ad4().unwrap();
    let done = hal.adi6().unwrap();
    let reset = hal.ad7().unwrap();

    log::info!("Configuring device...");
    let mut device = ice40::Fpga::new(spi, ss, done, reset, DummyDelay);
    device
        .configure(&bitstream[..])
        .expect("Failed to configure FPGA");
    log::info!("done!");
}
