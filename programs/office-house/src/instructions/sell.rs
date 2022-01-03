use anchor_lang::prelude::*;
use spl_token::instruction::{approve};
use anchor_lang::solana_program::{program::invoke};
use crate::constants::seeds::{FEE_PAYER, PREFIX};
use crate::constants::trade_state_size::TRADE_STATE_SIZE;
use crate::errorcodes::errors::Errors;
use crate::office_house_structs::sell::Sell;
use crate::utils::assert::{assert_is_ata, assert_metadata_valid};
use crate::utils::create_or_allocate_account_raw::create_or_allocate_account_raw;
use crate::utils::get_fee_payer::get_fee_payer;

pub fn sell<'info>(
    ctx: Context<'_, '_, '_, 'info, Sell<'info>>,
    trade_state_bump: u8,
    _free_trade_state_bump: u8,
    _program_as_signer_bump: u8,
    buyer_price: u64,
    token_size: u64,
) -> ProgramResult {
    let wallet = &ctx.accounts.wallet;
    let token_account = &ctx.accounts.token_account;
    let metadata = &ctx.accounts.metadata;
    let authority = &ctx.accounts.authority;
    let seller_trade_state = &ctx.accounts.seller_trade_state;
    let free_seller_trade_state = &ctx.accounts.free_seller_trade_state;
    let auction_house = &ctx.accounts.auction_house;
    let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let program_as_signer = &ctx.accounts.program_as_signer;
    let rent = &ctx.accounts.rent;

    if !wallet.to_account_info().is_signer {
        if buyer_price == 0 {
            return Err(Errors::SaleRequiresSigner.into());
        } else {
            if free_seller_trade_state.data_is_empty() {
                return Err(Errors::SaleRequiresSigner.into());
            } else if !free_seller_trade_state.data_is_empty()
                && (!authority.to_account_info().is_signer
                || !auction_house.can_change_sale_price)
            {
                return Err(Errors::SaleRequiresSigner.into());
            }
        }
    }

    let auction_house_key = auction_house.key();

    let seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        FEE_PAYER.as_bytes(),
        &[auction_house.fee_payer_bump],
    ];

    let (fee_payer, fee_seeds) = get_fee_payer(
        authority,
        auction_house,
        wallet.to_account_info(),
        auction_house_fee_account.to_account_info(),
        &seeds,
    )?;

    assert_is_ata(
        &token_account.to_account_info(),
        &wallet.key(),
        &token_account.mint,
    )?;

    assert_metadata_valid(metadata, token_account)?;

    if token_size > token_account.amount {
        return Err(Errors::InvalidTokenAmount.into());
    }

    if wallet.is_signer {
        invoke(
            &approve(
                &token_program.key(),
                &token_account.key(),
                &program_as_signer.key(),
                &wallet.key(),
                &[],
                token_size,
            )
                .unwrap(),
            &[
                token_program.to_account_info(),
                token_account.to_account_info(),
                program_as_signer.to_account_info(),
                wallet.to_account_info(),
            ],
        )?;
    }

    let ts_info = seller_trade_state.to_account_info();
    if ts_info.data_is_empty() {
        let token_account_key = token_account.key();
        let wallet_key = wallet.key();
        let ts_seeds = [
            PREFIX.as_bytes(),
            wallet_key.as_ref(),
            auction_house_key.as_ref(),
            token_account_key.as_ref(),
            auction_house.treasury_mint.as_ref(),
            token_account.mint.as_ref(),
            &buyer_price.to_le_bytes(),
            &token_size.to_le_bytes(),
            &[trade_state_bump],
        ];
        create_or_allocate_account_raw(
            *ctx.program_id,
            &ts_info,
            &rent.to_account_info(),
            &system_program,
            &fee_payer,
            TRADE_STATE_SIZE,
            fee_seeds,
            &ts_seeds,
        )?;
    }

    let data = &mut ts_info.data.borrow_mut();
    data[0] = trade_state_bump;

    Ok(())
}
