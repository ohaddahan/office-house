use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use crate::office_house_structs::auction_house::AuctionHouse;
use crate::constants::seeds::{FEE_PAYER, PREFIX, TREASURY, SIGNER};

#[derive(Accounts)]
#[instruction(escrow_payment_bump: u8, free_trade_state_bump: u8, program_as_signer_bump: u8, buyer_price: u64, token_size: u64)]
pub struct ExecuteSale<'info> {
    #[account(mut)]
    pub(crate) buyer: UncheckedAccount<'info>,
    #[account(mut)]
    pub(crate) seller: UncheckedAccount<'info>,
    // cannot mark these as real Accounts or else we blow stack size limit
    #[account(mut)]
    pub(crate) token_account: UncheckedAccount<'info>,
    pub(crate) token_mint: UncheckedAccount<'info>,
    pub(crate) metadata: UncheckedAccount<'info>,
    // cannot mark these as real Accounts or else we blow stack size limit
    pub(crate) treasury_mint: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), buyer.key().as_ref()], bump=escrow_payment_bump)]
    pub(crate) escrow_payment_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub(crate) seller_payment_receipt_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub(crate) buyer_receipt_token_account: UncheckedAccount<'info>,
    pub(crate) authority: UncheckedAccount<'info>,
    #[account(seeds=[PREFIX.as_bytes(), auction_house.creator.as_ref(), auction_house.treasury_mint.as_ref()], bump=auction_house.bump, has_one=authority, has_one=treasury_mint, has_one=auction_house_treasury, has_one=auction_house_fee_account)]
    pub(crate) auction_house: Account<'info, AuctionHouse>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), FEE_PAYER.as_bytes()], bump=auction_house.fee_payer_bump)]
    pub(crate) auction_house_fee_account: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), auction_house.key().as_ref(), TREASURY.as_bytes()], bump=auction_house.treasury_bump)]
    pub(crate) auction_house_treasury: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), buyer.key().as_ref(), auction_house.key().as_ref(), token_account.key().as_ref(), auction_house.treasury_mint.as_ref(), token_mint.key().as_ref(), &buyer_price.to_le_bytes(), &token_size.to_le_bytes()], bump=buyer_trade_state.to_account_info().data.borrow()[0])]
    pub(crate) buyer_trade_state: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), seller.key().as_ref(), auction_house.key().as_ref(), token_account.key().as_ref(), auction_house.treasury_mint.as_ref(), token_mint.key().as_ref(), &buyer_price.to_le_bytes(), &token_size.to_le_bytes()], bump=seller_trade_state.to_account_info().data.borrow()[0])]
    pub(crate) seller_trade_state: UncheckedAccount<'info>,
    #[account(mut, seeds=[PREFIX.as_bytes(), seller.key().as_ref(), auction_house.key().as_ref(), token_account.key().as_ref(), auction_house.treasury_mint.as_ref(), token_mint.key().as_ref(), &0u64.to_le_bytes(), &token_size.to_le_bytes()], bump=free_trade_state_bump)]
    pub(crate) free_trade_state: UncheckedAccount<'info>,
    pub(crate) token_program: Program<'info, Token>,
    pub(crate) system_program: Program<'info, System>,
    pub(crate) ata_program: Program<'info, AssociatedToken>,
    #[account(seeds=[PREFIX.as_bytes(), SIGNER.as_bytes()], bump=program_as_signer_bump)]
    pub(crate) program_as_signer: UncheckedAccount<'info>,
    pub(crate) rent: Sysvar<'info, Rent>,
}
