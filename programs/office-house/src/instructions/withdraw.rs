use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use crate::constants::seeds::{FEE_PAYER, PREFIX};
use crate::errorcodes::errors::Errors;
use crate::utils::assert::{assert_is_ata, assert_keys_equal};
use crate::utils::get_fee_payer::get_fee_payer;
use crate::utils::make_ata::make_ata;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use crate::office_house_structs::auction_house::AuctionHouse;

#[derive(Accounts)]
#[instruction(escrow_payment_bump: u8)]
pub struct Withdraw<'info> {
    pub(crate) wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub(crate) receipt_account: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), wallet.key().as_ref()], bump=escrow_payment_bump)]
    pub(crate) escrow_payment_account: UncheckedAccount<'info>,
    pub(crate) treasury_mint: Account<'info, Mint>,
    pub(crate) authority: UncheckedAccount<'info>,
    #[account(seeds=[PREFIX.as_bytes(), auction_house.creator.as_ref(), auction_house.treasury_mint.as_ref()], bump=auction_house.bump, has_one=authority, has_one=treasury_mint, has_one=auction_house_fee_account)]
    pub(crate) auction_house: Account<'info, AuctionHouse>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), FEE_PAYER.as_bytes()], bump=auction_house.fee_payer_bump)]
    pub(crate) auction_house_fee_account: UncheckedAccount<'info>,
    pub(crate) token_program: Program<'info, Token>,
    pub(crate) system_program: Program<'info, System>,
    pub(crate) ata_program: Program<'info, AssociatedToken>,
    pub(crate) rent: Sysvar<'info, Rent>,
}

pub fn withdraw<'info>(
    ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
    escrow_payment_bump: u8,
    amount: u64,
) -> ProgramResult {
    let wallet = &ctx.accounts.wallet;
    let receipt_account = &ctx.accounts.receipt_account;
    let escrow_payment_account = &ctx.accounts.escrow_payment_account;
    let authority = &ctx.accounts.authority;
    let auction_house = &ctx.accounts.auction_house;
    let auction_house_fee_account = &ctx.accounts.auction_house_fee_account;
    let treasury_mint = &ctx.accounts.treasury_mint;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let ata_program = &ctx.accounts.ata_program;
    let rent = &ctx.accounts.rent;

    let auction_house_key = auction_house.key();
    let seeds = [
        PREFIX.as_bytes(),
        auction_house_key.as_ref(),
        FEE_PAYER.as_bytes(),
        &[auction_house.fee_payer_bump],
    ];

    let ah_seeds = [
        PREFIX.as_bytes(),
        auction_house.creator.as_ref(),
        auction_house.treasury_mint.as_ref(),
        &[auction_house.bump],
    ];

    let auction_house_key = auction_house.key();
    let wallet_key = wallet.key();

    if !wallet.to_account_info().is_signer && !authority.to_account_info().is_signer {
        return Err(Errors::NoValidSignerPresent.into());
    }

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

    if !is_native {
        if receipt_account.data_is_empty() {
            make_ata(
                receipt_account.to_account_info(),
                wallet.to_account_info(),
                treasury_mint.to_account_info(),
                fee_payer.to_account_info(),
                ata_program.to_account_info(),
                token_program.to_account_info(),
                system_program.to_account_info(),
                rent.to_account_info(),
                &fee_seeds,
            )?;
        }

        let rec_acct = assert_is_ata(
            &receipt_account.to_account_info(),
            &wallet.key(),
            &treasury_mint.key(),
        )?;

        // make sure you cant get rugged
        if rec_acct.delegate.is_some() {
            return Err(Errors::BuyerATACannotHaveDelegate.into());
        }

        assert_is_ata(receipt_account, &wallet.key(), &treasury_mint.key())?;
        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                &escrow_payment_account.key(),
                &receipt_account.key(),
                &auction_house.key(),
                &[],
                amount,
            )?,
            &[
                escrow_payment_account.to_account_info(),
                receipt_account.to_account_info(),
                token_program.to_account_info(),
                auction_house.to_account_info(),
            ],
            &[&ah_seeds],
        )?;
    } else {
        assert_keys_equal(receipt_account.key(), wallet.key())?;
        invoke_signed(
            &system_instruction::transfer(
                &escrow_payment_account.key(),
                &receipt_account.key(),
                amount,
            ),
            &[
                escrow_payment_account.to_account_info(),
                receipt_account.to_account_info(),
                system_program.to_account_info(),
            ],
            &[&escrow_signer_seeds],
        )?;
    }

    Ok(())
}
