
pub const OFFICE_HOUSE_SIZE: usize = 8 + //key
    32 + //fee payer
    32 + //treasury
    32 + //treasury_withdrawal_destination
    32 + //fee withdrawal destination
    32 + //treasury mint
    32 + //authority
    32 + // creator
    1 + // bump
    1 + // treasury_bump
    1 + // fee_payer_bump
    2 + // seller fee basis points
    1 + // requires sign off
    1 + // can change sale price
    220; //padding
