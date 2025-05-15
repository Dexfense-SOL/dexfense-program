use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid token mint address")]
    InvalidMint,

    #[msg("Game is still active")]
    GameStillActive,

    #[msg("Session already verified")]
    AlreadyVerified,

    #[msg("Game session is already closed")]
    AlreadyClosed, // 기존 에러 유지

    #[msg("Invalid mint between vault and pool")]
    InvalidPair,

    #[msg("Game already settled")]
    AlreadySettled,

    #[msg("Game session has not been verified")]
    Unverified,
}
