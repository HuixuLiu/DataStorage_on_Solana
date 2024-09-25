use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum IntroInstruction {
    InitUserInput { address_info: String },
}
