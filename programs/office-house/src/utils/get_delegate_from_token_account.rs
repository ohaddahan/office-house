use anchor_lang::prelude::*;
use arrayref::array_ref;

/// Cheap method to just grab delegate Pubkey from token account, instead of deserializing entire thing
pub fn get_delegate_from_token_account(
    token_account_info: &AccountInfo,
) -> Result<Option<Pubkey>, ProgramError> {
    // TokeAccount layout:   mint(32), owner(32), ...
    let data = token_account_info.try_borrow_data()?;
    let key_data = array_ref![data, 76, 32];
    let coption_data = u32::from_le_bytes(*array_ref![data, 72, 4]);
    if coption_data == 0 {
        Ok(None)
    } else {
        Ok(Some(Pubkey::new_from_array(*key_data)))
    }
}
