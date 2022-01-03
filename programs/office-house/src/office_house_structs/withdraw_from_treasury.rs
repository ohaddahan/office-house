use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::office_house_structs::auction_house::AuctionHouse;
use crate::constants::seeds::{TREASURY, PREFIX};

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
