use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;

use crate::state::{
    utils::{load_ix_data, DataLen},
    Member, Multisig, MultisigConfig, Permission,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct UpdateThreshold {
    pub min_threshold: u64,
}

impl DataLen for UpdateThreshold {
    const LEN: usize = core::mem::size_of::<UpdateThreshold>();
}

pub fn process_update_threshold_instruction(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let [payer, creator, multisig, multisig_config, treasury, _remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    // Multisig PDA
    let seed = [(b"multisig_config"), multisig.key().as_slice()];
    let seeds = &seed[..];
    let (pda_config, multisig_config_bump) = pubkey::find_program_address(seeds, &crate::ID);
    assert_eq!(&pda_config, multisig_config.key());

    let update_threshold_data = load_ix_data::<UpdateThreshold>(data)?;

    // Load the MultisigConfig account to modify it
    let mut multisig_config_account =
        MultisigConfig::from_account_info(&multisig_config_account_info)?;

    if multisig_config_account.min_threshold == update_threshold_data.min_threshold {
        return Err(ProgramError::InvalidInstructionData); // Or a more specific custom error
    }

    // Update the min_threshold
    multisig_config_account.min_threshold = update_threshold_data.min_threshold;

    log!(
        "Updated min_threshold to {}",
        multisig_config_account.min_threshold
    );

    Ok(())
}
