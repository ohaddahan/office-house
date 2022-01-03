use anchor_lang::prelude::*;
use crate::errorcodes::errors::Errors;

pub fn assert_keys_equal(key1: Pubkey, key2: Pubkey) -> ProgramResult {
    if key1 != key2 {
        Err(Errors::PublicKeyMismatch.into())
    } else {
        Ok(())
    }
}
