use borsh::{BorshDeserialize, BorshSerialize}; 
use solana_program::{program_error::ProgramError};

pub enum IntroInstruction {
    InitUserInput {
        name: String,
    }
}

#[derive(BorshDeserialize, Debug)]
struct StudentIntroPayload {
    name: String,
}

impl IntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        let payload = StudentIntroPayload::try_from_slice(rest).unwrap();

        Ok(match variant {
            0 => Self::InitUserInput {
                name: payload.name,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}