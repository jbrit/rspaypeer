// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::generated::errors::PaypeerError;

#[derive(BorshSerialize, Debug)]
pub enum PaypeerInstruction {
/// Initialize a Pool account for this contract.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable]` swap_authority: [SwapAuthority] 
/// 2. `[writable]` pool: [Pool] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - ngn_mint: [Pubkey] 
/// - usd_mint: [Pubkey] 
	SetPool(SetPoolArgs),

/// Initialize Swap
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` pool: [Pool] 
/// 2. `[]` token_program: [AccountInfo] 
/// 3. `[]` user_authority: [AccountInfo] 
/// 4. `[writable]` user_source: [AccountInfo] 
/// 5. `[writable]` user_destination: [AccountInfo] 
/// 6. `[writable]` swap_authority: [SwapAuthority] 
/// 7. `[writable]` swap_source: [AccountInfo] 
/// 8. `[writable]` swap_destination: [AccountInfo] 
/// 9. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - amount_in: [u64] 
/// - min_amount_out: [u64] 
	Swap(SwapArgs),

}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SetPoolArgs {
	pub ngn_mint: Pubkey,
	pub usd_mint: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SwapArgs {
	pub amount_in: u64,
	pub min_amount_out: u64,
}

impl PaypeerInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(PaypeerError::InvalidInstruction)?;

        Ok(match variant {
			0 => Self::SetPool(SetPoolArgs::try_from_slice(rest).unwrap()),
			1 => Self::Swap(SwapArgs::try_from_slice(rest).unwrap()),
			_ => return Err(PaypeerError::InvalidInstruction.into())
        })
    }
}