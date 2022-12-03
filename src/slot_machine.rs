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

pub enum WinningType {
    Jackpot,
    Big,
    Medium,
    Small,
    Loss,
}

// Generate random number to be the index into the reel array containing the values.
pub fn reel_stop_value() -> u32 {
    let reel = [
        0, 8, 5, 2, 6, 9, 0, 1, 4, 3, 1, 2, 5, 9, 1, 4, 3, 0, 7, 6, 2, 5, 3, 4, 1, 8, 3, 6, 0, 2,
    ];

    // Get random number for the index
    let mut rng = thread_rng();
    let index = rng.gen_range(0..30);
    reel[index]
}
