# blinky-rs
Bart Massey 2023

This Rust app for the BBC micro:bit v2 is a bare-minimum
blinky light program.

Alternate branches:

* `bare`: Blinky without the `microbit_v2` crate.
* `offboard`: Blinky for an LED connected to pin 8 on
  the MB2 edge connector.

## Build and Run

You can follow the instructions from the embedded micro:bit
[*Discovery Book*](https://docs.rust-embedded.org/discovery/microbit/index.html)
to set up your build environment.  Then you can say

    cargo embed --release

to flash and run this.

You can also follow the setup instructions in the `README`
on the `microbit` crate
[repo](https://github.com/nrf-rs/microbit). You can then say

    cargo run --release

## License

This work is made available under the "MIT License". Please
see the file `LICENSE.txt` in this distribution for license
terms.

## Acknowledgements

The hardware-using code and build infrastructure here were
heavily derived from the examples in the amazing `microbit`
crate.
