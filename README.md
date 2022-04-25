# Solana program derived address(PDA) in action

This project demonstrates how to make use of PDAs in solana via an on-chain program and a rust client program that invokes the program.

PDA(more [here](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses) and [here](https://github.com/solana-labs/solana/blob/70d57245b4ffaeec35a931db25282c5c35fe0be3/sdk/program/src/pubkey.rs#L456))s are a cool feature of solana - they add a much needed secure storage mechansim for immutable on-chain programs. PDAs allow programs(from which the PDAs are derived from) to act as singers - preventing un-authorized modification from any other actors.

We find an [program derived address](https://github.com/ratulb/solana_program_derived_address/blob/34e7e045173b8687b83f6596c2e059134b307176/client/src/main.rs#L35) and its corresponding bump seed from the deployed program's id and a seed("PROGRAM"), [fund](https://github.com/ratulb/solana_program_derived_address/blob/34e7e045173b8687b83f6596c2e059134b307176/client/src/main.rs#L40) the PDA with some SOL. We transfer SOL from the PDA to a randomly generated [recipient](https://github.com/ratulb/solana_program_derived_address/blob/34e7e045173b8687b83f6596c2e059134b307176/client/src/main.rs#L45). PDAs do not have private keys and they can not authorize(sign) the transfer. Hence we [submit](https://github.com/ratulb/solana_program_derived_address/blob/34e7e045173b8687b83f6596c2e059134b307176/client/src/main.rs#L86) the transaction to our on-chain program, which does a cross program [invocation(CPI)](https://github.com/ratulb/solana_program_derived_address/blob/85953ae9b229ac1613eba99a51aa18ab181228a5/program/src/lib.rs#L42) to the system program. System program does the actual transfer while our on-chain program acts on the behalf of the PDA assuming the role of PDA's signer.

The project comprises of:

* An [on-chain program](https://github.com/ratulb/solana_program_derived_address/blob/999604aead61e7a2c05ba42e027693e1fc4b2336/program/src/lib.rs#L14) that receives a client transaction and does a CPI on the clients behalf
* A rust [client](https://github.com/ratulb/solana_program_derived_address/blob/999604aead61e7a2c05ba42e027693e1fc4b2336/client/src/main.rs#L27) that that submits transactiion to the on-chain to do a SOL transfer from a PDA that is derived from the on-chain program's id.
* A shared crate called [common](https://github.com/ratulb/solana_program_derived_address/blob/main/common/src/lib.rs) which exposes a struct called [InstructionData](https://github.com/ratulb/solana_program_derived_address/blob/999604aead61e7a2c05ba42e027693e1fc4b2336/common/src/lib.rs#L4) that carries instruction data from the client to deployed program.
## Table of Contents
- [On-chain program and client](#on-chain_program_and_client)
  - [Table of Contents](#table-of-contents)
  - [Quick Start](#quick-start)
    - [Configure CLI](#configure-cli)
    - [Start local Solana cluster](#start-local-solana-cluster)
    - [Build the on-chain program](#build-the-on-chain-program)
    - [Deploy the on-chain program locally](#deploy-the-on-chain-program-locally)
    - [Deploy to devnet](#deploy-to-devnet)
    - [Deploy to testnet](#deploy-to-testnet)
    - [Run the rust client](#run-the-rust-client)
    - [Expected output](#expected-output)
      - [Not seeing the expected output?](#not-seeing-the-expected-output)
    - [Project structure](#project-structure)

  - [More about the client](#more-about-the-client)
    
  - [More about the on-chain program](#more-about-the-on-chain-program)
    
## Quick Start

The following dependencies are required to build and run this example:

- Install Rust v1.60.0 or later from https://rustup.rs/
- Install Solana v1.10.9 or later from
  https://docs.solana.com/cli/install-solana-cli-tools

If this is your first time using Rust, these [Installation
Notes](README-installation-notes.md) might be helpful.

### Configure CLI

> If you're on Windows, it is recommended to use [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) to run these commands

1. Set CLI config url to localhost cluster

```bash
solana config set --url localhost
or 
solana config set -ul

```

2. Create CLI Keypair

If this is your first time using the Solana CLI, you will need to generate a new keypair:

```bash
solana-keygen new
```

### Start local Solana cluster

This example connects to a local Solana cluster by default.

Start a local Solana cluster:
```bash
solana-test-validator
```
If you want start with a clean slate after couple of trials, you can do:

```bash
solana-test-validator --reset
```

> **Note**: You may need to do some [system tuning](https://docs.solana.com/running-validator/validator-start#system-tuning) (and restart your computer) to get the validator to run

On-chain deployed program's logs can be viewed by launcing a separate terminal and firing the following command:
```bash
solana logs
```
> **Note**: For logging messages inside the on-chain program, we should use the `msg!` [macro](https://docs.solana.com/developing/on-chain-programs/developing-rust#logging).

### Build the on-chain program

Go inside the 'solana_program_derived_address' directory if not already done:

```bash
cd solana_program_derived_address

cargo build-bpf
```
### Deploy the on-chain program locally

```bash
solana program deploy target/deploy/program.so
```

### Run the rust client

```bash
cargo run
```

### Expected output

Values will differ!

```bash
The program address: 9rmCVEicv13Yvvsf9jxfFDN32qLmuASgPMtyDq44nGjZ
The program derived address: DuViEkGrA6FtsdX7TX85A94Q1VdVYMSgHZ6fYDHo2Vt2
Current account balance for address DuViEkGrA6FtsdX7TX85A94Q1VdVYMSgHZ6fYDHo2Vt2 is 0
Post funding account balance for address DuViEkGrA6FtsdX7TX85A94Q1VdVYMSgHZ6fYDHo2Vt2 is 20000
00000
The recipient address: 28qBD3umyZsx7rejtPKxcWpdtoiY4ieTkCpo4XrWMiqL
Account balance for recipient 0
Transfer transaction submitted
Recipient account balance post transfer 1000000000
```

#### Not seeing the expected output?

- Ensure you've [started the local cluster](#start-local-solana-cluster),
  [build the on-chain program](#build-the-on-chain-program) and [deployed the program to the cluster](#deploy-the-on-chain-program-locally).
  
  

### Deploy to devnet

```bash
solana config set --url d
solana program deploy target/deploy/program.so
```
#### Or
### Deploy to testnet

```bash
solana config set -ut

solana program deploy target/deploy/program.so
```
> **Note**: If deployed to devnet or testnet, we need to pass the `rpc_url` env variable to the client program accordinly!

> **Note**: You may not have required SOL balance to deploy and run transactions in devnet or testnet. To request SOL into your account do an airdrop:

#### Check account sol balance:
```bash
solana balance
```
#### Request sol airdrop:
```bash
solana airdrop 10
```

#### Run the client:
```bash
cargo run
```

### Project structure

The following image shows the project layout. We are making use of cargo [workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
- [program](https://github.com/ratulb/solana_program_derived_address/tree/main/program) - this is the on-chain program that does a cross program invocation
- [client](https://github.com/ratulb/solana_program_derived_address/tree/main/client) - this is the rust client program that invokes the on-chain program to do a SOL transfer from PDA to a random recipient account.
- [common](https://github.com/ratulb/solana_program_derived_address/tree/main/common) - this crate contains the `InstructionData` serde serializable/deserializable struct used by both client and the on-chain program.

For experimentation, tweaking files under the program folder would require [rebuild](#build-the-on-chain-program) and [redeployment](#deploy-the-on-chain-program-locally).

Now when you rerun `cargo run`, you should see the results of your changes.
## More about the client

The client is a rust cli [program](https://github.com/ratulb/solana_program_derived_address/blob/main/client/src/main.rs) with a main function.

#### Main function

The [main function](https://github.com/ratulb/solana_program_derived_address/blob/481f131e17531489e51980785b15c15424496749/client/src/main.rs#L27) does following things:


#### Creates program derived address based on the on-chain program id and a seed
Relevant code can be found [here](https://github.com/ratulb/solana_program_derived_address/blob/481f131e17531489e51980785b15c15424496749/client/src/main.rs#L35)

#### Does a fund airdrop to the PDA

Relevant [code](https://github.com/ratulb/solana_program_derived_address/blob/481f131e17531489e51980785b15c15424496749/client/src/main.rs#L40).

#### Creates a random recipient address

Relevant [code](https://github.com/ratulb/solana_program_derived_address/blob/481f131e17531489e51980785b15c15424496749/client/src/main.rs#L43).

#### Populates the InstructionMeta struct that would be sent accross to the on-chain program

Relevant [code](https://github.com/ratulb/solana_program_derived_address/blob/24cd37bc14d1f01f78a82f4ee07de23c671513da/client/src/main.rs#L53)

#### Prepares the AccountMeta structs that would be read/writen by the on-chain program and passed along to the system program as part of CPI invocation.

Relevant [code](https://github.com/ratulb/solana_program_derived_address/blob/24cd37bc14d1f01f78a82f4ee07de23c671513da/client/src/main.rs#L60).

#### Prepares Instruction, message and submits the transaction

Relevant [code](https://github.com/ratulb/solana_program_derived_address/blob/24cd37bc14d1f01f78a82f4ee07de23c671513da/client/src/main.rs#L78)

#### Prints the random recipient's account balance after the transfer transaction was processed via the cross program invocation call.

Relevant [code](https://github.com/ratulb/solana_program_derived_address/blob/24cd37bc14d1f01f78a82f4ee07de23c671513da/client/src/main.rs#L91)


## More about the on-chain program

The on-chain program validates the passed in AccountInfo(s) for attributes like signer and writable in this [section](https://github.com/ratulb/solana_program_derived_address/blob/ba300ce996d63afb6d6a6ff17a419c0a2e460270/program/src/lib.rs#L19-L29).

[Here](https://github.com/ratulb/solana_program_derived_address/blob/ba300ce996d63afb6d6a6ff17a419c0a2e460270/program/src/lib.rs#L31), it reconstructs the `InstructionData` from the passed in [instruction_data](https://github.com/ratulb/solana_program_derived_address/blob/ba300ce996d63afb6d6a6ff17a419c0a2e460270/program/src/lib.rs#L17) byte array.

Finally, [here](https://github.com/ratulb/solana_program_derived_address/blob/ba300ce996d63afb6d6a6ff17a419c0a2e460270/program/src/lib.rs#L41-L42) the on chain program does a CPI call to execute the SOL transfer.
