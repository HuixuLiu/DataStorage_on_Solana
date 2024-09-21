use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum IntroInstruction {
    InitUserInput { name: String },
}
