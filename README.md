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
    - [Main function](#main-function)
    - [Establish a connection to the cluster](#instantiates-the-client-that-wraps-up-an-underlying-rpcclient)
    - [Setup an account to store counter program state](#setup-an-account-to-store-counter-program-state)
    - [Check if the counter on-chain program has been deployed](#check-if-the-counter-on-chain-program-has-been-deployed)
    - [Send a counter Increament transaction to the on-chain program](#send-a-counter-increament-transaction-to-the-on-chain-program)
    - [Query the counter account](#query-the-counter-account)
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

> **Note**: You may not have required SOL balance to deploy and run transactions in devnet or testnet. To request SOL into your account do an airdrop:

#### Check account sol balance:
```bash
solana balance
```
#### Request sol airdrop:
```bash
solana airdrop 1
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

#### Does a fund dposit(airdrop) to the PDA

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

To write an on-chain solana program - primarily we need to follow these steps:
- Provide a function whose type signature matches [this](https://github.com/solana-labs/solana/blob/f7d557d5ae5d2ebfb70c2eaefa7dd1e2068b748c/sdk/program/src/entrypoint.rs#L25-L26). Here `program_id` in the function signature is, of course, program pubkey. We can change this program id to some other id of our liking(Using `solana-keygen grind` to generate a vanity keypair and passing that as `--program-id` during deployment). The second parameter `accounts` - is the consolidated shared list of all the accounts that all instructions(embedded inside the message that a transaction contains) read and/or write to. They appear in the order that [AccountMeta](https://docs.rs/solana-program/latest/solana_program/instruction/struct.AccountMeta.html) structs are added to an instruction and the order that instructions are added to the containing message. All accounts(the type [AccountInfo](https://docs.rs/solana-program/latest/solana_program/account_info/struct.AccountInfo.html) that appears in the function signature - is a runtime construct - it does not stay physically in disc) should all have their writable/readable/signer attributes set. In our case, while we are constructing the counter account `AccountMeta` [here](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L330) - we are saying that its not a singer but writable because the program [writes](https://github.com/ratulb/solana_counter_program/blob/be92a744f0b083b91dbd9ff72fd41473f343f5c8/program/src/processor.rs#L28) to it while increamenting the counter value(`AccountMeta::new` - creates a writable account).
- Decorate the implementation with the [entrypoint macro](https://github.com/solana-labs/solana/blob/f7d557d5ae5d2ebfb70c2eaefa7dd1e2068b748c/sdk/program/src/entrypoint.rs#L116). As we can see - net effect of [invoking this macro](https://github.com/ratulb/solana_counter_program/blob/dbbb8839b1e6940ab227065a654156b0484228cd/program/src/entrypoint.rs#L7) is that our implemention function get embedded inside an external `c` function called `entrypoint`. Also, this external `c` function has got `no_mangle` annotation defined - which means compiler will keep its name as it is.
- Define `no-entrypoint` feature - During on-chain program development we might depend on other crates for many useful APIs that they might provide. But they might have their own entrypoints as we do. So there is the issue of entrypoint collisions. Since there can not be multiple entrypoints at runtime - we need to take care to exclude or include entrypoint sections as needed during the compilation phase. Wich is why we define the entrypoint feature [here](https://github.com/ratulb/solana_counter_program/blob/dabb68466c3401e3bfffd5de44ee5f8ed936c19a/program/Cargo.toml#L12). If, let say, someone is developing there own on-chain program and wants use some API from our crate - but wants to exclude our entrypoint - they would add a section `program = { version = "0.1.0", path = "../program",features = [ "no-entrypoint" ] }` to their Cargo.toml. Read more [here](https://docs.solana.com/developing/on-chain-programs/developing-rust#project-layout).
- [Build the on-chain program](#build-the-on-chain-program)- During the build process the program is compiled to [Berkeley Packet Filter(BPF)](https://en.wikipedia.org/wiki/Berkeley_Packet_Filter) bytecode and stored as an
Executable and Linkable Format [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) shared
object [locally](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L23). A program [keypair](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L24) is also generated - this generated keypair's pubkey becomes the default program_id.
- [Deploy the program to the network](#deploy-the-on-chain-program-locally) - Solana CLI breaks up compiled program byte code into smaller chunks(due to restricted transaction size) and sends the chunks to an intermediate on-chain buffer account in a series of transactions. Once transmission is complete and verified, a final transaction instruction moves the intermediate buffered content to program's data account. This completes a new deployment or a program upgrade. As usual, transaction costs are deducted from payer's account. See [this excellent post](https://jstarry.notion.site/Program-deploys-29780c48794c47308d5f138074dd9838) for more info.


> **End note**: If we start the validator in a clean state(`solana-test-validator --reset`) and run - **for _ in {0..99}; do cargo run; done** - from two terminals - we must observe counter value as **200**. But we don't get! What gives?

Hint: Check all the entries in ~/.config/solana/cli/config.yml

> **End note**: If we use `solana deploy program.so[path to .so]` to deploy our program - then the deployed program is owned by the bpf loader. We get a randomly generated program id. If that is the case - while running the client program - we need to pass an environment variable named `program_id` with value set generated id. Refer [here](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L106-L109).

