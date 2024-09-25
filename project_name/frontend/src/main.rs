// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{
//     signature::{Keypair, read_keypair_file, Signer},
//     transaction::Transaction,
//     pubkey::Pubkey,
//     system_instruction,
//     instruction::{Instruction},
//     message::Message,
// };
// use solana_program::system_program;
// use std::str::FromStr;

// pub mod instruction; 
// use instruction::IntroInstruction;
// use borsh::BorshSerialize;

// fn main() {

//     let rpc_url = "https://api.testnet.solana.com";
//     let client = RpcClient::new(rpc_url.to_string());


//     // let payer = Keypair::new();
//     //let payer = load_keypair_from_file("~/.config/solana/id.json");

//     let path = "/home/huixu/Git_Repo/DataStorage_on_Solana/project_name/frontend/src/id.json";
    
//     let payer: Keypair = read_keypair_file(path).expect("Failed to read keypair file");

//     let program_id = Pubkey::from_str("89ZBsUYNTxZuwjmjjpJdTg9ckqUmcxRKThtmv3zAtLGHGH").unwrap();


//     let student_name = String::from("Helene-mayer-Ringe 10");
//     let 


//     let instruction_data = IntroInstruction::InitUserInput {
//         name: student_name.clone(),
//     }.try_to_vec().unwrap(); 

//     let accounts = vec![];

//     let instruction = Instruction {
//         program_id,
//         accounts,
//         data: instruction_data,
//     };


//     let recent_blockhash = client.get_latest_blockhash().unwrap();
//     let message = Message::new(&[instruction], Some(&payer.pubkey()));
//     let transaction = Transaction::new(&[&payer], message, recent_blockhash);

//     let signature = client.send_and_confirm_transaction(&transaction).unwrap();
//     println!("Transaction sent. Signature: {}", signature);


//     println!("Stored student name: {}", student_name);
// }

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair,read_keypair_file, Signer},
    pubkey::Pubkey,
    system_instruction, transaction::Transaction,
    system_program,
    instruction::Instruction,
    commitment_config::CommitmentConfig,
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AddressInfo {
    name: String,
    house_number: u8,
    street: String,
    city: String,
}

impl AddressInfo {
    fn new(name: String, house_number: u8, street: String, city: String) -> Self {
        AddressInfo {
            name,
            house_number,
            street,
            city,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let rpc_url = "https://api.devnet.solana.com"; // Using testnet endpoint
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Create keypairs for payer and new account
 
    let path = "/home/huixu/Git_Repo/DataStorage_on_Solana/project_name/frontend/src/id.json";
    
    let payer: Keypair = read_keypair_file(path).expect("Failed to read keypair file");
    let new_account = Keypair::new();

    // Program ID (replace with actual program ID deployed on-chain)
    let program_id = Pubkey::from_str("89ZBsUYNTxZuwjmjjpJdTg9ckqUmcxRKThtmv3zAtLGH").unwrap();

    // Address Info that will be serialized and stored in the new account
    let address_info = AddressInfo::new(
        "Diego Wagner".to_string(),
        10,
        "Helene Mayer Ringe".to_string(),
        "Munich".to_string(),
    );

    // Serialize the data using Borsh
    let address_info_data = address_info.try_to_vec().unwrap();

    // Create the transaction to create a new account and store the address info
    let lamports = client.get_minimum_balance_for_rent_exemption(address_info_data.len())?;
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        lamports,
        address_info_data.len() as u64,
        &program_id,
    );

    let store_data_ix = Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(new_account.pubkey(), false),
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
        ],
        data: address_info_data,
    };

    let recent_blockhash = client.get_latest_blockhash()?;

    let mut transaction = Transaction::new_with_payer(
        &[create_account_ix, store_data_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &new_account], recent_blockhash);

    // Send the transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction successful, signature: {}", signature);

    // Reading the data from the account
    let account_data = client.get_account_data(&new_account.pubkey())?;
    let stored_address_info = AddressInfo::try_from_slice(&account_data)?;
    println!("Stored Address Info: {:?}", stored_address_info);

    Ok(())
}
