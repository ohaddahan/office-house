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
use crate::constants::seeds::{FEE_PAYER, PREFIX};
use crate::office_house_structs::withdraw_from_fee::WithdrawFromFee;


pub fn withdraw_from_fee<'info>(
    ctx: Context<'_, '_, '_, 'info, WithdrawFromFee<'info>>,
    amount: u64,
) -> ProgramResult {
    let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
    let fee_withdrawal_destination = &ctx.accounts.fee_withdrawal_destination;
    let auction_house = &ctx.accounts.auction_house;
    let system_program = &ctx.accounts.system_program;

    let auction_house_key = auction_house.key();
    let seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        FEE_PAYER.as_bytes(),
        &[auction_house.fee_payer_bump],
    ];

    invoke_signed(
        &system_instruction::transfer(
            &auction_house_fee_account.key(),
            &fee_withdrawal_destination.key(),
            amount,
        ),
        &[
            auction_house_fee_account.to_account_info(),
            fee_withdrawal_destination.to_account_info(),
            system_program.to_account_info(),
        ],
        &[&seeds],
    )?;

    Ok(())
}
