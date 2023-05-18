#![cfg_attr(not(feature = "std"), no_std)]

use embedded_hal::{
    blocking::{delay, spi},
    digital,
    prelude::*,
};

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
    SPI: spi::Write<u8, Error = SPIERR>,
    SS: digital::v2::OutputPin<Error = IOERR>,
    DONE: digital::v2::InputPin<Error = IOERR>,
    RESET: digital::v2::OutputPin<Error = IOERR>,
{
    pub fn new(spi: SPI, ss: SS, done: DONE, reset: RESET) -> Self {
        Self {
            spi,
            ss,
            done,
            reset,
        }
    }

    pub fn set_reset_low(&mut self) -> Result<(), Error<SPIERR, IOERR>> {
        self.reset.set_low().map_err(|err| Error::IoErr(err))
    }

    pub fn set_reset_high(&mut self) -> Result<(), Error<SPIERR, IOERR>> {
        self.reset.set_high().map_err(|err| Error::IoErr(err))
    }

    pub fn set_ss_low(&mut self) -> Result<(), Error<SPIERR, IOERR>> {
        self.ss.set_low().map_err(|err| Error::IoErr(err))
    }

    pub fn set_ss_high(&mut self) -> Result<(), Error<SPIERR, IOERR>> {
        self.ss.set_high().map_err(|err| Error::IoErr(err))
    }

    pub fn is_done(&mut self) -> Result<bool, Error<SPIERR, IOERR>> {
        self.done.is_high().map_err(|err| Error::IoErr(err))
    }

    /// Configure the device throough slave SPI interface.
    /// See [iCE40 Programming and Configuration Technical Note](https://www.latticesemi.com/-/media/LatticeSemi/Documents/ApplicationNotes/IK/FPGA-TN-02001-3-3-iCE40-Programming-Configuration.ashx?document_id=46502)
    pub fn configure<DELAY: delay::DelayUs<u16>>(
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
        self.set_ss_low()?;
        // Send configuration image serially
        log::debug!("Writing bitestream...");
        for chunk in bitstream.chunks(1024) {
            self.write(chunk).map_err(|err| Error::BusError(err))?;
        }
        // Wait for 100 (104) clock cycles for CDONE to go high
        self.write(&[0u8; 13]).map_err(|err| Error::BusError(err))?;
        // CDONE = 1?
        let cdone = self.is_done()?;
        log::debug!("CDONE = {}", cdone);
        if !cdone {
            // Error
            return Err(Error::ConfigurationFailed);
        }
        // Send a minimum of 49 additional dummy bits and 49 additional clock cycles to activate the user-I/O pins.
        self.write(&[0u8; 7]).map_err(|err| Error::BusError(err))?;
        // Done
        Ok(())
    }
}

impl<SPI, SS, DONE, RESET, SPIERR> spi::Write<u8> for Device<SPI, SS, DONE, RESET>
where
    SPI: spi::Write<u8, Error = SPIERR>,
{
    type Error = SPIERR;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(words)
    }
}

impl<SPI, SS, DONE, RESET, SPIERR> spi::Transfer<u8> for Device<SPI, SS, DONE, RESET>
where
    SPI: spi::Transfer<u8, Error = SPIERR>,
{
    type Error = SPIERR;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.spi.transfer(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
