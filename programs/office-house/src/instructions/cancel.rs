use anchor_lang::prelude::*;
use spl_token::instruction::{revoke};
use anchor_lang::solana_program::program::invoke;
use crate::constants::seeds::{FEE_PAYER, PREFIX};
use crate::errorcodes::errors::Errors;
use crate::office_house_structs::cancel::Cancel;
use crate::utils::assert::{assert_keys_equal};
use crate::utils::get_fee_payer::get_fee_payer;

pub fn cancel<'info>(
    ctx: Context<'_, '_, '_, 'info, Cancel<'info>>,
    _buyer_price: u64,
    _token_size: u64,
) -> ProgramResult {
    let wallet = &ctx.accounts.wallet;
    let token_account = &ctx.accounts.token_account;
    let token_mint = &ctx.accounts.token_mint;
    let authority = &ctx.accounts.authority;
    let auction_house = &ctx.accounts.auction_house;
    let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
    let trade_state = &ctx.accounts.trade_state;
    let token_program = &ctx.accounts.token_program;

    assert_keys_equal(token_mint.key(), token_account.mint)?;

    if !wallet.to_account_info().is_signer && !authority.to_account_info().is_signer {
        return Err(Errors::NoValidSignerPresent.into());
    }

    let auction_house_key = auction_house.key();
    let seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        FEE_PAYER.as_bytes(),
        &[auction_house.fee_payer_bump],
    ];

    let (fee_payer, _) = get_fee_payer(
        authority,
        auction_house,
        wallet.to_account_info(),
        auction_house_fee_account.to_account_info(),
        &seeds,
    )?;

    let curr_lamp = trade_state.lamports();
    **trade_state.lamports.borrow_mut() = 0;

    **fee_payer.lamports.borrow_mut() = fee_payer
        .lamports()
        .checked_add(curr_lamp)
        .ok_or(Errors::NumericalOverflow)?;

    if token_account.owner == wallet.key() && wallet.is_signer {
        invoke(
            &revoke(
                &token_program.key(),
                &token_account.key(),
                &wallet.key(),
                &[],
            )
                .unwrap(),
            &[
                token_program.to_account_info(),
                token_account.to_account_info(),
                wallet.to_account_info(),
            ],
        )?;
    }

    Ok(())
}
