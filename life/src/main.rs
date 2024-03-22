#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, timer::Timer},
};
use nanorand::{pcg64::Pcg64, Rng};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

mod life;
use life::*;

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut life_timer = Timer::new(board.TIMER1);
    let mut game_board: [[u8; 5]; 5];
    let mut rng_rng = Pcg64::new_seed(1);
    loop {
        game_board = [
            [0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
        ];
        loop {
            let mut rng = Pcg64::new_seed(rng_rng.generate());

            display.show(&mut timer, game_board, 1000);
            // Update the board based on Game of Life rules
            life(&mut game_board);

            // Display the current board
            timer.delay_ms(100u32);
            // Check button A status
            if board.buttons.button_a.is_low().unwrap() {
                // Re-randomize the board
                game_board = generate_board(&mut rng);
            }

            // Check button B status
            if board.buttons.button_b.is_low().unwrap() {
                // Complement the board
                complement_board(&mut game_board);
                // Ignore button B for 5 frames
                life_timer.delay_ms(500u32);
            }

            // Check if all cells are off
            if done(&game_board) {
                // rprintln!("Board is empty!");
                // Wait for 5 frames before generating a new random board
                life_timer.delay_ms(500u32);
                // Generate a random board
                game_board = generate_board(&mut rng);
            }
        }
    }
}

// Function to generate a random board
fn generate_board(rng: &mut Pcg64) -> [[u8; 5]; 5] {
    let mut board = [[0; 5]; 5];
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            let b: bool = rng.generate();
            *cell = if b { 1 } else { 0 };
        }
    }
    board
}

// Function to complement the board
fn complement_board(board: &mut [[u8; 5]; 5]) {
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            *cell = if *cell == 0 { 1 } else { 0 };
        }
    }
}
