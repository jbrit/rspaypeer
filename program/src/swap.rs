use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::{account_info::AccountInfo, program::invoke};
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	Pool,
	SwapAuthority,
};


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
pub fn swap<'info>(
	_program_id: &Pubkey,
	_pool: &AccountPDA<Pool>,
	token_program: &AccountInfo<'info>,
	user_authority: &AccountInfo<'info>,
	user_source: &AccountInfo<'info>,
	user_destination: &AccountInfo<'info>,
	swap_authority: &mut AccountPDA<'_, 'info, SwapAuthority>,
	swap_source: &AccountInfo<'info>,
	swap_destination: &AccountInfo<'info>,
	amount_in: u64,
	min_amount_out: u64,
) -> ProgramResult {
    let transfer_in_inst = spl_token::instruction::transfer(
		token_program.key,
		user_source.key,
		swap_destination.key,
		user_authority.key,
		&[user_authority.key],
		amount_in
	)?;
	let transfer_in_accounts = [
		user_source.clone(),
		swap_destination.clone(),
		user_authority.clone(),
	];
	invoke(&transfer_in_inst, &transfer_in_accounts)?;
	
	let amount_out = 0;
	// TODO: implement actual amount out / swap logic

	if amount_out < min_amount_out{
		panic!("amount out less than expected")
	}
	let transfer_out_inst = spl_token::instruction::transfer(
		token_program.key,
		swap_source.key,
		user_destination.key,
		swap_authority.info.key,
		&[],
		amount_out
	)?;

	let transfer_out_accounts = [
		swap_source.clone(),
		user_destination.clone(),
		swap_authority.info.clone(),
	];
	invoke_signed(&transfer_out_inst, &transfer_out_accounts,&[&[b"swapauthority",&[swap_authority.bump]]] )?;
	Ok(())
}