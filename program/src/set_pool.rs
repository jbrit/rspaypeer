use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	Pool,
	SwapAuthority,
};


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
pub fn set_pool(
	_program_id: &Pubkey,
	swap_authority: &mut AccountPDA<SwapAuthority>,
	pool: &mut AccountPDA<Pool>,
	ngn_mint: Pubkey,
	usd_mint: Pubkey,
) -> ProgramResult {
	pool.data.ngn = ngn_mint;
	pool.data.usd = usd_mint;
    Ok(())
}