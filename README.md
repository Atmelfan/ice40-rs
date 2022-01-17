[![CI](https://github.com/Atmelfan/ice40-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Atmelfan/ice40-rs/actions/workflows/ci.yml)

# ice40-rs
This is an [embedded-hal] driver for configuration of [iCE40 series FPGAs](https://www.latticesemi.com/Products.aspx) from [Lattice](https://www.latticesemi.com/).

See technical note [TN1248 - iCE40 Programming and Configuration](https://www.latticesemi.com/~/media/LatticeSemi/Documents/ApplicationNotes/IK/iCE40ProgrammingandConfiguration.pdf) on how the slave configuration interfaceworks.


# Quickstart
```toml
[dependencies]
ice40-rs = { git = "https://github.com/Atmelfan/ice40-rs.git" }
```

# Example
The examples folder contains a utility for configuring a device using linux and ftdi embedded-hal.

You can use them with the following commands:

## FTDI
Assumes the FTDI circuit is connected like the ice40-breakout board.

`cargo run --example ftdi -- my_image.bin`
```
ice40-rs/ftdi 0.1.0
FTDI demo

USAGE:
    ftdi [OPTIONS] <binary>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --frequency <frequency>    Bus frequency [default: 3000000]

ARGS:
    <binary>    Binary file
```

## Linux
Default arguments are for a Raspberry pi Model 4.

`cargo run --example linux -- my_image.bin`
```
ice40-rs/linux 0.1.0
Linux demo

USAGE:
    linux [OPTIONS] <binary>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --cdone <cdone>            CDONE pin [default: 24]
        --creset <creset>          CRESET pin [default: 25]
    -f, --frequency <frequency>    Bus frequency [default: 3000000]
        --spi <spi>                SPI bus [default: /dev/spidev0.0]
        --ss <ss>                  SS pin [default: 8]

ARGS:
    <binary>    Binary file
```

# Limitations
The library only support volatile configuration of the device, not external nonvolatile memory.

[embedded-hal]: https://github.com/rust-embedded/embedded-hal
