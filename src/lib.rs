#![cfg_attr(not(feature = "std"), no_std)]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::SpiBus;

#[derive(Debug, Clone, Copy)]
pub enum Error<SPIERR, IOERR> {
    /// Error with SPI bus
    BusError(SPIERR),
    // Error with IO pins
    IoErr(IOERR),
    /// CDone was not asserted high after transfer.
    ConfigurationFailed,
}

pub struct Device<SPI, SS, DONE, RESET> {
    spi: SPI,
    ss: SS,
    done: DONE,
    reset: RESET,
}

impl<SPI, SS, DONE, RESET, SPIERR, IOERR> Device<SPI, SS, DONE, RESET>
where
    SPI: SpiBus<u8, Error = SPIERR>,
    SS: OutputPin<Error = IOERR>,
    DONE: InputPin<Error = IOERR>,
    RESET: OutputPin<Error = IOERR>,
{
    pub fn new(spi: SPI, ss: SS, done: DONE, reset: RESET) -> Self {
        Self {
            spi,
            ss,
            done,
            reset,
        }
    }

    fn set_reset_low(&mut self) -> Result<(), Error<SPIERR, IOERR>> {
        self.reset.set_low().map_err(|err| Error::IoErr(err))
    }

    fn set_reset_high(&mut self) -> Result<(), Error<SPIERR, IOERR>> {
        self.reset.set_high().map_err(|err| Error::IoErr(err))
    }

    fn set_ss_low(&mut self) -> Result<(), Error<SPIERR, IOERR>> {
        self.ss.set_low().map_err(|err| Error::IoErr(err))
    }

    fn set_ss_high(&mut self) -> Result<(), Error<SPIERR, IOERR>> {
        self.ss.set_high().map_err(|err| Error::IoErr(err))
    }

    fn is_done(&mut self) -> Result<bool, Error<SPIERR, IOERR>> {
        self.done.is_high().map_err(|err| Error::IoErr(err))
    }

    /// Configure the device throough slave SPI interface.
    /// See [iCE40 Programming and Configuration Technical Note](https://www.latticesemi.com/-/media/LatticeSemi/Documents/ApplicationNotes/IK/FPGA-TN-02001-3-3-iCE40-Programming-Configuration.ashx?document_id=46502)
    pub fn configure<DELAY: DelayNs>(
        &mut self,
        delay: &mut DELAY,
        bitstream: &[u8],
    ) -> Result<(), Error<SPIERR, IOERR>> {
        // Drive CRESET_B = 0
        log::debug!("Resetting device...");
        self.set_reset_low()?;
        // Drive SPI_SS_B = 0, SPI_SCK = 1
        self.set_ss_low()?;
        // Wait a minimum of 200ns
        delay.delay_us(1);
        // Release CRESET_B or drive CRESET_B = 1
        self.set_reset_high()?;
        // Wait a minimum of 1200µs to clear internal configuration memory
        delay.delay_us(1200);
        // Set SPI_SS_B = 1
        // Send 8 dummy clocks
        self.set_ss_high()?;
        self.spi.write(&[0u8]).map_err(|err| Error::BusError(err))?;
        self.spi.flush().map_err(Error::BusError)?;
        self.set_ss_low()?;
        // Send configuration image serially
        log::debug!("Writing bitestream...");
        for chunk in bitstream.chunks(1024) {
            self.spi.write(chunk).map_err(|err| Error::BusError(err))?;
        }
        // Wait for 100 (104) clock cycles for CDONE to go high
        self.spi
            .write(&[0u8; 13])
            .map_err(|err| Error::BusError(err))?;
        self.spi.flush().map_err(Error::BusError)?;
        // CDONE = 1?
        let cdone = self.is_done()?;
        log::debug!("CDONE = {}", cdone);
        if !cdone {
            // Error
            return Err(Error::ConfigurationFailed);
        }
        // Send a minimum of 49 additional dummy bits and 49 additional clock cycles to activate the user-I/O pins.
        self.spi
            .write(&[0u8; 7])
            .map_err(|err| Error::BusError(err))?;
        self.spi.flush().map_err(Error::BusError)?;
        // Done
        Ok(())
    }

    /// Breaks down this Device into its SPI bus and pins
    pub fn release(self) -> (SPI, SS, DONE, RESET) {
        (self.spi, self.ss, self.done, self.reset)
    }
}
