// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use std::str::FromStr;
use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info, next_account_infos};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::{msg, system_program};
use solana_program::sysvar::Sysvar;
use solana_program::program_pack::Pack;
use crate::generated::errors::PaypeerError;
use crate::generated::instructions::PaypeerInstruction;

use crate::generated::state::{
	Account,
	AccountPDA,
	SwapAuthority,
	Pool,
};
use crate::src::*;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let instruction = PaypeerInstruction::unpack(data)?;

        match instruction {
			PaypeerInstruction::SetPool(args) => {
				msg!("Instruction: SetPool");
				Self::process_set_pool(
					program_id,
					accounts, 
					args.ngn_mint,
					args.usd_mint,
				)
			}
			PaypeerInstruction::Swap(args) => {
				msg!("Instruction: Swap");
				Self::process_swap(
					program_id,
					accounts, 
					args.amount_in,
					args.min_amount_out,
				)
			}
        }
    }

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
	pub fn process_set_pool(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		ngn_mint: Pubkey,
		usd_mint: Pubkey,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let swap_authority_info = next_account_info(account_info_iter)?;
		let pool_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (swap_authority_pubkey, swap_authority_bump) = Pubkey::find_program_address(
			&[b"swapauthority"],
			program_id,
		);
		let (pool_pubkey, pool_bump) = Pubkey::find_program_address(
			&[b"pool"],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(PaypeerError::InvalidSignerPermission.into());
		}

		if *swap_authority_info.key != swap_authority_pubkey {
			return Err(PaypeerError::NotExpectedAddress.into());
		}

		if *pool_info.key != pool_pubkey {
			return Err(PaypeerError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PaypeerError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		if swap_authority_info.lamports() == 0 && *swap_authority_info.owner == system_program::id() {
			let space = SwapAuthority::LEN;
			let rent = Rent::get()?;
			let rent_minimum_balance = rent.minimum_balance(space);

			invoke_signed(
				&create_account(
					&fee_payer_info.key,
					&swap_authority_info.key,
					rent_minimum_balance,
					space as u64,
					program_id,
				),
				&[fee_payer_info.clone(), swap_authority_info.clone()],
				&[&[b"swapauthority", &[swap_authority_bump]]],
			)?;
		}

		if pool_info.lamports() == 0 && *pool_info.owner == system_program::id() {
			let space = Pool::LEN;
			let rent = Rent::get()?;
			let rent_minimum_balance = rent.minimum_balance(space);

			invoke_signed(
				&create_account(
					&fee_payer_info.key,
					&pool_info.key,
					rent_minimum_balance,
					space as u64,
					program_id,
				),
				&[fee_payer_info.clone(), pool_info.clone()],
				&[&[b"pool", &[pool_bump]]],
			)?;
		}


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PaypeerError::WrongAccountOwner.into());
		}

		if swap_authority_info.data_len() != SwapAuthority::LEN {
			return Err(PaypeerError::InvalidAccountLen.into());
		}

		if pool_info.data_len() != Pool::LEN {
			return Err(PaypeerError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let swap_authority = &mut AccountPDA::new(
			&swap_authority_info,
			try_from_slice_unchecked::<SwapAuthority>(&swap_authority_info.data.borrow()).unwrap(),
			swap_authority_bump,
		);
		let pool = &mut AccountPDA::new(
			&pool_info,
			try_from_slice_unchecked::<Pool>(&pool_info.data.borrow()).unwrap(),
			pool_bump,
		);

		// Calling STUB
		set_pool::set_pool(
			program_id,
			swap_authority,
			pool,
			ngn_mint,
			usd_mint,
		)?;

		// Accounts Serialization
		swap_authority.data.serialize(&mut &mut swap_authority_info.data.borrow_mut()[..])?;		pool.data.serialize(&mut &mut pool_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_swap(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		amount_in: u64,
		min_amount_out: u64,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let pool_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let user_authority_info = next_account_info(account_info_iter)?;
		let user_source_info = next_account_info(account_info_iter)?;
		let user_destination_info = next_account_info(account_info_iter)?;
		let swap_authority_info = next_account_info(account_info_iter)?;
		let swap_source_info = next_account_info(account_info_iter)?;
		let swap_destination_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (pool_pubkey, pool_bump) = Pubkey::find_program_address(
			&[b"pool"],
			program_id,
		);
		let (swap_authority_pubkey, swap_authority_bump) = Pubkey::find_program_address(
			&[b"swapauthority"],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(PaypeerError::InvalidSignerPermission.into());
		}

		if *pool_info.key != pool_pubkey {
			return Err(PaypeerError::NotExpectedAddress.into());
		}

		if *swap_authority_info.key != swap_authority_pubkey {
			return Err(PaypeerError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PaypeerError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		if swap_authority_info.lamports() == 0 && *swap_authority_info.owner == system_program::id() {
			let space = SwapAuthority::LEN;
			let rent = Rent::get()?;
			let rent_minimum_balance = rent.minimum_balance(space);

			invoke_signed(
				&create_account(
					&fee_payer_info.key,
					&swap_authority_info.key,
					rent_minimum_balance,
					space as u64,
					program_id,
				),
				&[fee_payer_info.clone(), swap_authority_info.clone()],
				&[&[b"swapauthority", &[swap_authority_bump]]],
			)?;
		}


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PaypeerError::WrongAccountOwner.into());
		}

		if pool_info.data_len() != Pool::LEN {
			return Err(PaypeerError::InvalidAccountLen.into());
		}

		if swap_authority_info.data_len() != SwapAuthority::LEN {
			return Err(PaypeerError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let pool = AccountPDA::new(
			&pool_info,
			try_from_slice_unchecked::<Pool>(&pool_info.data.borrow()).unwrap(),
			pool_bump,
		);
		let swap_authority = &mut AccountPDA::new(
			&swap_authority_info,
			try_from_slice_unchecked::<SwapAuthority>(&swap_authority_info.data.borrow()).unwrap(),
			swap_authority_bump,
		);

		// Calling STUB
		swap::swap(
			program_id,
			&pool,
			token_program_info,
			user_authority_info,
			user_source_info,
			user_destination_info,
			swap_authority,
			swap_source_info,
			swap_destination_info,
			amount_in,
			min_amount_out,
		)?;

		// Accounts Serialization
		swap_authority.data.serialize(&mut &mut swap_authority_info.data.borrow_mut()[..])?;
		
		Ok(())
	}
}