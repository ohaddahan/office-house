use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use crate::office_house_structs::auction_house::AuctionHouse;
use crate::constants::seeds::{FEE_PAYER, PREFIX};

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
