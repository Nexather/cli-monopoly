use std::fs;
use std::io::BufRead;

use rand::Rng;

mod tiles;
mod community_cards;

struct PlayerState {
    p_money: i32,
    p_position: i32,
    p_properties: Vec<i32>,
    p_jailed: bool,
}

fn roll() -> i32 {
    return rand::thread_rng().gen_range(2..=12);
}

fn turn(player: &PlayerState)
{
    let prompt = String::from("Your turn! What will you do?\n1. Roll the Dice!    2. Build Houses/Hotels   3. Sell Houses/Hotels   4. Mortgage Properties");
    let jail_prompt = String::from("You're in jail! What will you do? \n1. Roll for Doubles   2. Build Houses/Hotels   3. Sell Houses/Hotels   4. Mortgage Properties   5. Pay $50 Fine");
    let mut choice_string:String = String::new();
    let mut choice:i32 = 0;

    if player.p_jailed {
        println!("{}", jail_prompt);
    } else {
        println!("{}", prompt);
    }

    std::io::stdin().lock().read_line(&mut choice_string).unwrap();
    choice = choice_string.trim().parse().expect("Input not an integer");
    //println!("{}", choice);
}

fn main() {
    let player = PlayerState {
        p_money: 1500,
        p_position: 0,
        p_properties: Vec::new(),
        p_jailed: false,
    };

    let board: &str = include_str!("board.txt");
    println!("Welcome to CLI Monopoly!");

    loop { //game running
        for line in board.lines() {
            println!("{}", line);
        }
        turn(&player);

    }
}
