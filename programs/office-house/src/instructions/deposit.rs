use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::invoke};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::constants::seeds::{FEE_PAYER, PREFIX};
use crate::utils::assert::{assert_is_ata, assert_keys_equal};
use crate::utils::create_missing::create_program_token_account_if_not_present;
use crate::utils::get_fee_payer::get_fee_payer;
use crate::office_house_structs::auction_house::AuctionHouse;

#[derive(Accounts)]
#[instruction(escrow_payment_bump: u8)]
pub struct Deposit<'info> {
    pub(crate) wallet: Signer<'info>,
    #[account(mut)]
    pub(crate) payment_account: UncheckedAccount<'info>,
    pub(crate) transfer_authority: UncheckedAccount<'info>,
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
    pub(crate) rent: Sysvar<'info, Rent>,
}

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
