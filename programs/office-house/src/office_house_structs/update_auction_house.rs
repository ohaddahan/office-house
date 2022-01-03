use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use crate::office_house_structs::auction_house::AuctionHouse;
use crate::constants::seeds::{FEE_PAYER, PREFIX};

#[derive(Accounts)]
pub struct UpdateAuctionHouse<'info> {
    pub(crate) treasury_mint: Account<'info, Mint>,
    pub(crate) payer: Signer<'info>,
    authority: Signer<'info>,
    pub(crate) new_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub(crate) fee_withdrawal_destination: UncheckedAccount<'info>,
    #[account(mut)]
    pub(crate) treasury_withdrawal_destination: UncheckedAccount<'info>,
    pub(crate) treasury_withdrawal_destination_owner: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.creator.as_ref(), treasury_mint.key().as_ref()], bump=auction_house.bump, has_one=authority, has_one=treasury_mint)]
    pub(crate) auction_house: Account<'info, AuctionHouse>,
    pub(crate) token_program: Program<'info, Token>,
    pub(crate) system_program: Program<'info, System>,
    pub(crate) ata_program: Program<'info, AssociatedToken>,
    pub(crate) rent: Sysvar<'info, Rent>,
}
