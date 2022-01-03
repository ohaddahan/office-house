use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use spl_token::instruction::initialize_account2;
use spl_associated_token_account::get_associated_token_address;
use anchor_lang::solana_program::program_pack::IsInitialized;
use anchor_lang::solana_program::program_pack::Pack;
use std::{convert::TryInto, slice::Iter};
use arrayref::array_ref;
use metaplex_token_metadata::state::Metadata;
use anchor_lang::solana_program::{program::invoke_signed, program_option::COption, system_instruction, program::invoke};
use crate::constants::seeds::{FEE_PAYER, PREFIX};
use crate::office_house_structs::deposit::Deposit;
use crate::utils::assert::{assert_is_ata, assert_keys_equal};
use crate::utils::create_missing::create_program_token_account_if_not_present;
use crate::utils::get_fee_payer::get_fee_payer;


pub fn deposit<'info>(
    ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
    escrow_payment_bump: u8,
    amount: u64,
) -> ProgramResult {
    let wallet = &ctx.accounts.wallet;
    let payment_account = &ctx.accounts.payment_account;
    let transfer_authority = &ctx.accounts.transfer_authority;
    let escrow_payment_account = &ctx.accounts.escrow_payment_account;
    let authority = &ctx.accounts.authority;
    let auction_house = &ctx.accounts.auction_house;
    let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
    let treasury_mint = &ctx.accounts.treasury_mint;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let rent = &ctx.accounts.rent;

    let auction_house_key = auction_house.key();
    let seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        FEE_PAYER.as_bytes(),
        &[auction_house.fee_payer_bump],
    ];
    let wallet_key = wallet.key();

    let escrow_signer_seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        wallet_key.as_ref(),
        &[escrow_payment_bump],
    ];

    let (fee_payer, fee_seeds) = get_fee_payer(
        authority,
        auction_house,
        wallet.to_account_info(),
        auction_house_fee_account.to_account_info(),
        &seeds,
    )?;

    let is_native = treasury_mint.key() == spl_token::native_mint::id();

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

    if !is_native {
        assert_is_ata(payment_account, &wallet.key(), &treasury_mint.key())?;
        invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                &payment_account.key(),
                &escrow_payment_account.key(),
                &transfer_authority.key(),
                &[],
                amount,
            )?,
            &[
                escrow_payment_account.to_account_info(),
                payment_account.to_account_info(),
                token_program.to_account_info(),
                transfer_authority.to_account_info(),
            ],
        )?;
    } else {
        assert_keys_equal(payment_account.key(), wallet.key())?;
        invoke(
            &system_instruction::transfer(
                &payment_account.key(),
                &escrow_payment_account.key(),
                amount,
            ),
            &[
                escrow_payment_account.to_account_info(),
                payment_account.to_account_info(),
                system_program.to_account_info(),
            ],
        )?;
    }

    Ok(())
}
