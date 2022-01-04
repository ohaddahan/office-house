use anchor_lang::prelude::*;

mod errorcodes;
mod instructions;
use instructions::*;
mod office_house_structs;
mod utils;
mod constants;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod office_house {

    use super::*;

    pub fn withdraw_from_fee<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawFromFee<'info>>,
        amount: u64,
    ) -> ProgramResult {
        instructions::withdraw_from_fee::withdraw_from_fee(ctx, amount)
    }

    pub fn withdraw_from_treasury<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawFromTreasury<'info>>,
        amount: u64,
    ) -> ProgramResult {
        instructions::withdraw_from_treasury(ctx, amount)
    }

    pub fn withdraw<'info>(
        ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
        escrow_payment_bump: u8,
        amount: u64,
    ) -> ProgramResult {
        instructions::withdraw(ctx, escrow_payment_bump, amount)
    }


    pub fn update_auction_house<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateAuctionHouse<'info>>,
        seller_fee_basis_points: Option<u16>,
        requires_sign_off: Option<bool>,
        can_change_sale_price: Option<bool>,
    ) -> ProgramResult {
        instructions::update_auction_house(ctx, seller_fee_basis_points, requires_sign_off, can_change_sale_price)
    }

    pub fn sell<'info>(
        ctx: Context<'_, '_, '_, 'info, Sell<'info>>,
        trade_state_bump: u8,
        _free_trade_state_bump: u8,
        _program_as_signer_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> ProgramResult {
        instructions::sell(ctx, trade_state_bump, _free_trade_state_bump , _program_as_signer_bump, buyer_price, token_size)
    }

    pub fn execute_sale<'info>(
        ctx: Context<'_, '_, '_, 'info, ExecuteSale<'info>>,
        escrow_payment_bump: u8,
        _free_trade_state_bump: u8,
        program_as_signer_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> ProgramResult {
        instructions::execute_sale(ctx, escrow_payment_bump, _free_trade_state_bump, program_as_signer_bump, buyer_price, token_size)
    }

    pub fn deposit<'info>(
        ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
        escrow_payment_bump: u8,
        amount: u64,
    ) -> ProgramResult {
        instructions::deposit(ctx, escrow_payment_bump, amount)
    }

    pub fn buy<'info>(
        ctx: Context<'_, '_, '_, 'info, Buy<'info>>,
        trade_state_bump: u8,
        escrow_payment_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> ProgramResult {
        instructions::buy(ctx, trade_state_bump, escrow_payment_bump, buyer_price, token_size)
    }

    pub fn create_auction_house<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateAuctionHouse<'info>>,
        bump: u8,
        fee_payer_bump: u8,
        treasury_bump: u8,
        seller_fee_basis_points: u16,
        requires_sign_off: bool,
        can_change_sale_price: bool,
    ) -> ProgramResult {
        instructions::create_auction_house(ctx, bump, fee_payer_bump, treasury_bump, seller_fee_basis_points, requires_sign_off, can_change_sale_price)
    }

    pub fn cancel<'info>(
        ctx: Context<'_, '_, '_, 'info, Cancel<'info>>,
        _buyer_price: u64,
        _token_size: u64,
    ) -> ProgramResult {
        instructions::cancel(ctx, _buyer_price, _token_size)
    }
}
