use rand::prelude::*;

#[derive(Debug)]
pub struct SlotMachine {
    pub id: u32,
    jackpot: u32,
}

impl SlotMachine {
    pub fn new() -> Self {
        Self {
            id: generate_uuid(),
            jackpot: 10000,
        }
    }

    pub fn increment_jackpot(&mut self, amount: u32) {
        self.jackpot += amount;
    }

    pub fn decrement_jackpot(&mut self, amount: u32) {
        self.jackpot -= amount;
    }
}

// TODO: Generate a real UUID
fn generate_uuid() -> u32 {
    1000
}

enum WinningType {
    Jackpot,
    Big,
    Medium,
    Small,
    Loss,
}

// Generate random number to be the index into the reel array containing the values.
fn reel_stop_value() -> u32 {
    let reel = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 7,
    ];

    // Get random number for the index
    let mut rng = thread_rng();
    let index = rng.gen_range(0..30);
    reel[index]
}

pub fn play_round(wager_value: u8) -> (bool, u32) {
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
