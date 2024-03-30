This example shows how software on an STM32H743 microcontroller can use the ice40 library to
configure an iCE40 FPGA using SPI.

The FPGA image `ice40_simple_configuration.bin` doesn't do anything very interesting, but the
FPGA accepts it.

# Hardware

Microcontroller board: [NUCLEO-H743ZI2](https://www.st.com/en/evaluation-tools/nucleo-h743zi.html)
* Default jumper configuration

FPGA board: iCE40UP5K-B-EVN
* Make these changes from the default jumper configuration:
  * Remove J7 (this deactivates the on-board flash)
  * Remove J28 (this leaves room to connect a wire for the CDONE signal)

Connections:
| Microcontroller pin | Microcontroller board connector | FPGA board connector | Description |
|---------------------|-----------------|-----------|-------------|
| PA6                 | CN7 pin 12      | J28 pin 2 | CDONE       |
| PA15                | CN7 pin 9       | J1 pin 9  | SS          |
| PB3                 | CN7 pin 15      | J1 pin 7  | SCK         |
| PB4                 | CN7 pin 19      | J1 pin 12 | MISO (labeled FLASH MOSI on the FPGA board) |
| PB5                 | CN7 pin 13      | J1 pin 5  | MOSI (labeled FLASH MISO on the FPGA board) |
| PB9                 | CN7 pin 4       | J11 pin 2 | CRESET      |
| ground              | CN7 pin 8       | J1 pin 10 | ground      |

# Steps to run

Connect the microcontroller board to a computer with a USB cable. Connect the FPGA board to
a computer, or any powered USB port, with a USB cable.

By default this project uses [probe-rs](https://probe.rs/) to flash the microcontroller and
display the output.
If you have probe-rs installed already, simply run `cargo run --release`.

The microcontroller should print "Successfully configured FPGA" to the terminal.

This firmware turns on a yellow LED on the microcontroller board while configuring the FPGA.
It turns on a green LED after the configuration succeeds or a red LED after it fails.