use anchor_lang::prelude::*;
use arrayref::array_ref;

/// Cheap method to just grab mint Pubkey from token account, instead of deserializing entire thing
pub fn get_mint_from_token_account(
    token_account_info: &AccountInfo,
) -> Result<Pubkey, ProgramError> {
    // TokeAccount layout:   mint(32), owner(32), ...
    let data = token_account_info.try_borrow_data()?;
    let mint_data = array_ref![data, 0, 32];
    Ok(Pubkey::new_from_array(*mint_data))
}
