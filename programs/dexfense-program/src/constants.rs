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

pub const BACKEND_SIGNER: Pubkey = Pubkey::new_from_array([
    56,  190, 213, 115,  48,  151, 244,  93,
    25,  204,  11,  76, 113, 183,  60,  84,
    193, 218, 202,  93, 136, 173, 109, 237,
    76,  56, 134, 191, 208, 215,  48, 211,
]);

// This is a placeholder for the DFP token mint address.
// In a real-world scenario, you would replace this with the actual mint address of the DFP token.
pub const DFP_TOKEN_MINT: Pubkey = Pubkey::new_from_array([
    1,  2,  3,  4,  5,  6,  7,  8,
    9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32
]);