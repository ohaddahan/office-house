mod utils;
mod errorcodes;
mod instructions;
mod constants;
mod office_house_structs;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod office_house {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    // pub fn withdraw_from_fee<'info>(
    //     ctx: Context<'_, '_, '_, 'info, WithdrawFromFee<'info>>,
    //     amount: u64,
    // ) -> ProgramResult {
    //     withdraw_from_fee(ctx, amount)
    // }
}

#[derive(Accounts)]
pub struct Initialize {}
