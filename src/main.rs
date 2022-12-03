use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{stdout, Write};

mod slot_machine;
use crate::slot_machine::{reel_stop_value, SlotMachine, WinningType};

const THORN_SYMBOL: char = '\u{00FE}';
const SLOTS_ACCOUNT_FILE: &str = "slots_data.toml";

fn crossterm_example() -> Result<()> {
    let mut stdout = stdout();
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

fn display_title_greeting() {
    println!("Welcome to Lots o\' Slots CLI!");
}

fn display_rules() {
    println!("\n--- Rules ---");
    println!(
        "The currency in Lots o\' Slots CLI is the Thorn ({}).",
        THORN_SYMBOL
    );
    println!("The player starts with 100{}.", THORN_SYMBOL);
    println!(
        "To place a bet enter 0-9. 0 being a max bet of 10{}",
        THORN_SYMBOL
    );
}

fn display_commands() {
    println!("\n--- Commands ---");
    println!("1 \t-> Bet of 1{}", THORN_SYMBOL);
    println!("2 \t-> Bet of 2{}", THORN_SYMBOL);
    println!("3 \t-> Bet of 3{}", THORN_SYMBOL);
    println!("4 \t-> Bet of 4{}", THORN_SYMBOL);
    println!("5 \t-> Bet of 5{}", THORN_SYMBOL);
    println!("6 \t-> Bet of 6{}", THORN_SYMBOL);
    println!("7 \t-> Bet of 7{}", THORN_SYMBOL);
    println!("8 \t-> Bet of 8{}", THORN_SYMBOL);
    println!("9 \t-> Bet of 9{}", THORN_SYMBOL);
    println!("0 \t-> Max bet of 10{}", THORN_SYMBOL);
    println!("help \t-> Displays the commands.");
    println!("rules \t-> Displays the rules.");
    println!("quit \t-> Saves and closes the application.\n");
}

fn prompt_user_for_name() -> String {
    println!("Enter your name:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input for name.");
    input
}

fn prompt_user_for_command_input() -> String {
    println!("What would you like to do?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get command input.");
    input
}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    account_name: String,
    account_balance: u32,
}

impl Account {
    fn new(name: &str) -> Self {
        Self {
            account_name: String::from(name),
            account_balance: 100,
        }
    }
}

fn update_account(account: &mut Account, game_result: (bool, u32)) {
    if game_result.0 {
        account.account_balance += game_result.1;
    } else {
        account.account_balance -= game_result.1;
    }
}

fn save_account(account: &Account) {
    // Serialize and write to file.
    let serialized = toml::to_string(&account).unwrap();
    let mut account_file = File::create(SLOTS_ACCOUNT_FILE).expect("Failed to create file.");
    account_file
        .write_all(serialized.as_bytes())
        .expect("Failed to write all the bytes to the file.");
    println!("You progess has been saved.");
}

fn load_account() -> Account {
    // Read from file and deserialize.
    let mut account_file = File::open(SLOTS_ACCOUNT_FILE).expect("Failed to open the file.");
    let mut data = String::new();
    let _serialized_data = account_file
        .read_to_string(&mut data)
        .expect("Failed to get data");
    let account: Account = toml::from_str(data.as_str()).expect("Failed here too.");
    account
}

fn sufficient_funds(account: &Account, value: u8) -> bool {
    if account.account_balance < value as u32 {
        false
    } else {
        true
    }
}

fn play_round(wager_value: u8) -> (bool, u32) {
    let reel_1_stop_value = reel_stop_value();
    let reel_2_stop_value = reel_stop_value();
    let reel_3_stop_vlue = reel_stop_value();

    println!(
        "{} | {} | {}",
        reel_1_stop_value, reel_2_stop_value, reel_3_stop_vlue
    );

    let mut winning_type: WinningType = WinningType::Loss;
    if reel_1_stop_value == 7 {
        winning_type = WinningType::Jackpot;
    } else if reel_1_stop_value == 8 || reel_1_stop_value == 9 {
        winning_type = WinningType::Big;
    } else if reel_1_stop_value == 4 || reel_1_stop_value == 5 || reel_1_stop_value == 6 {
        winning_type = WinningType::Medium;
    } else if reel_1_stop_value == 0
        || reel_1_stop_value == 1
        || reel_1_stop_value == 2
        || reel_1_stop_value == 3
    {
        winning_type = WinningType::Small;
    }

    let mut winner: bool = false;
    if reel_1_stop_value == reel_2_stop_value && reel_2_stop_value == reel_3_stop_vlue {
        match winning_type {
            WinningType::Jackpot => winner = true,
            WinningType::Big => winner = true,
            WinningType::Medium => winner = true,
            WinningType::Small => winner = true,
            WinningType::Loss => winner = false,
        }
    }

    (winner, wager_value as u32)
}

fn main() {
    let mut slot_machine = SlotMachine::new();

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

    println!("\nWelcome, {}", account.account_name);

    loop {
        println!("\n{:?}", account);
        if account.account_balance == 0 {
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
