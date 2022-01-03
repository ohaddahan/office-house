use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use spl_token::instruction::{initialize_account2, revoke};
use spl_associated_token_account::get_associated_token_address;
use anchor_lang::solana_program::program_pack::IsInitialized;
use anchor_lang::solana_program::program_pack::Pack;
use std::{convert::TryInto, slice::Iter};
use arrayref::array_ref;
use metaplex_token_metadata::state::Metadata;
use anchor_lang::solana_program::{program::invoke_signed, program_option::COption, system_instruction};
use anchor_lang::solana_program::program::invoke;
use crate::constants::seeds::{FEE_PAYER, PREFIX, TREASURY};
use crate::errorcodes::errors::Errors;
use crate::instructions::create_office_house::CreateAuctionHouse;
use crate::office_house_structs::cancel::Cancel;
use crate::utils::assert::{assert_is_ata, assert_keys_equal};
use crate::utils::create_missing::create_program_token_account_if_not_present;
use crate::utils::get_fee_payer::get_fee_payer;
use crate::utils::make_ata::make_ata;

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
