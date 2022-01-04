use anchor_lang::prelude::*;
use instructions::*;

mod utils;
mod errorcodes;
pub mod instructions;
pub mod office_house_structs;
mod constants;

use office_house_structs::WithdrawFromFee;


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
    //     instructions::withdraw_from_fee::handler(ctx, amount)
    // }
}

#[derive(Accounts)]
pub struct Initialize {}
