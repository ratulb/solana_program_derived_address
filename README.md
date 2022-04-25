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

On entry to the [account setup process](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L169), we retrieve the payer pubkey(i.e. pubkey from `~/.config/solana/id.json`), then look for the program id(pubkey from ./target/deploy/program-keypair.json). If the program has not been built - account set up would [fail](https://github.com/ratulb/solana_counter_program/blob/da583a9c8516a8cb69d0c32058f9a161e5a1280c/client/src/client.rs#L165) fast.

Next, we construct the counter account pubkey based on payer pubkey, [seed](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L21) and the program id(owner of the account) and make a rpc call to the chain to retrieve the account. Successful retrieval of the account results in [early exit](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L184) from this call because required counter account already exists and we have nothing to setup.

We proceed to setup the account if it does not already exist. We calculate the minimum balance that would be required for the counter account to stay rent exempt based on the how much space it would maintain in its [data](https://github.com/solana-labs/solana/blob/57ff7371b402d52d59dbd3555a181c415e8ed30c/sdk/src/account.rs#L27) field. This data field will hold the serialzed bytes of [Counter](https://github.com/ratulb/solana_counter_program/blob/6b26d824c2e566c13928f9f70200c41e0a2fb031/common/src/state.rs#L5-L7) struct. This struct has a `count` field of type [u64](https://doc.rust-lang.org/std/primitive.u64.html) - which is 8 bytes long. The count struct derives borsh [BorshSerialize](https://github.com/near/borsh-rs/blob/7325a0aab74049237b9f978e1e2c0fcf7bb799c2/borsh/src/ser/mod.rs#L43) and [BorshDeserialize](https://github.com/near/borsh-rs/blob/7325a0aab74049237b9f978e1e2c0fcf7bb799c2/borsh/src/de/mod.rs#L30) traits [here](https://github.com/ratulb/solana_counter_program/blob/6b26d824c2e566c13928f9f70200c41e0a2fb031/common/src/state.rs#L3) - rendering it capable of being serialized to and from a byte slice. We calculate size of a `Counter` struct [here](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L22).

We fetch minimum required lamports balance for the counter account [here](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L187-L193). This amount would be deducted from the payer's account when we execute the create account transaction later.

After this - we proceed to construct the system instruction for creating the counter account in this [section](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L195-L202). We pass the lamports amount, space and owner(program id) along with other relevant fields.

Next, we [query](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L205-L208) the latest blockhash from the solana network. This is a measure of how long ago the client has seen the network state and used by the network to accept/reject [transaction](https://docs.rs/solana-sdk/latest/solana_sdk/transaction/struct.Transaction.html).

We [query](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L213-L216) network again to find out the required fee for the transaction message - this is the amount for executing transaction on the network passsing the message and the blockhash retrieved in the previous step. 

We sum up the minimum rent exemption lamports and transaction cost([fee_for_message](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L218)) and do ourselves a lamports [airdrop](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L221-L223). Airdrop request would [not hit the network](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L134-L138) if we pass an environment variable named 'skip_airdrop' with value set to some non-empty value(for experimentation!) or the payer account has sufficient lamports to provide for the transaction cost and minimum rent exemption amount required for the counter account to stay afloat(aka rent free!).

At the end, we [send our account setup transaction across](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L226-L229) to the network and We get back a transaction [signature](https://docs.rs/solana-sdk/latest/solana_sdk/signature/struct.Signature.html)! We can make use of the signature to find out the transaction status, if we want. We are ignoring the returned signature here(Have not seen it fail & retry would muddy this learning exercise!).


### Check if the counter on-chain program has been deployed

Deployment [verification](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L255) starts by checking for the existence [program keypair](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L24) that must have been generated at the [program build phase](#build-the-on-chain-program). If [keypair](https://docs.rs/solana-sdk/latest/solana_sdk/signer/keypair/index.html) can not be found - the program exits with appropriate error message.
We try to retrieve the program account corresponding to the [pubkey](https://docs.rs/solana-sdk/latest/solana_sdk/pubkey/struct.Pubkey.html) of the program keypair - here the intent being two pronged - to verify that the program has been deployed to the chain and it, indeed, is executable.
Now here is a catch - we can load the program account and check for [executable](https://github.com/solana-labs/solana/blob/77182fcdda510154ed1194e0188ede80c64e7907/sdk/src/account.rs#L31) flag on it and decide whether to proceed further or not. But this alone is not sufficient - because programs owned by the [upgradable bpf loader](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L261) maybe closed(`solana program close program_id`) - and it will still report the program as being executable.

This is not the case with [bpf loader](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L263) - It does not allow closing a deployed program.

Programs owned by upgradable loader store their executable bits in a seprate account which can be seen below:
<p align="left">
  <a href="#">
    <img alt="Program" src="step1.png" width="500" height="300"/>
  </a>
</p>

We can query the program data account (underlined red in the image) and it will spit out a huge pile of hexadecimal numbers. When we close a program - it is this program data account that gets wiped out - but program account still says it is executable - which is not very helpful. That is why we try to retrieve the program data account where actual program byte codes are stored - in the case that program is owned by upgradeable bpf loader and deployed on-chain program may have been closed.

> **Note**: `solana program deploy program.so` - deployes to upgradable loader and `solana deploy program.so` - deploys to bpf loader. Programs owned by bpf loader are are not upgradeable and store progrm byte code in the program account itself.

### Send a counter Increament transaction to the on-chain program

[Here](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L319) we submit a transaction to our on-chain counter program to increament the counter value that is maintained in its owned account.

Usual steps like loading payer keypair, program id, querying for latest blockhash and fee for message etc happen in appropriate places - but one thing to note here is that we are packing an enum defined [here](https://github.com/ratulb/solana_counter_program/blob/421d7cfb80fab2a02b0982f03d2a47356e7eadfe/common/src/instruction.rs#L4) with the [instruction](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L329).
To invoke a solana on-chain program - we send a Transaction, which contains a message and the [message](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/message/versions/v0/mod.rs#L54) encaptulates one or more [instructions](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/instruction.rs#L324) within it. We see that `instruction` construct has a [data](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/instruction.rs#L333) field within it - which is a [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) of bytes. We can send any data specific to our program so long as the program knows how to deserialize and handle it - solana runtime is [agnostic](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/instruction.rs#L277) about the format of data that an instruction carries but it exposes useful APIs for constructing `instructions` from both [borsh](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/instruction.rs#L383) and [bincode](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/instruction.rs#L435) serializable types. [Borsh](https://borsh.io/) is preferred because of its stable specification. [Bincode](https://github.com/bincode-org/bincode) is mentioned as being computationally expensive and not to have had a published spec in solana documentations but now bincode encoding spec can be found [here](https://github.com/bincode-org/bincode/blob/trunk/docs/spec.md).

As said - solana runtime does not care what data we pack inside an instruction as long as our on-chain program is able to deserialize and decipher it. It is not mandatory to use borsh or bincode - we can,very well, invent our own serialization mechanism if we want and make use of [this](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/instruction.rs#L492) API to construct an instruction, embed it in a message, submit a transaction that carries the message to the network. During execution, solana runtime will faithfully make available the [packed data](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/instruction.rs#L333) in the instruction to the [program](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/instruction.rs#L327) that the instruction was created for - in the form of [byte array](https://github.com/ratulb/solana_counter_program/blob/dbbb8839b1e6940ab227065a654156b0484228cd/program/src/entrypoint.rs#L12).

In any case, we did not wnat to re-invent the wheel instead use borsh serialization and deserialization [here](https://github.com/ratulb/solana_counter_program/blob/dbbb8839b1e6940ab227065a654156b0484228cd/common/src/state.rs#L3) and [here](https://github.com/ratulb/solana_counter_program/blob/dbbb8839b1e6940ab227065a654156b0484228cd/common/src/instruction.rs#L3).
We pack our application specific custom data(which is an enum with just one variant) [here](https://github.com/ratulb/solana_counter_program/blob/dbbb8839b1e6940ab227065a654156b0484228cd/client/src/client.rs#L301) and solana runtime makes that data available to our program [here](https://github.com/ratulb/solana_counter_program/blob/dbbb8839b1e6940ab227065a654156b0484228cd/program/src/entrypoint.rs#L12) and we reconstruct our enum variant [here](https://github.com/ratulb/solana_counter_program/blob/dbbb8839b1e6940ab227065a654156b0484228cd/program/src/processor.rs#L20).
Its also mandatory that we pass along [accounts](https://github.com/solana-labs/solana/blob/5c7060eaeb384cdff4db9299ecbf52d446110859/sdk/program/src/instruction.rs#L330) that our program reads or modifies during its execution. Our program increaments the counter value in the counter account that it owns. Hence we [pass](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L330) that information in a [AccountMeta](https://github.com/solana-labs/solana/blob/5c7060eaeb384cdff4db9299ecbf52d446110859/sdk/program/src/instruction.rs#L533) struct marking that as writable. Passing accounts that an on-chain program touches during its execution lets solana runtime parallelize transactions leading to faster execution time. 

> **Note**: This [line](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L338) is commented out. Its clones the instruction and packs it twice inside the message. What will happen if we uncomment this line and comment out the above line? Check that out!

### Query the counter account

Each time we run our client program - it [increaments](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L319) the [count field inside the counter account](https://github.com/ratulb/solana_counter_program/blob/2768076d9c576230a320327c48665f270dbbb4a2/program/src/processor.rs#L24-L30) owned by our on-chain program.
We load the counter account [here](https://github.com/ratulb/solana_counter_program/blob/97d463aecc7d21b138b95cd53bdd3e2d951ba663/client/src/client.rs#L362-L377) - deserialize the data field of the account into Counter struct and print out the count fields value.

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

