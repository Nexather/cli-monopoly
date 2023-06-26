use std::fs;
use std::io::BufRead;

use rand::Rng;

mod tiles;
mod community_cards;

struct PlayerState {
    p_money: u8,
    p_position: u8,
    p_properties: Vec<u8>,
    p_jailed: bool,
}

fn roll() -> i32 {
    return rand::thread_rng().gen_range(2..=12);
}

fn main() {
    let player = PlayerState::new(1500, 0, Vec::new(), false);
    let board = fs::read_to_string("src/board.txt").expect("Could not read board file");
    let prompt = String::from("Your turn! What will you do?\n. Roll the Dice!    2. Build Houses/Hotels   3. Sell Houses/Hotels   4. Mortgage Properties");
    let jail_prompt = String::from("You're in jail! What will you do? \n1. Roll for Doubles   2. Pay $50 Fine   3. Build Houses/Hotels   4. Sell Houses/Hotels   5. Mortgage Properties");

    println!("Welcome to CLI Monopoly!");
    while () { //game running
        for line in board.lines() {
            println!("{}", line);
        }
        if player.p_jailed {
            println!(jail_prompt);
        } else {
            println!(prompt);
        }
    }
}
