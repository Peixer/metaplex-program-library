//! Init pack instruction processing

use crate::{
    error::NFTPacksError,
    instruction::InitPackSetArgs,
    state::{InitPackSetParams, PackSet, MAX_DESCRIPTION_LEN, MAX_URI_LENGTH},
    utils::*,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{clock::Clock, rent::Rent, Sysvar}, msg,
};

/// Process InitPack instruction
pub fn init_pack(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InitPackSetArgs,
) -> ProgramResult {

    msg!("init_pack brother");
    let account_info_iter = &mut accounts.iter();
    let pack_set_account = next_account_info(account_info_iter)?;
    let authority_account = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(rent_info)?;
    let clock_info = next_account_info(account_info_iter)?;
    let clock = &Clock::from_account_info(clock_info)?;

    assert_rent_exempt(rent, pack_set_account)?;
    assert_signer(authority_account)?;

    let mut pack_set = PackSet::unpack_unchecked(&pack_set_account.data.borrow_mut())?;

    if pack_set.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if args.uri.len() > MAX_URI_LENGTH {
        return Err(NFTPacksError::UriTooLong.into());
    }

    if args.description.len() > MAX_DESCRIPTION_LEN {
        return Err(NFTPacksError::DescriptionTooLong.into());
    }

    if args.allowed_amount_to_redeem == 0 {
        return Err(NFTPacksError::WrongAllowedAmountToRedeem.into());
    }

    let current_timestamp = clock.unix_timestamp as u64;

    let redeem_start_date = args.redeem_start_date.unwrap_or(current_timestamp);

    if redeem_start_date < current_timestamp {
        return Err(NFTPacksError::WrongRedeemDate.into());
    }

    if let Some(redeem_end_date) = args.redeem_end_date {
        if redeem_end_date <= redeem_start_date {
            return Err(NFTPacksError::WrongRedeemDate.into());
        }
    }

    pack_set.init(InitPackSetParams {
        name: args.name,
        description: args.description,
        uri: args.uri,
        authority: *authority_account.key,
        mutable: args.mutable,
        distribution_type: args.distribution_type,
        allowed_amount_to_redeem: args.allowed_amount_to_redeem,
        redeem_start_date,
        redeem_end_date: args.redeem_end_date,
    });

    pack_set.puff_out_data_fields();


    PackSet::pack(pack_set, *pack_set_account.data.borrow_mut())?;

    Ok(())
}
