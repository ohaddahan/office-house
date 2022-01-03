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
use crate::errorcodes::errors::Errors;
use crate::utils::assert::{assert_is_ata, assert_keys_equal};
use crate::utils::make_ata::make_ata;
use crate::office_house_structs::update_auction_house::UpdateAuctionHouse;

pub fn update_auction_house<'info>(
    ctx: Context<'_, '_, '_, 'info, UpdateAuctionHouse<'info>>,
    seller_fee_basis_points: Option<u16>,
    requires_sign_off: Option<bool>,
    can_change_sale_price: Option<bool>,
) -> ProgramResult {
    let treasury_mint = &ctx.accounts.treasury_mint;
    let payer = &ctx.accounts.payer;
    let new_authority = &ctx.accounts.new_authority;
    let auction_house = &mut ctx.accounts.auction_house;
    let fee_withdrawal_destination = &ctx.accounts.fee_withdrawal_destination;
    let treasury_withdrawal_destination_owner =
        &ctx.accounts.treasury_withdrawal_destination_owner;
    let treasury_withdrawal_destination = &ctx.accounts.treasury_withdrawal_destination;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let ata_program = &ctx.accounts.ata_program;
    let rent = &ctx.accounts.rent;
    let is_native = treasury_mint.key() == spl_token::native_mint::id();

    if let Some(sfbp) = seller_fee_basis_points {
        if sfbp > 10000 {
            return Err(Errors::InvalidBasisPoints.into());
        }

        auction_house.seller_fee_basis_points = sfbp;
    }

    if let Some(rqf) = requires_sign_off {
        auction_house.requires_sign_off = rqf;
    }
    if let Some(chsp) = can_change_sale_price {
        auction_house.can_change_sale_price = chsp;
    }

    auction_house.authority = new_authority.key();
    auction_house.treasury_withdrawal_destination = treasury_withdrawal_destination.key();
    auction_house.fee_withdrawal_destination = fee_withdrawal_destination.key();

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
