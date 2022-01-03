use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use crate::office_house_structs::auction_house::AuctionHouse;
use crate::constants::seeds::{FEE_PAYER, PREFIX, TREASURY};
use crate::constants::office_size::OFFICE_HOUSE_SIZE;

#[derive(Accounts)]
#[instruction(bump: u8, fee_payer_bump: u8, treasury_bump: u8)]
pub struct CreateAuctionHouse<'info> {
    treasury_mint: Account<'info, Mint>,
    payer: Signer<'info>,
    authority: UncheckedAccount<'info>,
    #[account(mut)]
    fee_withdrawal_destination: UncheckedAccount<'info>,
    #[account(mut)]
    treasury_withdrawal_destination: UncheckedAccount<'info>,
    treasury_withdrawal_destination_owner: UncheckedAccount<'info>,
    #[account(init, seeds=[PREFIX.as_bytes(), authority.key().as_ref(), treasury_mint.key().as_ref()], bump=bump, space=OFFICE_HOUSE_SIZE, payer=payer)]
    auction_house: Account<'info, AuctionHouse>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), FEE_PAYER.as_bytes()], bump=fee_payer_bump)]
    auction_house_fee_account: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), TREASURY.as_bytes()], bump=treasury_bump)]
    auction_house_treasury: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    ata_program: Program<'info, AssociatedToken>,
    rent: Sysvar<'info, Rent>,
}
