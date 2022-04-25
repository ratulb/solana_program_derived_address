use common::InstructionData;
use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        hash::Hash,
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
        signature::Signer,
        signer::keypair::{read_keypair_file, Keypair},
        transaction::Transaction,
    },
    std::error::Error,
};

static PROGRAM_KEYPAIR_PATH: &str = "target/deploy/program-keypair.json";
static PDA_SEED: &str = "PROGRAM";

///We find a PDA with the seed 'PDA_SEED'("PROGRAM") and the program id of the deployed
///program. This will return the same program derived address(pubkey without a corresponding
///private key) each time we call this program. We fund(airdrop) the PDA with some SOL deposit
///so that we can transfer SOL out of the PDA account. But since the PDA has no private key
///associated with it - it can not authorize that transfer. Since PDA was derived based on
///the program id - it has exclusive rights over the PDA account - the program can act as
///signer on behalf of the PDA by invoking 'invoke_signed'.

fn main() -> Result<(), Box<dyn Error>> {
    //Get the program keypair - this must have been generated during 'cargo build-bpf' command
    let program_keypair = read_keypair(PROGRAM_KEYPAIR_PATH);
    let program_id = program_keypair.pubkey();
    print!("The program address: ");
    program_id.log();

    //Find a PDA based on seed as "PROGRAM" and the program id
    let (pda, bump_seed) = Pubkey::find_program_address(&[PDA_SEED.as_bytes()], &program_id);
    print!("The program derived address: ");
    pda.log();
    let client = get_rpc_client();
    //Fund the PDA with 2 SOLs
    airdrop(2 * 1000000000, &pda, &client);

    //Create a random recipient keypair. To this random keypair's address we want to transfer
    //Sol from the PDA. This transaction has to be signed by on-chain program
    let recipient = Keypair::new();
    let recipent_address = recipient.pubkey();
    print!("The recipient address: ");
    recipent_address.log();
    let balance = get_account_balance(&recipent_address, &client);
    println!("Account balance for recipient {}", balance);

    //Create the instruction that needs to be encaptulated as part of the transaction message
    //We are packing the seed used, bump_seed returned(as part find_program_address
    //method invocation used above) and lamports to be transferred to the rabdom recipient
    //as InstructionData struct and bytes from the InstructionData struct into 'data' field 
    //of the instruction
    //
    //Lets transfer 1 SOL out of 2 SOLs deposited to PDA account
    let lamports_to_transfer = 2 * 1000000000 - 1 * 1000000000;
    let data = InstructionData::new(PDA_SEED, bump_seed, lamports_to_transfer);

    //The default wallet keypair for paying transaction costs
    let payer = get_payer_keypair();
    //Populate the accounts that would be accessed by the on-chain program and set singer
    //and writable attributes as required
    //Note: AccountMeta::new(..) - generates a writable AccountMeta
    let accounts = vec![
        //Singer - true, writable -true - has to bear transaction costs
        AccountMeta::new(payer.pubkey(), true),
        //Not a signer - can not sign because pda has no private key that it can
        //sign with. The on-chain program would assume the singer role for the PDA
        AccountMeta::new(pda, false),
        //Not a signer, writable true
        AccountMeta::new(recipent_address, false),
        //We need to make 'system_program' also make available as AccountInfo in our
        //on-chain program
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
    ];

    let instruction = Instruction::new_with_bytes(program_id, &data.as_bytes(), accounts);
    //Get the latest blockhash
    let blockhash = get_lastest_blockhash(&client);
    //Embed the instruction inside a message
    let message = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    //Construct the transaction
    let transaction = Transaction::new(&[&payer], message, blockhash);
    let _signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Transaction failed");

    println!("Transfer transaction submitted");

    let balance = get_account_balance(&recipent_address, &client);
    println!("Recipient account balance post transfer {}", balance);

    Ok(())
}
//Read a keypair file from a given path
fn read_keypair(path: &str) -> Keypair {
    let keypair = read_keypair_file(path);
    match keypair {
        Ok(keypair) => keypair,
        Err(err) => panic!("Error reading keypair {}", err),
    }
}
//Get a rpc handle to a network defined by environment variable rpc_url or local network
fn get_rpc_client() -> RpcClient {
    let client = match std::env::var("rpc_url") {
        Ok(ref rpc_url) => RpcClient::new(rpc_url),
        Err(_) => RpcClient::new("http://localhost:8899"),
    };
    client
}

//Get the account balance for an address from the network
pub fn get_account_balance(address: &Pubkey, client: &RpcClient) -> u64 {
    client.get_balance(address).unwrap_or_else(|_| {
        println!("Could not get account balance for address {}", address);
        0
    })
}

//Request an airdrop(deposit free SOL) in an account
pub fn airdrop(amount: u64, address: &Pubkey, client: &RpcClient) {
    //Skip airdrop for experimentation by setting the skip_airdrop env variable
    if std::env::var("skip_airdrop").ok().is_some() {
        return;
    }
    let balance = get_account_balance(address, client);
    println!(
        "Current account balance for address {} is {}",
        address, balance
    );
    let sig = client
        .request_airdrop(address, amount)
        .expect(&format!("Airdrop request failed"));
    //Wait a while for airdrop transaction to confirm - might keep looping!
    while !client.confirm_transaction(&sig).unwrap_or(false) {}
    let balance = get_account_balance(address, client);
    println!(
        "Post funding account balance for address {} is {}",
        address, balance
    );
}

//Get latest blockhash from the network - its a measure of how long ago the client has
//observed the network state. The network decides to accept/reject transactions based
//on the provided blockash in a transaction
fn get_lastest_blockhash(client: &RpcClient) -> Hash {
    client
        .get_latest_blockhash()
        .expect("Failed to get lastest blockhash from the network!")
}

//Get the default keypair that pays for all the transaction cost
fn get_payer_keypair() -> Keypair {
    let keypair_file = match home::home_dir() {
        Some(mut homedir) => {
            homedir.push(".config/solana/id.json");
            homedir
                .into_os_string()
                .into_string()
                .expect("Error reading keypair file")
        }
        None => panic!("Error reading home directory!"),
    };
    read_keypair(&keypair_file)
}
