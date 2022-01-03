use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use spl_token::instruction::initialize_account2;
use spl_token::state::Account;
use spl_associated_token_account::get_associated_token_address;
use anchor_lang::solana_program::program_pack::IsInitialized;
use anchor_lang::solana_program::program_pack::Pack;
use std::{convert::TryInto, slice::Iter};
use arrayref::array_ref;
use metaplex_token_metadata::state::Metadata;
use anchor_lang::solana_program::{program::invoke_signed, program_option::COption, system_instruction};
use crate::errorcodes::errors::Errors;
use crate::office_house_structs::auction_house::AuctionHouse;

pub fn get_fee_payer<'a, 'b>(
    authority: &UncheckedAccount,
    auction_house: &anchor_lang::Account<AuctionHouse>,
    wallet: AccountInfo<'a>,
    auction_house_fee_account: AccountInfo<'a>,
    auction_house_seeds: &'b [&'b [u8]],
) -> Result<(AccountInfo<'a>, &'b [&'b [u8]]), ProgramError> {
    let mut seeds: &[&[u8]] = &[];
    let fee_payer: AccountInfo;
    if authority.to_account_info().is_signer {
        seeds = auction_house_seeds;
        fee_payer = auction_house_fee_account;
    } else if wallet.is_signer {
        if auction_house.requires_sign_off {
            return Err(Errors::CannotTakeThisActionWithoutAuctionHouseSignOff.into());
        }
        fee_payer = wallet
    } else {
        return Err(Errors::NoPayerPresent.into());
    };

    Ok((fee_payer, &seeds))
}
