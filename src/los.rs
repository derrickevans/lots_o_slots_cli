use std::io;

pub const THORN_SYMBOL: char = '\u{00FE}';

pub fn display_title_greeting() {
    println!("Welcome to Lots o\' Slots CLI!");
}

pub fn display_rules() {
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
    println!("Your last bet is saved and can be quick bet by pressing [Enter]");
}

pub fn display_commands() {
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

pub fn prompt_user_for_name() -> String {
    println!("\nEnter your name:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input for name.");
    input
}

pub fn prompt_user_for_command_input() -> String {
    println!("\nWhat would you like to do?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get command input.");
    input
}
