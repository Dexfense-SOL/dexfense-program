use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn deposit_amount(&self) -> u64 {
        match self {
            Difficulty::Easy => 1,
            Difficulty::Normal => 10,
            Difficulty::Hard => 100,
        }
    }
}
