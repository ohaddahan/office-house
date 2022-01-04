use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use crate::constants::seeds::{TREASURY, PREFIX};
use anchor_spl::token::{Mint, Token};
use crate::office_house_structs::auction_house::AuctionHouse;

#[derive(Accounts)]
pub struct WithdrawFromTreasury<'info> {
    pub(crate) treasury_mint: Account<'info, Mint>,
    authority: Signer<'info>,
    #[account(mut)]
    pub(crate) treasury_withdrawal_destination: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), TREASURY.as_bytes()], bump=auction_house.treasury_bump)]
    pub(crate) auction_house_treasury: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.creator.as_ref(), treasury_mint.key().as_ref()], bump=auction_house.bump, has_one=authority, has_one=treasury_mint, has_one=treasury_withdrawal_destination, has_one=auction_house_treasury)]
    pub(crate) auction_house: Account<'info, AuctionHouse>,
    pub(crate) token_program: Program<'info, Token>,
    pub(crate) system_program: Program<'info, System>,
}

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
