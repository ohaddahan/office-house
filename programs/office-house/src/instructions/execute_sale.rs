use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, program_option::COption, system_instruction, program::invoke};
use metaplex_token_metadata::utils::assert_derivation;
use crate::constants::seeds::{FEE_PAYER, PREFIX, SIGNER};
use crate::errorcodes::errors::Errors;
use crate::office_house_structs::execute_sale::ExecuteSale;
use crate::utils::assert::{assert_is_ata, assert_keys_equal};
use crate::utils::get_delegate_from_token_account::get_delegate_from_token_account;
use crate::utils::get_fee_payer::get_fee_payer;
use crate::utils::get_mint_from_token_account::get_mint_from_token_account;
use crate::utils::make_ata::make_ata;
use crate::utils::pay_auction_house_fees::pay_auction_house_fees;
use crate::utils::pay_creator_fees::pay_creator_fees;

pub fn execute_sale<'info>(
    ctx: Context<'_, '_, '_, 'info, ExecuteSale<'info>>,
    escrow_payment_bump: u8,
    _free_trade_state_bump: u8,
    program_as_signer_bump: u8,
    buyer_price: u64,
    token_size: u64,
) -> ProgramResult {
    let buyer = &ctx.accounts.buyer;
    let seller = &ctx.accounts.seller;
    let token_account = &ctx.accounts.token_account;
    let token_mint = &ctx.accounts.token_mint;
    let metadata = &ctx.accounts.metadata;
    let treasury_mint = &ctx.accounts.treasury_mint;
    let seller_payment_receipt_account = &ctx.accounts.seller_payment_receipt_account;
    let buyer_receipt_token_account = &ctx.accounts.buyer_receipt_token_account;
    let escrow_payment_account = &ctx.accounts.escrow_payment_account;
    let authority = &ctx.accounts.authority;
    let auction_house = &ctx.accounts.auction_house;
    let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
    let auction_house_treasury = &ctx.accounts.auction_house_treasury;
    let buyer_trade_state = &ctx.accounts.buyer_trade_state;
    let seller_trade_state = &ctx.accounts.seller_trade_state;
    let free_trade_state = &ctx.accounts.free_trade_state;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let ata_program = &ctx.accounts.ata_program;
    let program_as_signer = &ctx.accounts.program_as_signer;
    let rent = &ctx.accounts.rent;

    let metadata_clone = metadata.to_account_info();
    let escrow_clone = escrow_payment_account.to_account_info();
    let auction_house_clone = auction_house.to_account_info();
    let ata_clone = ata_program.to_account_info();
    let token_clone = token_program.to_account_info();
    let sys_clone = system_program.to_account_info();
    let rent_clone = rent.to_account_info();
    let treasury_clone = auction_house_treasury.to_account_info();
    let authority_clone = authority.to_account_info();
    let buyer_receipt_clone = buyer_receipt_token_account.to_account_info();
    let token_account_clone = token_account.to_account_info();

    let is_native = treasury_mint.key() == spl_token::native_mint::id();

    if buyer_price == 0 && !authority_clone.is_signer && !seller.is_signer {
        return Err(Errors::CannotMatchFreeSalesWithoutAuctionHouseOrSellerSignoff.into());
    }

    let token_account_mint = get_mint_from_token_account(&token_account_clone)?;

    assert_keys_equal(token_mint.key(), token_account_mint)?;
    let delegate = get_delegate_from_token_account(&token_account_clone)?;
    if let Some(d) = delegate {
        assert_keys_equal(program_as_signer.key(), d)?;
    } else {
        msg!("No delegate detected on token account.");
        return Err(Errors::BothPartiesNeedToAgreeToSale.into());
    }

    if buyer_trade_state.data_is_empty() || seller_trade_state.data_is_empty() {
        return Err(Errors::BothPartiesNeedToAgreeToSale.into());
    }

    let auction_house_key = auction_house.key();
    let seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        FEE_PAYER.as_bytes(),
        &[auction_house.fee_payer_bump],
    ];

    let wallet_to_use = if buyer.is_signer { buyer } else { seller };

    let (fee_payer, fee_payer_seeds) = get_fee_payer(
        authority,
        auction_house,
        wallet_to_use.to_account_info(),
        auction_house_fee_account.to_account_info(),
        &seeds,
    )?;
    let fee_payer_clone = fee_payer.to_account_info();

    assert_is_ata(
        &token_account.to_account_info(),
        &seller.key(),
        &token_account_mint,
    )?;

    assert_derivation(
        &metaplex_token_metadata::id(),
        &metadata.to_account_info(),
        &[
            metaplex_token_metadata::state::PREFIX.as_bytes(),
            metaplex_token_metadata::id().as_ref(),
            token_account_mint.as_ref(),
        ],
    )?;

    if metadata.data_is_empty() {
        return Err(Errors::MetadataDoesntExist.into());
    }

    let auction_house_key = auction_house.key();
    let wallet_key = buyer.key();
    let escrow_signer_seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        wallet_key.as_ref(),
        &[escrow_payment_bump],
    ];

    let ah_seeds = [
        PREFIX.as_bytes(),
        auction_house.creator.as_ref(),
        auction_house.treasury_mint.as_ref(),
        &[auction_house.bump],
    ];

    // with the native account, the escrow is it's own owner,
    // whereas with token, it is the auction house that is owner.
    let signer_seeds_for_royalties = if is_native {
        escrow_signer_seeds
    } else {
        ah_seeds
    };

    let buyer_leftover_after_royalties = pay_creator_fees(
        &mut ctx.remaining_accounts.iter(),
        &metadata_clone,
        &escrow_clone,
        &auction_house_clone,
        &fee_payer_clone,
        treasury_mint,
        &ata_clone,
        &token_clone,
        &sys_clone,
        &rent_clone,
        &signer_seeds_for_royalties,
        &fee_payer_seeds,
        buyer_price,
        is_native,
    )?;

    let auction_house_fee_paid = pay_auction_house_fees(
        &auction_house,
        &treasury_clone,
        &escrow_clone,
        &token_clone,
        &sys_clone,
        &signer_seeds_for_royalties,
        buyer_price,
        is_native,
    )?;

    let buyer_leftover_after_royalties_and_house_fee = buyer_leftover_after_royalties
        .checked_sub(auction_house_fee_paid)
        .ok_or(Errors::NumericalOverflow)?;

    if !is_native {
        if seller_payment_receipt_account.data_is_empty() {
            make_ata(
                seller_payment_receipt_account.to_account_info(),
                seller.to_account_info(),
                treasury_mint.to_account_info(),
                fee_payer.to_account_info(),
                ata_program.to_account_info(),
                token_program.to_account_info(),
                system_program.to_account_info(),
                rent.to_account_info(),
                &fee_payer_seeds,
            )?;
        }

        let seller_rec_acct = assert_is_ata(
            &seller_payment_receipt_account.to_account_info(),
            &seller.key(),
            &treasury_mint.key(),
        )?;

        // make sure you cant get rugged
        if seller_rec_acct.delegate.is_some() {
            return Err(Errors::SellerATACannotHaveDelegate.into());
        }

        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                &escrow_payment_account.key(),
                &seller_payment_receipt_account.key(),
                &auction_house.key(),
                &[],
                buyer_leftover_after_royalties_and_house_fee,
            )?,
            &[
                escrow_payment_account.to_account_info(),
                seller_payment_receipt_account.to_account_info(),
                token_program.to_account_info(),
                auction_house.to_account_info(),
            ],
            &[&ah_seeds],
        )?;
    } else {
        assert_keys_equal(seller_payment_receipt_account.key(), seller.key())?;
        invoke_signed(
            &system_instruction::transfer(
                &escrow_payment_account.key,
                seller_payment_receipt_account.key,
                buyer_leftover_after_royalties_and_house_fee,
            ),
            &[
                escrow_payment_account.to_account_info(),
                seller_payment_receipt_account.to_account_info(),
                system_program.to_account_info(),
            ],
            &[&escrow_signer_seeds],
        )?;
    }

    if buyer_receipt_token_account.data_is_empty() {
        make_ata(
            buyer_receipt_token_account.to_account_info(),
            buyer.to_account_info(),
            token_mint.to_account_info(),
            fee_payer.to_account_info(),
            ata_program.to_account_info(),
            token_program.to_account_info(),
            system_program.to_account_info(),
            rent.to_account_info(),
            &fee_payer_seeds,
        )?;
    }

    let buyer_rec_acct = assert_is_ata(&buyer_receipt_clone, &buyer.key(), &token_mint.key())?;

    // make sure you cant get rugged
    if buyer_rec_acct.delegate.is_some() {
        return Err(Errors::BuyerATACannotHaveDelegate.into());
    }

    let program_as_signer_seeds = [
        PREFIX.as_bytes(),
        SIGNER.as_bytes(),
        &[program_as_signer_bump],
    ];

    invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            &token_account.key(),
            &buyer_receipt_token_account.key(),
            &program_as_signer.key(),
            &[],
            token_size,
        )?,
        &[
            token_account.to_account_info(),
            buyer_receipt_clone,
            program_as_signer.to_account_info(),
            token_clone,
        ],
        &[&program_as_signer_seeds],
    )?;

    let curr_seller_lamp = seller_trade_state.lamports();
    **seller_trade_state.lamports.borrow_mut() = 0;

    **fee_payer.lamports.borrow_mut() = fee_payer
        .lamports()
        .checked_add(curr_seller_lamp)
        .ok_or(Errors::NumericalOverflow)?;

    let curr_buyer_lamp = buyer_trade_state.lamports();
    **buyer_trade_state.lamports.borrow_mut() = 0;

    **fee_payer.lamports.borrow_mut() = fee_payer
        .lamports()
        .checked_add(curr_buyer_lamp)
        .ok_or(Errors::NumericalOverflow)?;

    if free_trade_state.lamports() > 0 {
        let curr_buyer_lamp = free_trade_state.lamports();
        **free_trade_state.lamports.borrow_mut() = 0;

        **fee_payer.lamports.borrow_mut() = fee_payer
            .lamports()
            .checked_add(curr_buyer_lamp)
            .ok_or(Errors::NumericalOverflow)?;
    }
    Ok(())
}
