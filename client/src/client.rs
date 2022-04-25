use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
        signature::Signer,
        signer::keypair::{read_keypair_file, Keypair},
        transaction::Transaction,
    },
    std::error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let program_keypair =
        read_keypair_file("../check_invoke_signed/target/deploy/check_invoke_signed-keypair.json")?;
    let pda =
        Pubkey::find_program_address(&["invoke_signed".as_bytes()], &program_keypair.pubkey());
    let bump_seed = pda.1;
    let pda = pda.0;
    println!("The pda...bump seed {:?}", bump_seed);
    pda.log();
    println!();
    let client = RpcClient::new("http://localhost:8899");
    let version = client
        .get_version()
        .expect("Error getting node solana version");
    println!("Cluster node solana version {:?}", version);

    let program_id = program_keypair.pubkey();
    println!("The program id...");
    program_id.log();
    println!();
    let recipient = Keypair::new();
    println!("The recipient id...");

    recipient.pubkey().log();
    println!();
    let payer = read_keypair_file("/home/rakheeburagohain/.config/solana/id.json").unwrap();
    let mut seeds = "invoke_signed".as_bytes().to_vec();
    seeds.push(bump_seed);
    println!("The seeds {:?}", seeds);
    let instruction = Instruction::new_with_bytes(
        program_id,
        &seeds,
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new(recipient.pubkey(), false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
    );
    let blockhash = client.get_latest_blockhash().unwrap();
    let message = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let transaction = Transaction::new(&[&payer], message, blockhash);
    let _signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transfer transaction sent");

    Ok(())
}
