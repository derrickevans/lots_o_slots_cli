use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

pub const SLOTS_ACCOUNT_FILE: &str = "slots_data.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    account_name: String,
    account_balance: u32,
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            account_name: String::from(name),
            account_balance: 100,
        }
    }

    // TODO: Why does this work?
    // Dangling Reference??? I don't think so.
    // I think this is fine as long as self is in scope.
    pub fn get_account_name(&self) -> &String {
        &self.account_name
    }

    pub fn get_account_balance(&self) -> u32 {
        self.account_balance
    }
}

pub fn update_account(account: &mut Account, game_result: (bool, u32)) {
    if game_result.0 {
        account.account_balance += game_result.1;
    } else {
        account.account_balance -= game_result.1;
    }
}

pub fn save_account(account: &Account) {
    // Serialize and write to file.
    let serialized = toml::to_string(&account).unwrap();
    let mut account_file = File::create(SLOTS_ACCOUNT_FILE).expect("Failed to create file.");
    account_file
        .write_all(serialized.as_bytes())
        .expect("Failed to write all the bytes to the file.");
    println!("You progess has been saved.");
}

pub fn load_account() -> Account {
    // Read from file and deserialize.
    let mut account_file = File::open(SLOTS_ACCOUNT_FILE).expect("Failed to open the file.");
    let mut data = String::new();
    let _serialized_data = account_file
        .read_to_string(&mut data)
        .expect("Failed to get data");
    let account: Account = toml::from_str(data.as_str()).expect("Failed here too.");
    account
}

pub fn sufficient_funds(account: &Account, value: u8) -> bool {
    if account.account_balance < value as u32 {
        false
    } else {
        true
    }
}
