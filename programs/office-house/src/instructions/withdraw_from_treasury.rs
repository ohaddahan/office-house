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
use anchor_lang::solana_program::{program::invoke_signed, program_option::COption, system_instruction};
use crate::constants::seeds::{TREASURY, PREFIX};
use crate::office_house_structs::withdraw_from_treasury::WithdrawFromTreasury;

pub fn withdraw_from_treasury<'info>(
    ctx: Context<'_, '_, '_, 'info, WithdrawFromTreasury<'info>>,
    amount: u64,
) -> ProgramResult {
    let treasury_mint = &ctx.accounts.treasury_mint;
    let treasury_withdrawal_destination = &ctx.accounts.treasury_withdrawal_destination;
    let auction_house_treasury = &ctx.accounts.auction_house_treasury;
    let auction_house = &ctx.accounts.auction_house;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;

    let is_native = treasury_mint.key() == spl_token::native_mint::id();
    let auction_house_seeds = [
        PREFIX.as_bytes(),
        auction_house.creator.as_ref(),
        auction_house.treasury_mint.as_ref(),
        &[auction_house.bump],
    ];

    let ah_key = auction_house.key();
    let auction_house_treasury_seeds = [
        PREFIX.as_bytes(),
        ah_key.as_ref(),
        TREASURY.as_bytes(),
        &[auction_house.treasury_bump],
    ];
    if !is_native {
        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                &auction_house_treasury.key(),
                &treasury_withdrawal_destination.key(),
                &auction_house.key(),
                &[],
                amount,
            )?,
            &[
                auction_house_treasury.to_account_info(),
                treasury_withdrawal_destination.to_account_info(),
                token_program.to_account_info(),
                auction_house.to_account_info(),
            ],
            &[&auction_house_seeds],
        )?;
    } else {
        invoke_signed(
            &system_instruction::transfer(
                &auction_house_treasury.key(),
                &treasury_withdrawal_destination.key(),
                amount,
            ),
            &[
                auction_house_treasury.to_account_info(),
                treasury_withdrawal_destination.to_account_info(),
                system_program.to_account_info(),
            ],
            &[&auction_house_treasury_seeds],
        )?;
    }

    Ok(())
}
