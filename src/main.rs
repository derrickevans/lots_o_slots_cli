use crossterm::{cursor::MoveTo, terminal, ExecutableCommand, QueueableCommand, Result};
use std::fs::File;
use std::io;

mod account;
mod los;
mod slot_machine;
use crate::account::*;
use crate::los::*;
use crate::slot_machine::*;

fn clear_screen(stdout: &mut io::Stdout) -> Result<()> {
    stdout.queue(MoveTo(0, 0))?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

fn main() {
    let _slot_machine = SlotMachine::new();
    let mut stdout = io::stdout();

    display_title_greeting();

    // Check to see if there is a previous save file, if not, prompt user for name and create new account.
    let mut account = match File::open(SLOTS_ACCOUNT_FILE) {
        Ok(_) => load_account(),
        Err(_) => Account::new(&prompt_user_for_name().trim()),
    };

    clear_screen(&mut stdout).expect("Oops. Cannot clear the screen.");
    println!("\nWelcome, {}", account.get_account_name());

    // TODO: Press key to continue...

    loop {
        //        clear_screen(&mut stdout).expect("Oops. Cannot clear the screen.");

        println!("\n{:?}", account);
        if account.get_account_balance() == 0 {
            println!(
                "It looks like your balance is 0{}. You can edit {} to give yourself more money, but please note that account_balance is an unsigned 32 bit integer! Entering a value larger than an unsigned 32 bit integer can hold will cause unknown issues. I\'m too lazy to take five seconds to test it!",
                THORN_SYMBOL, SLOTS_ACCOUNT_FILE
            );
        }

        match prompt_user_for_command_input().trim() {
            "" => {
                clear_screen(&mut stdout).expect("Oops. Cannot clear the screen.");
                println!("You have bet 1{}", THORN_SYMBOL);
                let bet_value = 1;
                if sufficient_funds(&account, bet_value) {
                    update_account(&mut account, play_round(bet_value));
                } else {
                    println!("Insufficient funds! Please enter a new command.");
                    continue;
                }
            }
            "rules" => {
                clear_screen(&mut stdout).expect("Oops. Cannot clear the screen.");
                display_rules();
            }
            "help" => {
                clear_screen(&mut stdout).expect("Oops. Cannot clear the screen.");
                display_commands();
            }
            "quit" => {
                clear_screen(&mut stdout).expect("Oops. Cannot clear the screen.");
                println!("You have quit the game!");
                save_account(&account);
                break;
            }
            _ => println!("Invalid command. Try \'help\' for a list of valid commands."),
        }
    }
}
