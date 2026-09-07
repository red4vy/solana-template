use solana_program_error::ProgramError;

#[repr(u8)]
pub enum Instruction {
    // replace with your variants
    Todo,
}
impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use ProgramError::InvalidInstructionData;

        let (&_tag, _rest) = input.split_first().ok_or(InvalidInstructionData)?;
        todo!();
    }
}
