use {
    crate::instruction::Instruction,
    solana_account_info::AccountInfo,
    solana_program_error::ProgramResult,
    solana_pubkey::Pubkey,
};

pub fn process(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = Instruction::unpack(input)?;
    match instruction {
        _ => todo!(),
    }
}
