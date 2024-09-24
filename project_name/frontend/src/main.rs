use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, read_keypair_file, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction,
    instruction::{Instruction},
    message::Message,
};
use solana_program::system_program;
use std::str::FromStr;


pub mod instruction; 
use instruction::IntroInstruction;
use borsh::BorshSerialize;

fn main() {

    let rpc_url = "https://api.testnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());


    // let payer = Keypair::new();
    //let payer = load_keypair_from_file("~/.config/solana/id.json");

    let path = "/home/huixu/project_name/name_frontend/src/id.json";
    
    let payer: Keypair = read_keypair_file(path).expect("Failed to read keypair file");

    let program_id = Pubkey::from_str("Emto22Ug8ZxvEdQyFYZ4qoxtEfqvE7Q8u4hQ53kiov2q").unwrap();


    let student_name = String::from("Diego Maradona");


    let instruction_data = IntroInstruction::InitUserInput {
        name: student_name.clone(),
    }.try_to_vec().unwrap(); 

    let accounts = vec![];

    let instruction = Instruction {
        program_id,
        accounts,
        data: instruction_data,
    };


    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let transaction = Transaction::new(&[&payer], message, recent_blockhash);

    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction sent. Signature: {}", signature);


    println!("Stored student name: {}", student_name);
}
