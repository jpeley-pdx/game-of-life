//! Embedded Rust Programming - Prog 1
//! Game of Life
//!
//! https://canvas.pdx.edu/courses/101554/assignments/1055437
//!
// You will write a program that interactively plays Game of Life on your MB2.
// Specs:
// The program runs the game at 10 frames per second (updates once per 100ms).
// The program starts with a random board.
// While the A button is held, the board is re-randomized every frame.
// Otherwise, when the B button is not ignored and is pressed, the board is
// "complemented": every "on" cell is turned "off" and every "off" cell is
// turned "on". The B button is then ignored for 5 frames (0.5s).
//
// Otherwise, if the program reaches a state where all cells on the board
// are off, the program waits 5 frames (0.5s). If it has not received a
// button press, it then starts with a new random board.
// Otherwise, normal Life steps are taken.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embedded_hal::{delay::DelayNs, digital::InputPin, digital::OutputPin};
use microbit::{
    board::Board,
    gpio::DisplayPins,
    hal::rng::Rng,
    hal::timer::Timer,
    hal::uarte::{Baudrate, Parity, Uarte},
};

/// This is a function to turn the LED on or off for the current cell.
fn set_led(state: bool, x: usize, y: usize, display_pins: &mut DisplayPins) {
    match (state, y) {
        (true, 0) => {
            display_pins.row1.set_high().unwrap();
        }
        (true, 1) => {
            display_pins.row2.set_high().unwrap();
        }
        (true, 2) => {
            display_pins.row3.set_high().unwrap();
        }
        (true, 3) => {
            display_pins.row4.set_high().unwrap();
        }
        (true, 4) => {
            display_pins.row5.set_high().unwrap();
        }

        (false, 0) => {
            display_pins.row1.set_low().unwrap();
        }
        (false, 1) => {
            display_pins.row2.set_low().unwrap();
        }
        (false, 2) => {
            display_pins.row3.set_low().unwrap();
        }
        (false, 3) => {
            display_pins.row4.set_low().unwrap();
        }
        (false, 4) => {
            display_pins.row5.set_low().unwrap();
        }

        (_, _) => {}
    }

    match (state, x) {
        (true, 0) => {
            display_pins.col1.set_low().unwrap();
        }
        (true, 1) => {
            display_pins.col2.set_low().unwrap();
        }
        (true, 2) => {
            display_pins.col3.set_low().unwrap();
        }
        (true, 3) => {
            display_pins.col4.set_low().unwrap();
        }
        (true, 4) => {
            display_pins.col5.set_low().unwrap();
        }

        (false, 0) => {
            display_pins.col1.set_high().unwrap();
        }
        (false, 1) => {
            display_pins.col2.set_high().unwrap();
        }
        (false, 2) => {
            display_pins.col3.set_high().unwrap();
        }
        (false, 3) => {
            display_pins.col4.set_high().unwrap();
        }
        (false, 4) => {
            display_pins.col5.set_high().unwrap();
        }

        (_, _) => {}
    }
}

/// Function to write a byte to the serial port
fn serial_write<T>(serial: &mut Uarte<T>, buffer: &[u8])
where
    T: microbit::hal::uarte::Instance,
{
    for b in buffer {
        match serial.write(&[*b]) {
            Ok(_r) => (),
            Err(e) => rprintln!("Serial Error: {:?}", e),
        }
    }
}

// Bart's Code:
//
// Conway's Game of Life implemented on a 5×5 "frame
// buffer" of `u8` pixels that can be either 0 or 1.

// Return `true` iff the frame buffer contains no 1
// pixels.
pub fn done(fb: &[[u8; 5]; 5]) -> bool {
    fb == &[[0u8; 5]; 5]
}

/// Make a step according to the Game of Life rules.
pub fn life(fb: &mut [[u8; 5]; 5]) {
    let prev = *fb;
    for row in 0..5 {
        for col in 0..5 {
            let prev_row = (row + 4) % 5;
            let next_row = (row + 1) % 5;
            let prev_col = (col + 4) % 5;
            let next_col = (col + 1) % 5;
            let coords = [
                (prev_row, prev_col),
                (prev_row, col),
                (prev_row, next_col),
                (row, prev_col),
                (row, next_col),
                (next_row, prev_col),
                (next_row, col),
                (next_row, next_col),
            ];
            let neighbors = coords.into_iter().map(|(r, c)| prev[r][c]).sum();
            #[allow(clippy::manual_range_contains)]
            match (prev[row][col], neighbors) {
                (1, n) if n < 2 || n > 3 => fb[row][col] = 0,
                (0, 3) => fb[row][col] = 1,
                (_, _) => (),
            }
        }
    }
}

