use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::office_house_structs::auction_house::AuctionHouse;
use crate::constants::seeds::{FEE_PAYER, PREFIX, TREASURY};
use crate::constants::office_size::OFFICE_HOUSE_SIZE;

#[derive(Accounts)]
#[instruction(buyer_price: u64, token_size: u64)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub(crate) wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub(crate) token_account: Account<'info, TokenAccount>,
    pub(crate) token_mint: Account<'info, Mint>,
    pub(crate) authority: UncheckedAccount<'info>,
    #[account(seeds=[PREFIX.as_bytes(), auction_house.creator.as_ref(), auction_house.treasury_mint.as_ref()], bump=auction_house.bump, has_one=authority, has_one=auction_house_fee_account)]
    pub(crate) auction_house: Account<'info, AuctionHouse>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), FEE_PAYER.as_bytes()], bump=auction_house.fee_payer_bump)]
    pub(crate) auction_house_fee_account: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), wallet.key().as_ref(), auction_house.key().as_ref(), token_account.key().as_ref(), auction_house.treasury_mint.as_ref(), token_mint.key().as_ref(), &buyer_price.to_le_bytes(), &token_size.to_le_bytes()], bump=trade_state.to_account_info().data.borrow()[0])]
    pub(crate) trade_state: UncheckedAccount<'info>,
    pub(crate) token_program: Program<'info, Token>,
}
