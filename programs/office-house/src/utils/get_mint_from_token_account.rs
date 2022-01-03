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

/// Cheap method to just grab mint Pubkey from token account, instead of deserializing entire thing
pub fn get_mint_from_token_account(
    token_account_info: &AccountInfo,
) -> Result<Pubkey, ProgramError> {
    // TokeAccount layout:   mint(32), owner(32), ...
    let data = token_account_info.try_borrow_data()?;
    let mint_data = array_ref![data, 0, 32];
    Ok(Pubkey::new_from_array(*mint_data))
}