#[entry]
fn init() -> ! {
    // Init rtt Print
    rtt_init_print!();

    // Acquire the board, timer, and Rng functions
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut rng = Rng::new(board.RNG);

    // Set up UARTE for microbit v2 using UartePort wrapper
    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    // setup the button inputs
    let mut button_a = board.buttons.button_a.into_floating_input();
    let mut button_b = board.buttons.button_b.into_floating_input();

    // send an ASCII command to clear the terminal
    serial_write(&mut serial, b"\x1Bc");

    // Init the world storage
    let mut world: [[u8; 5]; 5] = [[0; 5]; 5];
    let mut last_world: [[u8; 5]; 5] = [[0; 5]; 5];

    // Randomly populate the world array
    // for i in 0..5 {
    //     for j in 0..5 {
    //         world[i][j] = rng.random_u8() % 2;
    //     }
    // }
    for row in &mut world {
        for mut _col in row {
            _col = &mut (rng.random_u8() % 2);
        }
    }

    //init variables
    let mut tick_count = 0;
    let mut update_serial = true;
    let mut a_ignore = false;
    let mut b_ignore = false;
    let mut b_ignore_count = 0;
    let mut same_board_count = 0;

    // disabling the clippy warning because it's suggested
    // method breaks the program.
    #[allow(clippy::needless_range_loop)]
    // Main loop
    loop {
        // tick advance
        // used to determine if it's time tp update the world
        tick_count += 1;

        // Update the world every 10 ticks
        if tick_count > 10 {
            // life moves forward...
            life(&mut world);

            // clear the terminal
            serial_write(&mut serial, b"\x1Bc");

            // indicate that the serial display needs updating
            update_serial = true;

            // check the board has remained static
            // increment the same would count if so
            if last_world == world {
                same_board_count += 1
            } else {
                last_world = world;
            }

            // if the world has remained static for 1000 ticks,
            // randomly populate the world with new cells
            if same_board_count > 50 {
                for i in 0..5 {
                    for j in 0..5 {
                        world[i][j] = rng.random_u8() % 2;
                    }
                }
            }

            //reset the tick counter
            tick_count = 0;
        }

        // if button A is pressed..
        if button_a.is_low().unwrap() {
            timer.delay_ms(20); // debounce time

            // randomly populate the world with new cells
            for i in 0..5 {
                for j in 0..5 {
                    world[i][j] = rng.random_u8() % 2;
                }
            }

            // set the ignore flag for button A
            a_ignore = true;

            // indicate that the serial display needs updating
            update_serial = true;

            // clear the terminal
            serial_write(&mut serial, b"\x1Bc");
        }

        // if a_ignore flag is set and the A button is not pressed
        // reset the a ignore flag
        if a_ignore & button_a.is_high().unwrap() {
            a_ignore = false;
        }

        // if the A ignore flag is not active
        // and the B ignore flag is not set
        // and Button B is low
        if !a_ignore & !b_ignore & button_b.is_low().unwrap() {
            timer.delay_ms(20); // debounce time

            // invert all the cells in the world
            for i in 0..5 {
                for j in 0..5 {
                    if world[i][j] == 1 {
                        world[i][j] = 0;
                    } else {
                        world[i][j] = 1;
                    }
                }
            }

            // set the B Ignore flag
            b_ignore = true;

            // indicate that the serial display needs updating
            update_serial = true;

            // clear the terminal
            serial_write(&mut serial, b"\x1Bc");
        }

        // if the B ignore flag is set
        // increment the b ignore count
        // this ensures that the b button is ignored
        // for a set period of time
        if b_ignore & (b_ignore_count < 100) {
            b_ignore_count += 1;
        } else {
            // reset the b ignore count
            b_ignore_count = 0;

            //reset the b ignore flag
            b_ignore = false;
        }

        // update the led display and the serial display with the
        // current status of the world
        for i in 0..5 {
            for j in 0..5 {
                if world[i][j] == 1 {
                    // Cell is alive

                    // Blink the LED
                    // the loop is so rapid it appears that the LED stays on
                    set_led(true, i, j, &mut board.display_pins);
                    timer.delay_ms(1);
                    set_led(false, i, j, &mut board.display_pins);

                    // Write an X to indicate a live cell on the serial display
                    if update_serial {
                        serial_write(&mut serial, b"X");
                    }
                } else {
                    // Cell is dead

                    //  turn off the led
                    set_led(false, i, j, &mut board.display_pins);

                    // Write . to indicate a dead cell on the serial display
                    if update_serial {
                        serial_write(&mut serial, b".");
                    }
                }
            }

            // Write new line and carriage return
            // to end the current serial line
            if update_serial {
                serial_write(&mut serial, b"\n\r");
            }
        }

        // reset the update serial flag
        update_serial = false;
    }
}
