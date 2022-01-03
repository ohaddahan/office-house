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
use crate::constants::seeds::{PREFIX, TREASURY};
use crate::errorcodes::errors::Errors;
use crate::instructions::create_office_house::CreateAuctionHouse;
use crate::utils::assert::{assert_is_ata, assert_keys_equal};
use crate::utils::create_missing::create_program_token_account_if_not_present;
use crate::utils::make_ata::make_ata;

pub fn create_auction_house<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateAuctionHouse<'info>>,
    bump: u8,
    fee_payer_bump: u8,
    treasury_bump: u8,
    seller_fee_basis_points: u16,
    requires_sign_off: bool,
    can_change_sale_price: bool,
) -> ProgramResult {
    let treasury_mint = &ctx.accounts.treasury_mint;
    let payer = &ctx.accounts.payer;
    let authority = &ctx.accounts.authority;
    let auction_house = &mut ctx.accounts.auction_house;
    let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
    let auction_house_treasury = &ctx.accounts.auction_house_treasury;
    let fee_withdrawal_destination = &ctx.accounts.fee_withdrawal_destination;
    let treasury_withdrawal_destination_owner =
        &ctx.accounts.treasury_withdrawal_destination_owner;
    let treasury_withdrawal_destination = &ctx.accounts.treasury_withdrawal_destination;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let ata_program = &ctx.accounts.ata_program;
    let rent = &ctx.accounts.rent;

    auction_house.bump = bump;
    auction_house.fee_payer_bump = fee_payer_bump;
    auction_house.treasury_bump = treasury_bump;
    if seller_fee_basis_points > 10000 {
        return Err(Errors::InvalidBasisPoints.into());
    }
    auction_house.seller_fee_basis_points = seller_fee_basis_points;
    auction_house.requires_sign_off = requires_sign_off;
    auction_house.can_change_sale_price = can_change_sale_price;
    auction_house.creator = authority.key();
    auction_house.authority = authority.key();
    auction_house.treasury_mint = treasury_mint.key();
    auction_house.auction_house_fee_account = auction_house_fee_account.key();
    auction_house.auction_house_treasury = auction_house_treasury.key();
    auction_house.treasury_withdrawal_destination = treasury_withdrawal_destination.key();
    auction_house.fee_withdrawal_destination = fee_withdrawal_destination.key();

    let is_native = treasury_mint.key() == spl_token::native_mint::id();

    let ah_key = auction_house.key();

    let auction_house_treasury_seeds = [
        PREFIX.as_bytes(),
        ah_key.as_ref(),
        TREASURY.as_bytes(),
        &[treasury_bump],
    ];

    create_program_token_account_if_not_present(
        auction_house_treasury,
        system_program,
        &payer,
        token_program,
        treasury_mint,
        &auction_house.to_account_info(),
        rent,
        &auction_house_treasury_seeds,
        &[],
        is_native,
    )?;

    if !is_native {
        if treasury_withdrawal_destination.data_is_empty() {
            make_ata(
                treasury_withdrawal_destination.to_account_info(),
                treasury_withdrawal_destination_owner.to_account_info(),
                treasury_mint.to_account_info(),
                payer.to_account_info(),
                ata_program.to_account_info(),
                token_program.to_account_info(),
                system_program.to_account_info(),
                rent.to_account_info(),
                &[],
            )?;
        }

        assert_is_ata(
            &treasury_withdrawal_destination.to_account_info(),
            &treasury_withdrawal_destination_owner.key(),
            &treasury_mint.key(),
        )?;
    } else {
        assert_keys_equal(
            treasury_withdrawal_destination.key(),
            treasury_withdrawal_destination_owner.key(),
        )?;
    }

    Ok(())
}
