use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::invoke};
use crate::constants::seeds::{FEE_PAYER, PREFIX};
use crate::constants::trade_state_size::TRADE_STATE_SIZE;
use crate::errorcodes::errors::Errors;
use crate::office_house_structs::buy::Buy;
use crate::utils::assert::{assert_initialized, assert_keys_equal, assert_metadata_valid};
use crate::utils::create_missing::create_program_token_account_if_not_present;
use crate::utils::create_or_allocate_account_raw::create_or_allocate_account_raw;
use crate::utils::get_fee_payer::get_fee_payer;

pub fn buy<'info>(
    ctx: Context<'_, '_, '_, 'info, Buy<'info>>,
    trade_state_bump: u8,
    escrow_payment_bump: u8,
    buyer_price: u64,
    token_size: u64,
) -> ProgramResult {
    let wallet = &ctx.accounts.wallet;
    let payment_account = &ctx.accounts.payment_account;
    let transfer_authority = &ctx.accounts.transfer_authority;
    let treasury_mint = &ctx.accounts.treasury_mint;
    let metadata = &ctx.accounts.metadata;
    let token_account = &ctx.accounts.token_account;
    let escrow_payment_account = &ctx.accounts.escrow_payment_account;
    let authority = &ctx.accounts.authority;
    let auction_house = &ctx.accounts.auction_house;
    let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
    let buyer_trade_state = &mut ctx.accounts.buyer_trade_state;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let rent = &ctx.accounts.rent;

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

    let is_native = treasury_mint.key() == spl_token::native_mint::id();

    let auction_house_key = auction_house.key();
    let wallet_key = wallet.key();
    let escrow_signer_seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        wallet_key.as_ref(),
        &[escrow_payment_bump],
    ];

    create_program_token_account_if_not_present(
        escrow_payment_account,
        system_program,
        &fee_payer,
        token_program,
        treasury_mint,
        &auction_house.to_account_info(),
        rent,
        &escrow_signer_seeds,
        fee_seeds,
        is_native,
    )?;

    if is_native {
        assert_keys_equal(wallet.key(), payment_account.key())?;

        if escrow_payment_account.lamports() < buyer_price {
            let diff = buyer_price
                .checked_sub(escrow_payment_account.lamports())
                .ok_or(Errors::NumericalOverflow)?;
            invoke(
                &system_instruction::transfer(
                    &payment_account.key(),
                    &escrow_payment_account.key(),
                    diff,
                ),
                &[
                    payment_account.to_account_info(),
                    escrow_payment_account.to_account_info(),
                    system_program.to_account_info(),
                ],
            )?;
        }
    } else {
        let escrow_payment_loaded: spl_token::state::Account =
            assert_initialized(escrow_payment_account)?;

        if escrow_payment_loaded.amount < buyer_price {
            let diff = buyer_price
                .checked_sub(escrow_payment_loaded.amount)
                .ok_or(Errors::NumericalOverflow)?;
            invoke(
                &spl_token::instruction::transfer(
                    &token_program.key(),
                    &payment_account.key(),
                    &escrow_payment_account.key(),
                    &transfer_authority.key(),
                    &[],
                    diff,
                )?,
                &[
                    transfer_authority.to_account_info(),
                    payment_account.to_account_info(),
                    escrow_payment_account.to_account_info(),
                    token_program.to_account_info(),
                ],
            )?;
        }
    }

    assert_metadata_valid(metadata, token_account)?;

    let ts_info = buyer_trade_state.to_account_info();
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
