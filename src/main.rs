use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::fs::File;
use std::io;

mod account;
mod los;
mod slot_machine;
use crate::account::*;
use crate::los::*;
use crate::slot_machine::*;

fn crossterm_example() -> Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    /*for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                // in this loop we are more efficient by not flushing the buffer.
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("█".magenta()))?;
            }
        }
    }*/

    stdout
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::PrintStyledContent("█".magenta()))?;
    Ok(())
}

fn main() {
    let _slot_machine = SlotMachine::new();

    // Display greeting and rules.
    display_title_greeting();
    display_rules();
    display_commands();

    // Check to see if there is a previous save file, if not, prompt user for name and create new account.
    let mut account = match File::open(SLOTS_ACCOUNT_FILE) {
        Ok(_) => load_account(),
        Err(_) => Account::new(&prompt_user_for_name().trim()),
    };

    match crossterm_example() {
        Ok(_) => println!(),
        Err(msg) => println!("crossterm::error: {}", msg),
    };

    println!("\nWelcome, {}", account.get_account_name());

    loop {
        println!("\n{:?}", account);
        if account.get_account_balance() == 0 {
            println!(
                "It looks like your balance is 0{}. You can edit {} to give yourself more money, but please note that account_balance is an unsigned 32 bit integer! Entering a value larger than an unsigned 32 bit integer can hold will cause unknown issues. I\'m too lazy to take five seconds to test it!",
                THORN_SYMBOL, SLOTS_ACCOUNT_FILE
            );
        }

        match prompt_user_for_command_input().trim() {
            "1" => {
                println!("You have bet 1{}", THORN_SYMBOL);
                let bet_value = 1;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "2" => {
                println!("You have bet 2{}", THORN_SYMBOL);
                let bet_value = 2;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "3" => {
                println!("You have bet 3{}", THORN_SYMBOL);
                let bet_value = 3;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "4" => {
                println!("You have bet 4{}", THORN_SYMBOL);
                let bet_value = 4;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "5" => {
                println!("You have bet 5{}", THORN_SYMBOL);
                let bet_value = 5;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "6" => {
                println!("You have bet 6{}", THORN_SYMBOL);
                let bet_value = 6;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "7" => {
                println!("You have bet 7{}", THORN_SYMBOL);
                let bet_value = 7;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "8" => {
                println!("You have bet 8{}", THORN_SYMBOL);
                let bet_value = 8;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "9" => {
                println!("You have bet 9{}", THORN_SYMBOL);
                let bet_value = 9;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "0" => {
                println!("Max bet! You have bet 10{}", THORN_SYMBOL);
                let bet_value = 10;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "rules" => display_rules(),
            "help" => display_commands(),
            "quit" => {
                println!("You have quit the game!");
                save_account(&account);
                break;
            }
            _ => println!("Invalid command. Try \'help\' for a list of valid commands."),
        }
    }
}
